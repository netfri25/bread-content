use std::fmt;
use std::path::PathBuf;

use crate::component::{read_file, usage_bar};

pub struct Gpu {
    busy_path: PathBuf,
}

impl Gpu {
    pub fn new(card: &str) -> Result<Self, NoSuchCard> {
        let busy_path = PathBuf::from(format!("/sys/class/drm/{card}/device/gpu_busy_percent"));
        busy_path
            .exists()
            .then_some(Self { busy_path })
            .ok_or(NoSuchCard)
    }
}

impl fmt::Display for Gpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let usage: u8 = read_file(self.busy_path.as_path()).unwrap();
        write!(f, "{}", usage_bar(usage as f32 / 100.))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("no such gpu card")]
pub struct NoSuchCard;
