use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, fs, io};

use super::read_file;
use crate::color::Color;
use crate::component::Fg;

const CAPACITY: &str = "capacity";
const STATE: &str = "status";
const CURRENT: &str = "current_now";
const CHARGE: &str = "charge_now";
const CHARGE_FULL: &str = "charge_full";
const CHARGE_THRESHOLD: &str = "charge_control_end_threshold";

pub struct Battery {
    bat_path: RefCell<PathBuf>,
}

impl Battery {
    pub fn new(battery: &str) -> Result<Self, NoSuchBattery> {
        let bat_path = PathBuf::from(format!("/sys/class/power_supply/{battery}"));
        bat_path
            .exists()
            .then(|| Self { bat_path: RefCell::new(bat_path) })
            .ok_or(NoSuchBattery)
    }

    fn read_file<T>(&self, file: impl AsRef<Path>) -> io::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        self.bat_path.borrow_mut().push(file);
        let output = read_file(self.bat_path.borrow().as_path());
        self.bat_path.borrow_mut().pop();
        output
    }

    fn read_to_string(&self, file: impl AsRef<Path>) -> io::Result<String> {
        self.bat_path.borrow_mut().push(file);
        let output = fs::read_to_string(self.bat_path.borrow().as_path());
        self.bat_path.borrow_mut().pop();
        output
    }

    fn get_estimate(&self, state: State) -> io::Result<Option<(Hours, Minutes)>> {
        Ok(match state {
            "+" => {
                let current: u64 = self.read_file(CURRENT)?;
                let charge: u64 = self.read_file(CHARGE)?;
                let charge_full: u64 = self.read_file(CHARGE_FULL)?;
                let charge_threshold: u64 = self.read_file(CHARGE_THRESHOLD)?;

                let max_charge = charge_full * charge_threshold / 100;
                let charge_left = max_charge - charge;
                Some(calculate_time_left(charge_left, current))
            }

            "-" => {
                let current: u64 = self.read_file(CURRENT)?;
                let charge: u64 = self.read_file(CHARGE)?;
                Some(calculate_time_left(charge, current))
            }

            _ => None,
        })
    }
}

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let capacity: u8 = self.read_file(CAPACITY).unwrap();

        let status = self.read_to_string(STATE).unwrap();
        let (state, state_color) = match status.trim() {
            "Charging" => ("+", Color::GREEN),
            "Discharging" => ("-", Color::RED),
            "Not charging" => ("o", crate::FG),
            _ => ("?", crate::FG),
        };

        let capacity_color = if capacity <= 30 && status != "Charging" {
            Color::RED
        } else {
            crate::FG
        };

        let estimate = self.get_estimate(state).unwrap();

        if let Some((hours, minutes)) = estimate {
            write!(
                f,
                "{:02}:{:02} {}{}{}{}",
                hours,
                minutes,
                Fg(state_color),
                state,
                Fg(capacity_color),
                capacity
            )
        } else {
            write!(
                f,
                "{}{}{}{}",
                Fg(state_color),
                state,
                Fg(capacity_color),
                capacity
            )
        }
    }
}

type State = &'static str;
type Hours = u64;
type Minutes = u64;

fn calculate_time_left(charge: u64, current: u64) -> (Hours, Minutes) {
    let total_minutes_left = (charge * 60 / current).max(1);

    let hours = total_minutes_left / 60;
    let minutes = total_minutes_left % 60;

    (hours, minutes)
}

#[derive(Debug, thiserror::Error)]
#[error("no such battery")]
pub struct NoSuchBattery;
