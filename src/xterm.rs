use crate::os::poll_read;
use crate::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{Read, Write as _};
use std::os::fd::AsRawFd;
use std::str::from_utf8;
use std::time::{Duration, Instant};

pub(crate) const MIN_TIMEOUT: Duration = Duration::from_millis(20);
pub(crate) const MAX_TIMEOUT: Duration = Duration::from_secs(1);

pub(crate) fn estimate_timeout(tty: &mut File) -> Result<Duration> {
    // We use a well-supported sequence such as CSI C to measure the latency.
    // this is to avoid mixing up the case where the terminal is slow to respond
    // (e.g. because we're connected via SSH and have a slow connection)
    // with the case where the terminal does not support querying for colors.
    let (_, latency) = query(tty, "\x1b[c", MAX_TIMEOUT)?;
    return Ok(min(max(latency * 2, MIN_TIMEOUT), MAX_TIMEOUT));
}

pub(crate) fn query(tty: &mut File, query: &str, timeout: Duration) -> Result<(String, Duration)> {
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
