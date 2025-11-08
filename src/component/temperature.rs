use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::color::Color;
use crate::component::Fg;

pub const COOL: Color = Color(0x2fb7c4);
pub const WARM: Color = Color(0xe0c555);
pub const HOT: Color = Color(0x3e493f);

pub struct Temperature {
    temp_path: PathBuf,
}

impl Temperature {
    pub fn new(zone: &str) -> Result<Self, NoSuchThermalZone> {
        let temp_path = PathBuf::from(format!("/sys/class/thermal/{zone}/temp"));
        temp_path
            .exists()
            .then_some(Self { temp_path })
            .ok_or(NoSuchThermalZone)
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // kilo-celsius
        let value_kc: u32 = fs::read_to_string(self.temp_path.as_path())
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let temp = value_kc / 1000;

        let color = if temp < 40 {
            COOL
        } else if temp < 50 {
            crate::FG
        } else if temp < 70 {
            WARM
        } else {
            HOT
        };

        write!(f, "{}{}°C", Fg(color), temp)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("no such thermal zone")]
pub struct NoSuchThermalZone;
