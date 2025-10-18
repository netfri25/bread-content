use std::fmt;

use crate::color::Color;
use crate::component::{Bg, Fg, Ramp};
use crate::{HEIGHT, SYS};

const CPU_BAR_WIDTH: u32 = 4;

pub struct Cpu;

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sys = SYS.lock().unwrap();
        sys.refresh_cpu_usage();
        for cpu in sys.cpus() {
            let height = cpu.cpu_usage().round() * HEIGHT / 100.;
            let height = height as u8;
            let fg = Fg(COLORS[height as usize]);
            let bg = Bg(Color(0x181818));
            let bar = Ramp {
                w: CPU_BAR_WIDTH,
                h: height as u32,
            };
            write!(f, "{}{}{}", fg, bg, bar)?;
        }

        Ok(())
    }
}

const COLORS: &[Color] = &[
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
