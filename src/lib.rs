//! Determines the background and foreground color of the terminal
//! using the `OSC 10` and `OSC 11` terminal sequence.
//!
//! ## Features
//! * Background and foreground color detection.
//! * Uses a variable timeout (for situations with high latency such as an SSH connection).
//! * *Correct* perceived lightness calculation.
//! * Works even if all of stderr, stdout and stdin are redirected.
//! * Safely restores the terminal from raw mode even if the library panicks.
//!
//! ## Supported Terminals
//! * macOS Terminal
//! * iTerm2
//! * Alacritty
//! * VSCode (xterm.js)
//! * IntelliJ IDEA
//! * Contour
//! * GNOME Terminal, (GNOME) Console, MATE Terminal, XFCE Terminal, (elementary) Terminal, LXTerminal
//! * Console
//! * foot
//! * xterm
//!
//! ## Example: Test If the Terminal Uses a Dark Background
//! ```no_run
//! use term_color::background_color;
//!
//! // Perceived lightness is a value between 0 (black) and 100 (white)
//! let is_light = background_color().map(|c| c.perceived_lightness() >= 50).unwrap_or_default();
//! ```
//!
//! ## Variable Timeout
//! TODO

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

/// Result used by this library.
pub type Result<T> = std::result::Result<T, Error>;

/// An error returned by this library.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// I/O error
    #[error("I/O error")]
    Io(#[from] io::Error),
    /// The terminal responed with invalid UTF-8.
    #[error("the terminal responed with invalid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    /// The terminal responded using an unsupported response format.
    #[error("failed to parse response {0:?}")]
    Parse(String),
    /// The query timed out.
    #[error("operation did not complete within {0:?}")]
    Timeout(Duration),
    /// The terminal is known to be unsupported.
    #[error("unsupported terminal")]
    UnsupportedTerminal,
}

/// Queries the terminal for it's foreground color.
pub fn foreground_color() -> Result<Color> {
    imp::foreground_color()
}

/// Queries the terminal for it's background color.
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
