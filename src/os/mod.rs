use std::io;
use thiserror::Error;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use macos::*;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) use unix::*;

fn timed_out() -> io::Error {
    io::Error::new(io::ErrorKind::TimedOut, PollReadTimedOutError)
}

#[derive(Debug, Error)]
#[error("poll_read timed out")]
struct PollReadTimedOutError;
