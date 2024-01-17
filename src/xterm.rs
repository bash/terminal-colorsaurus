use crate::os::poll_read;
use crate::Result;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{Read, Write as _};
use std::os::fd::AsRawFd;
use std::str::from_utf8;
use std::time::{Duration, Instant};

pub(crate) const MIN_TIMEOUT: Duration = Duration::from_millis(100);
pub(crate) const MAX_TIMEOUT: Duration = Duration::from_secs(1);

pub(crate) fn estimate_timeout(tty: &mut File) -> Result<Duration> {
    let (_, latency) = query(tty, "\x1b[c", MAX_TIMEOUT)?;
    let timeout = latency * 2; // We want to be in the same ballpark as the latency of our test query. Factor 2 is mostly arbitrary.
    Ok(min(max(timeout, MIN_TIMEOUT), MAX_TIMEOUT))
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
