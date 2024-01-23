use std::io;
use thiserror::Error;

#[cfg(not(target_os = "macos"))]
mod poll;
#[cfg(not(target_os = "macos"))]
pub(crate) use poll::*;

pub(super) fn timed_out() -> io::Error {
    io::Error::new(io::ErrorKind::TimedOut, PollReadTimedOutError)
}

#[derive(Debug, Error)]
#[error("poll_read timed out")]
struct PollReadTimedOutError;
