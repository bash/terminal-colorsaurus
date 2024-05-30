use std::error::Error;
use std::fmt;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use macos::*;
#[cfg(all(unix, not(target_os = "macos")))]
mod unix;
#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) use unix::*;

#[derive(Debug)]
struct PollReadTimedOutError;

impl fmt::Display for PollReadTimedOutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "poll_read timed out")
    }
}

impl Error for PollReadTimedOutError {}
