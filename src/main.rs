use std::fmt;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use sysinfo::System;

pub mod color;
pub mod component;

use crate::color::Color;
use component::*;

pub const HEIGHT: f32 = 24.;

pub static SYS: LazyLock<Mutex<System>> = LazyLock::new(Default::default);

pub const FG: Color = Color(0x888888);
pub const BG: Color = Color(0x000000);

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
    let interval = Duration::from_secs(2);
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

    let parts = middle.chain(right);

    loop {
        let start = Instant::now();
        println!("{}", parts);
        let elapsed = start.elapsed();

        if cfg!(feature = "timing") {
            eprintln!("{:?}", elapsed);
        }

        std::thread::sleep(interval - elapsed);
    }
}
