use std::io::{BufRead as _, BufReader};
use std::{fmt, fs, io};

macro_rules! interface {
    () => {
        "wlp98s0"
    };
}

const WIRELESS: &str = "/proc/net/wireless";
const STATE: &str = concat!("/sys/class/net/", interface!(), "/operstate");

pub struct Wifi;

impl fmt::Display for Wifi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = fs::read_to_string(STATE).unwrap();
        let state = state.trim();

        if state == "down" {
            return write!(f, "{}", state);
        }

        let quality = get_quality(interface!()).unwrap().unwrap_or_default();
        write!(f, "{} {}", state, quality)
    }
}

fn get_quality(interface: &'static str) -> io::Result<Option<u8>> {
    let Some(dbm) = get_dbm(interface)? else {
        return Ok(None);
    };

    // source: https://codeberg.org/dnkl/yambar/src/commit/abeffbd9a9fd0b2133343e1149e65d4a795a43d0/modules/network.c#L209
    let quality = 2 * (100u8.saturating_sub(dbm));
    let quality = quality.min(100);

    Ok(Some(quality))
}

fn get_dbm(interface: &'static str) -> io::Result<Option<u8>> {
    let mut file = BufReader::new(fs::File::open(WIRELESS)?);
    let mut buf = String::new();

    // skip the first 2 lines
    file.read_line(&mut buf)?;
    buf.clear();
    file.read_line(&mut buf)?;

    while {
        buf.clear();
        file.read_line(&mut buf)? != 0
    } {
        if !buf.starts_with(interface) {
            continue;
        }

        // face tus link level noise nwid crypt frag retry misc beacon 22
        let Some(dbm_text) = buf.split_ascii_whitespace().nth(3) else {
            return Ok(None);
        };

        let dbm = dbm_text
            .trim_start_matches('-')
            .trim_end_matches('.')
            .parse()
            .map_err(io::Error::other)?;

        return Ok(Some(dbm));
    }

    Ok(None)
}
