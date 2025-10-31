use std::str::FromStr;
use std::{fmt, fs, io};

pub mod battery;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod temperature;
pub mod time;
pub mod wifi;

use derive_more::Display;

pub use battery::*;
pub use cpu::*;
pub use gpu::*;
pub use memory::*;
pub use temperature::*;
pub use time::*;
pub use wifi::*;

use crate::color::Color;
use crate::HEIGHT;

pub trait DisplayExt: fmt::Display {
    fn chain<T: fmt::Display>(self, other: T) -> Chain<Self, T>
    where
        Self: Sized,
    {
        Chain { a: self, b: other }
    }
}

impl<T: fmt::Display> DisplayExt for T {}

#[derive(Display)]
#[display("{a}{b}")]
pub struct Chain<A, B> {
    a: A,
    b: B,
}

#[derive(Display)]
#[display("%{{F:{_0}}}")]
pub struct Fg(pub Color);

#[derive(Display)]
#[display("%{{B:{_0}}}")]
pub struct Bg(pub Color);

#[derive(Display)]
#[display("%{{l}}")]
pub struct AlignLeft;

#[derive(Display)]
#[display("%{{c}}")]
pub struct AlignCenter;

#[derive(Display)]
#[display("%{{r}}")]
pub struct AlignRight;

#[derive(Display)]
#[display("%{{R:{w}x{h}}}")]
pub struct Ramp {
    pub w: u32,
    pub h: u32,
}

pub fn read_file<T>(path: &'static str) -> io::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    fs::read_to_string(path)?
        .trim()
        .parse()
        .inspect_err(|e| eprintln!("failed to read {path}: {e}"))
        .map_err(io::Error::other)
}

const USAGE_BG: Color = Color(0x181818);
const USAGE_WIDTH: u32 = 4;

pub fn usage_bar(usage: f32) -> impl fmt::Display {
    let usage = usage.clamp(0., 1.);
    let height = (usage * HEIGHT) as u32;
    let fg = Fg(USAGE_COLORS[height as usize]);
    let bg = Bg(USAGE_BG);
    let ramp = Ramp {
        w: USAGE_WIDTH,
        h: height,
    };

    fg.chain(bg).chain(ramp)
}

const USAGE_COLORS: &[Color] = &[
    Color(0x000000),
    Color(0x002F44),
    Color(0x104055),
    Color(0x205C65),
    Color(0x307876),
    Color(0x419587),
    Color(0x53B298),
    Color(0x6AB59B),
    Color(0x88C087),
    Color(0xA4CC77),
    Color(0xBFD867),
    Color(0xD0D360),
    Color(0xE0C855),
    Color(0xE8B94B),
    Color(0xE2923E),
    Color(0xDC6B32),
    Color(0xD54526),
    Color(0xCD1E1A),
    Color(0xBB1817),
    Color(0xFF3A32),
    Color(0xFF1F26),
    Color(0xFF0B1A),
    Color(0xE8001C),
    Color(0xC4001E),
    Color(0x9F0020),
];
