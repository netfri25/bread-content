use std::fmt;

use crate::component::{Bg, Fg, Ramp, read_file};
use crate::HEIGHT;

const PATH: &str = "/sys/class/drm/card0/device/gpu_busy_percent";

pub struct Gpu;

impl fmt::Display for Gpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let usage: u8 = read_file(PATH).unwrap();
        let height = usage as f32 * HEIGHT / 100.;
        let height = height as u8;
        let fg = Fg(super::USAGE_COLORS[height as usize]);
        let bg = Bg(super::USAGE_BG);
        let bar = Ramp {
            w: super::USAGE_WIDTH,
            h: height as u32,
        };

        write!(f, "{}{}{}", fg, bg, bar)
    }
}
