use std::fmt;

use crate::component::{read_file, usage_bar};

const PATH: &str = "/sys/class/drm/card0/device/gpu_busy_percent";

pub struct Gpu;

impl fmt::Display for Gpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let usage: u8 = read_file(PATH).unwrap();
        write!(f, "{}", usage_bar(usage as f32 / 100.))
    }
}
