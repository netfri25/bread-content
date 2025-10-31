use std::fmt;

use crate::component::usage_bar;
use crate::SYS;

pub struct Cpu;

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sys = SYS.lock().unwrap();
        sys.refresh_cpu_usage();
        for cpu in sys.cpus() {
            let usage = cpu.cpu_usage().round() / 100.;
            write!(f, "{}", usage_bar(usage))?;
        }

        Ok(())
    }
}
