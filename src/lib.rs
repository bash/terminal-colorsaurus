//! Determines the background and foreground color of the terminal
//! using the `OSC 10` and `OSC 11` terminal sequence.
//!
//! ## Features
//! * Background and foreground color detection.
//! * Uses a variable timeout (for situations with high latency such as an SSH connection).
//! * *Correct* perceived lightness calculation.
//! * Works even if all of stderr, stdout and stdin are redirected.
//! * Safely restores the terminal from raw mode even if the library errors or panicks.
//! * Does not send any escape sequences if `TERM=dumb`.
//!
//! ## Example: Test If the Terminal Uses a Dark Background
//! ```no_run
//! use term_color::{background_color, QueryOptions};
//!
//! let bg = background_color(QueryOptions::default());
//! // Perceived lightness is a value between 0 (black) and 100 (white)
//! let is_light = bg.map(|c| c.perceived_lightness() >= 50).unwrap_or_default();
//! ```
//!
//! ## Terminals
//! The following terminals have known support or non-support for
//! querying for the background/foreground colors.
//!
//! Note that terminals that support the relevant terminal
//! sequences automatically work with this library even if they
//! are not explicitly listed below.
//!
//! ### Supported
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
//! * tmux (next-3.4)
//!
//! ### Unsupported
//! * linux
//! * Jetbrains Fleet
//!
//! ## Variable Timeout
//! Knowing whether or not a terminal supports querying for the
//! foreground and background colors hard to reliably detect.
//! Employing a fixed timeout is not the best options because the terminal might support the sequence
//! but have a lot of latency (e.g. the user is connected over SSH).
//!
//! This library assumes that the terminal support the [widely supported][terminal-survey] `CSI c` sequence.
//! Using this, it measures the latency. This measurement then informs the timeout enforced on the actual query.
//!
//! ## Comparison with Other Libraries
//! * termbg: TODO
//! * dark-light: TODO
//!
//! [terminal-survey]: https://github.com/bash/term-color/blob/main/doc/terminal-survey.md

use std::io;
use std::time::Duration;
use thiserror::Error;

mod color;
mod os;

mod terminal;
mod xterm;

#[cfg(any(unix, windows))]
use xterm as imp;

#[cfg(not(any(unix, windows)))]
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
    /// The query timed out. This can happen because \
    /// either the terminal does not support querying for colors \
    /// or the terminal has a lot of latency (e.g. when connected via SSH).
    #[error("operation did not complete within {0:?}")]
    Timeout(Duration),
    /// The terminal is known to be unsupported.
    #[error("unsupported terminal")]
    UnsupportedTerminal,
}

/// Options to be used with [`foreground_color`] and [`background_color`].
#[derive(Debug)]
#[non_exhaustive]
pub struct QueryOptions {
    /// The maximum time spent waiting for a response from the terminal \
    /// even when we *know* that the terminal supports querying for colors. Defaults to 1 s.
    ///
    /// Note that this timeout might not always apply as we use a variable timeout
    /// for the color query.
    ///
    ///  Consider leaving this on a high value as there might be a lot of latency \
    /// between you and the terminal (e.g. when you're connected via SSH).
    pub max_timeout: Duration,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            max_timeout: Duration::from_secs(1),
        }
    }
}

/// Queries the terminal for it's foreground color.
pub fn foreground_color(options: QueryOptions) -> Result<Color> {
    imp::foreground_color(options)
}

/// Queries the terminal for it's background color.
pub fn background_color(options: QueryOptions) -> Result<Color> {
    imp::background_color(options)
}

#[cfg(not(any(unix, windows)))]
mod unsupported {
    use crate::{Color, Error, QueryOptions, Result};

    pub(crate) fn foreground_color(_options: QueryOptions) -> Result<Color> {
        Err(Error::UnsupportedTerminal)
    }

    pub(crate) fn background_color(_options: QueryOptions) -> Result<Color> {
        Err(Error::UnsupportedTerminal)
    }
}
