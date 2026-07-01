use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::color::Color;
use crate::component::Fg;

pub const COOL: Color = Color(0x2fb7c4);
pub const WARM: Color = Color(0xe0c555);
pub const HOT: Color = Color::RED;

pub struct Temperature {
    temp_path: PathBuf,
}

impl Temperature {
    pub fn create(zone: &str) -> Result<Self, ThermalZoneError> {
        let dir = PathBuf::from("/sys/class/hwmon/");

        let iter = dir
            .read_dir()
            .map_err(|_| ThermalZoneError::NoHwmonDirectory)?;

        let mut found = None;

        for entry in iter {
            let Ok(entry) = entry else { continue };

            if !entry.file_name().as_encoded_bytes().starts_with(b"hwmon") {
                continue;
            }

            let mut path = entry.path();
            path.push("name");

            let Ok(name) = fs::read_to_string(&path) else {
                continue
            };

            if name.trim() != zone {
                continue
            }

            path.pop();
            path.push("temp1_input");
            found = Some(path);
            break
        }

        let Some(temp_path) = found else {
            return Err(ThermalZoneError::NoSuchThermalZone);
        };

        Ok(Self { temp_path })
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // milli-celsius
        let value_mc: u32 = fs::read_to_string(self.temp_path.as_path())
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let temp = value_mc / 1000;

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
pub enum ThermalZoneError {
    #[error("no such thermal zone")]
    NoSuchThermalZone,

    #[error("/sys/class/hwmon directory does not exist")]
    NoHwmonDirectory,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
