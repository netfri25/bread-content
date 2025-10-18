use std::fmt;

pub mod battery;
pub mod cpu;
pub mod memory;
pub mod temperature;
pub mod time;
pub mod wifi;

pub use battery::*;
pub use cpu::*;
use derive_more::Display;
pub use memory::*;
pub use temperature::*;
pub use time::*;
pub use wifi::*;

use crate::color::Color;

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
