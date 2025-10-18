use std::fmt;

use sysinfo::MemoryRefreshKind;

use crate::SYS;

pub struct Memory;

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sys = SYS.lock().unwrap();
        sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());
        let bytes = sys.used_memory();
        let mb = bytes >> 20;
        write!(f, "{:5}", mb)
    }
}
