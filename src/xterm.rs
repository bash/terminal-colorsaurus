use crate::os::{poll_read, run_in_raw_mode, tty, Tty};
use crate::terminal::TerminalKind;
use crate::{Color, Error, Result};
use std::cmp::{max, min};
use std::io::{Read, Write as _};
use std::os::fd::AsRawFd;
use std::str::from_utf8;
use std::time::{Duration, Instant};

const MIN_TIMEOUT: Duration = Duration::from_millis(100);
const MAX_TIMEOUT: Duration = Duration::from_secs(1);

pub(crate) fn foreground_color() -> Result<Color> {
    query_color("\x1b]10;?\x07", TerminalKind::from_env())
}

pub(crate) fn background_color() -> Result<Color> {
    query_color("\x1b]11;?\x07", TerminalKind::from_env())
}

fn query_color(query: &str, terminal: TerminalKind) -> Result<Color> {
    query_color_raw(query, terminal).and_then(parse_response)
}

fn parse_response(response: String) -> Result<Color> {
    response
        .strip_prefix("\x1b]11;")
        .and_then(|response| {
            response
                .strip_suffix('\x07')
                .or(response.strip_suffix("\x1b\\"))
        })
        .and_then(Color::parse_x11)
        .ok_or_else(|| Error::Parse(response))
}

fn query_color_raw(q: &str, terminal: TerminalKind) -> Result<String> {
    if let TerminalKind::Unsupported = terminal {
        return Err(Error::UnsupportedTerminal);
    }

    let mut tty = tty()?;
    run_in_raw_mode(tty.as_raw_fd(), move || match terminal {
        TerminalKind::Unsupported => unreachable!(),
        TerminalKind::Supported => Ok(query(&mut tty, q, MAX_TIMEOUT)?.0),
        TerminalKind::Unknown => {
            // We use a well-supported sequence such as CSI C to measure the latency.
            // this is to avoid mixing up the case where the terminal is slow to respond
            // (e.g. because we're connected via SSH and have a slow connection)
            // with the case where the terminal does not support querying for colors.
            let timeout = estimate_timeout(&mut tty)?;
            Ok(query(&mut tty, q, timeout)?.0)
        }
    })
}

fn estimate_timeout(tty: &mut Tty) -> Result<Duration> {
    let (_, latency) = query(tty, "\x1b[c", MAX_TIMEOUT)?;
    let timeout = latency * 2; // We want to be in the same ballpark as the latency of our test query. Factor 2 is mostly arbitrary.
    Ok(min(max(timeout, MIN_TIMEOUT), MAX_TIMEOUT))
}

fn query(tty: &mut Tty, query: &str, timeout: Duration) -> Result<(String, Duration)> {
    let mut buffer = vec![0; 100];

    write!(tty, "{}", query)?;
    tty.flush()?;

    let start = Instant::now();
    poll_read(tty.as_raw_fd(), timeout)?;
    let bytes_read = tty.read(&mut buffer)?;
    let duration = start.elapsed();

    let response = from_utf8(&buffer[..bytes_read])?.to_owned();

    Ok((response, duration))
}
