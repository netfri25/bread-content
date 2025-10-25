use std::borrow::Borrow as _;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write as _;
use std::os::unix::prelude::AsRawFd as _;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use mio::Interest;
use mio::unix::SourceFd;
use sysinfo::System;
use wayland_client::backend::ObjectId;
use wayland_client::protocol::wl_registry;
use wayland_client::{Connection, Dispatch, event_created_child};
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_handle_v1::{
    self, ZwlrForeignToplevelHandleV1,
};
use wayland_protocols_wlr::foreign_toplevel::v1::client::zwlr_foreign_toplevel_manager_v1::{
    self, ZwlrForeignToplevelManagerV1,
};

pub mod color;
pub mod component;

use crate::color::Color;
use component::*;

pub const FG: Color = Color(0x888888);
pub const BG: Color = Color(0x000000);
pub const HEIGHT: f32 = 24.;

const INTERVAL: Duration = Duration::from_secs(2);
const WAYLAND_TOKEN: mio::Token = mio::Token(1);
const TITLE_LIMIT: usize = 60;

pub static SYS: LazyLock<Mutex<System>> = LazyLock::new(Default::default);

fn label(text: &str) -> impl fmt::Display {
    Fg(Color::YELLOW).chain(text)
}

const fn reset_fg() -> impl fmt::Display {
    Fg(FG)
}

const fn reset_bg() -> impl fmt::Display {
    Bg(BG)
}

fn main() {
    // the structure of the bar, excluding the focused window name
    let right = AlignRight
        .chain(Cpu)
        .chain(reset_fg())
        .chain(reset_bg())
        .chain("  ")
        .chain(Temperature)
        .chain(reset_fg())
        .chain("  ")
        .chain(label("RAM ").chain(reset_fg()).chain(Memory))
        .chain("  ")
        .chain(label("WIFI ").chain(reset_fg()).chain(Wifi))
        .chain("  ")
        .chain(label("BAT ").chain(reset_fg().chain(Battery)));
    let middle = AlignCenter.chain(reset_fg()).chain(reset_bg()).chain(Time);
    let bar = middle.chain(right);

    // connect to wayland
    let conn = Connection::connect_to_env().unwrap();
    let display = conn.display();

    // create a new event queue for wayland requests/events
    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    // create a get registry request on the queue
    display.get_registry(&qhandle, ());

    // the structure that contains all of the current windows and the focused window
    let mut windows = Windows::default();

    // some dispatches to make sure the Windows gathered everything

    // registry
    event_queue.blocking_dispatch(&mut windows).unwrap();

    // foreign toplevel
    event_queue.blocking_dispatch(&mut windows).unwrap();

    // used for efficiently polling from the wayland socket
    let mut poll = mio::Poll::new().expect("unable to create Poll instance");
    let mut events = mio::Events::with_capacity(1);

    // some variables to make sure the bar is rendered every INTERVAL time, except for the title
    // which updates instantly
    let mut output = String::new();
    let mut last_update = Instant::now();
    let mut duration_left = Duration::ZERO;
    let mut end_of_info = 0;

    // add the wayland socket to the polling system
    {
        let read_guard = event_queue.prepare_read().unwrap();
        let wayland_fd = read_guard.connection_fd().as_raw_fd();
        let mut wayland_source = SourceFd(&wayland_fd);
        poll.registry()
            .register(&mut wayland_source, WAYLAND_TOKEN, Interest::READABLE)
            .unwrap();
    }


    loop {
        // dispatch pending messages on queue
        event_queue.flush().unwrap();
        event_queue.dispatch_pending(&mut windows).unwrap();

        // start the read guard
        let read_guard = event_queue.prepare_read().unwrap();

        // wait for either an event or a timeout
        events.clear();
        poll.poll(&mut events, Some(duration_left)).ok();

        // if the interval has already passed, update content
        if last_update.elapsed() > duration_left {
            output.clear();
            write!(output, "{}", bar).unwrap();
            end_of_info = output.len();
            last_update = Instant::now();
            duration_left = INTERVAL;
        }

        // if there was some event, then it's probably from wayland
        // dispatch the pending messages
        if !events.is_empty() && read_guard.read().is_ok() {
            event_queue.dispatch_pending(&mut windows).unwrap();
            event_queue.roundtrip(&mut windows).unwrap();
        }

        // remove the old focused window
        output.drain(end_of_info..);

        // write the new focused window, if exists
        if let Some((app_id, title)) = windows.current_info() {
            let data = AlignLeft.chain(label(app_id)).chain(reset_fg()).chain(": ");
            write!(output, "{}", data).unwrap();

            if app_id.len() + title.len() > TITLE_LIMIT {
                let mut limit = TITLE_LIMIT - app_id.len();
                while !title.is_char_boundary(limit) {
                    limit -= 1;
                }

                output.push_str(&title[..limit]);
                output.push_str("...");
            } else {
                output.push_str(title);
            }
        }

        // write out everything
        println!("{}", output);
    }
}

