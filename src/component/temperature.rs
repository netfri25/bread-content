use std::fmt;
use std::fs;

use crate::color::Color;
use crate::component::Fg;

const PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub const COOL: Color = Color(0x2fb7c4);
pub const WARM: Color = Color(0xe0c555);
pub const HOT: Color = Color(0x3e493f);

pub struct Temperature;

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // kilo-celsius
        let value_kc: u32 = fs::read_to_string(PATH).unwrap().trim().parse().unwrap();

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
