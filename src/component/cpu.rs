use std::fmt;

use crate::component::{Bg, Fg, Ramp};
use crate::{HEIGHT, SYS};

pub struct Cpu;

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sys = SYS.lock().unwrap();
        sys.refresh_cpu_usage();
        for cpu in sys.cpus() {
            let height = cpu.cpu_usage().round() * HEIGHT / 100.;
            let height = height as u8;
            let fg = Fg(super::USAGE_COLORS[height as usize]);
            let bg = Bg(super::USAGE_BG);
            let bar = Ramp {
                w: super::USAGE_WIDTH,
                h: height as u32,
            };
            write!(f, "{}{}{}", fg, bg, bar)?;
        }

        Ok(())
    }
}