#[derive(Default, Debug)]
pub struct WindowInfo {
    pub app_id: Option<Box<str>>,
    pub title: Option<Box<str>>,
}

#[derive(Default, Debug)]
pub struct Windows {
    pub current: Option<ObjectId>,
    pub windows: HashMap<ZwlrForeignToplevelHandleV1, WindowInfo>,
}

impl Windows {
    pub fn current_info(&self) -> Option<(&str, &str)> {
        self.current
            .as_ref()
            .and_then(|current| self.windows.get(current))
            .and_then(|info| {
                let app_id = info.app_id.as_deref()?;
                let title = info.title.as_deref()?;
                Some((app_id, title))
            })
    }
}

impl Dispatch<wl_registry::WlRegistry, ()> for Windows {
    fn event(
        _: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
            && interface == "zwlr_foreign_toplevel_manager_v1"
        {
            proxy.bind::<ZwlrForeignToplevelManagerV1, _, _>(name, version, qhandle, ());
        }
    }
}

impl Dispatch<ZwlrForeignToplevelManagerV1, ()> for Windows {
    fn event(
        state: &mut Self,
        _: &ZwlrForeignToplevelManagerV1,
        event: <ZwlrForeignToplevelManagerV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let zwlr_foreign_toplevel_manager_v1::Event::Toplevel { toplevel } = event {
            state.windows.insert(toplevel, Default::default());
        }
    }

    event_created_child!(Windows, ZwlrForeignToplevelManagerV1, [
        zwlr_foreign_toplevel_manager_v1::EVT_TOPLEVEL_OPCODE => (ZwlrForeignToplevelHandleV1, ())
    ]);
}

impl Dispatch<ZwlrForeignToplevelHandleV1, ()> for Windows {
    fn event(
        windows: &mut Self,
        proxy: &ZwlrForeignToplevelHandleV1,
        event: <ZwlrForeignToplevelHandleV1 as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        match event {
            zwlr_foreign_toplevel_handle_v1::Event::Title { title } => {
                let info = windows.windows.get_mut(proxy).unwrap();
                info.title = Some(title.into_boxed_str());
            }

            zwlr_foreign_toplevel_handle_v1::Event::AppId { app_id } => {
                let info = windows.windows.get_mut(proxy).unwrap();
                info.app_id = Some(app_id.into_boxed_str());
            }

            zwlr_foreign_toplevel_handle_v1::Event::State { state } => {
                let activated_state = zwlr_foreign_toplevel_handle_v1::State::Activated as u8;
                if state.contains(&activated_state) {
                    let id: &ObjectId = proxy.borrow();
                    windows.current = Some(id.clone());
                } else if Some(proxy.borrow()) == windows.current.as_ref() {
                    windows.current = None;
                }
            }

            zwlr_foreign_toplevel_handle_v1::Event::Closed => {
                let id: &ObjectId = proxy.borrow();
                windows.windows.remove(id);
                if windows.current.as_ref() == Some(id) {
                    windows.current = None;
                }
            }

            _ => {}
        }
    }
}
