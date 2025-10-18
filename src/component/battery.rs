use std::str::FromStr;
use std::{fmt, fs, io};

use crate::color::Color;
use crate::component::Fg;

macro_rules! path {
    () => {
        "/sys/class/power_supply/BAT1"
    };
}

const CAPACITY: &str = concat!(path!(), "/capacity");
const STATE: &str = concat!(path!(), "/status");
const CURRENT: &str = concat!(path!(), "/current_now");
const CHARGE: &str = concat!(path!(), "/charge_now");
const CHARGE_FULL: &str = concat!(path!(), "/charge_full");
const CHARGE_THRESHOLD: &str = concat!(path!(), "/charge_control_end_threshold");

pub struct Battery;

impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let capacity: u8 = read_file(CAPACITY).unwrap();

        let status = fs::read_to_string(STATE).unwrap();
        let (state, state_color) = match status.trim() {
            "Charging" => ("+", Color::GREEN),
            "Discharging" => ("-", Color::RED),
            "Not charging" => ("o", crate::FG),
            _ => ("?", crate::FG),
        };

        let capacity_color = if capacity <= 30 {
            Color::RED
        } else {
            crate::FG
        };

        let estimate = get_estimate(state).unwrap();

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

fn read_file<T>(path: &'static str) -> io::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    fs::read_to_string(path)?
        .trim()
        .parse()
        .inspect_err(|e| eprintln!("failed to read {path}: {e}"))
        .map_err(io::Error::other)
}

type State = &'static str;
type Hours = u64;
type Minutes = u64;

fn get_estimate(state: State) -> io::Result<Option<(Hours, Minutes)>> {
    Ok(match state {
        "+" => {
            let current: u64 = read_file(CURRENT)?;
            let charge: u64 = read_file(CHARGE)?;
            let charge_full: u64 = read_file(CHARGE_FULL)?;
            let charge_threshold: u64 = read_file(CHARGE_THRESHOLD)?;

            let max_charge = charge_full * charge_threshold / 100;
            let charge_left = max_charge - charge;
            Some(calculate_time_left(charge_left, current))
        }

        "-" => {
            let current: u64 = read_file(CURRENT)?;
            let charge: u64 = read_file(CHARGE)?;
            Some(calculate_time_left(charge, current))
        }

        _ => None,
    })
}

fn calculate_time_left(charge: u64, current: u64) -> (Hours, Minutes) {
    let total_minutes_left = (charge * 60 / current).max(1);

    let hours = total_minutes_left / 60;
    let minutes = total_minutes_left % 60;

    (hours, minutes)
}
