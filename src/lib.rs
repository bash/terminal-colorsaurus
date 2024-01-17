use std::io;
use std::time::Duration;
use thiserror::Error;

mod color;
#[cfg(unix)]
mod os;
#[cfg(unix)]
mod terminal;
#[cfg(unix)]
mod xterm;

#[cfg(unix)]
use xterm as imp;

#[cfg(not(unix))]
use unsupported as imp;

pub use color::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("the terminal responed with invalid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("failed to parse response {0:?}")]
    Parse(String),
    #[error("operation did not complete within {0:?}")]
    Timeout(Duration),
    #[error("unsupported terminal")]
    UnsupportedTerminal,
}

/// Determines the terminal's foreground color.
pub fn foreground_color() -> Result<Color> {
    imp::foreground_color()
}

/// Determines the terminal's background color.
pub fn background_color() -> Result<Color> {
    imp::background_color()
}

#[cfg(not(unix))]
mod unsupported {
    use crate::{Color, Error, Result};

    pub(crate) fn foreground_color() -> Result<Color> {
        Err(Error::UnsupportedTerminal)
    }

    pub(crate) fn background_color() -> Result<Color> {
        Err(Error::UnsupportedTerminal)
    }
}
