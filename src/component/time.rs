use std::fmt;

pub struct Time;

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ts = time_format::now().unwrap();
        let comps = time_format::components_local(ts).unwrap();
        let day = week_day_name(comps.week_day);

        write!(
            f,
            " {:02}:{:02} {} {:02}/{:02} ",
            comps.hour, comps.min, day, comps.month_day, comps.month
        )
    }
}

fn week_day_name(day: u8) -> &'static str {
    match day {
        0 => "Sun",
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => "???",
    }
}
