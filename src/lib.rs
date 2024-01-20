#![cfg_attr(feature = "__nightly", feature(doc_cfg, doc_auto_cfg))]

//! Determines the background and foreground color of the terminal
//! using the `OSC 10` and `OSC 11` terminal sequence.
//!
//! On Windows, the colors are queried using the Win32 Console API.
//!
//! This is useful for answering the question *"Is this terminal dark or light?"*.
//!
//! ## Features
//! * Background and foreground color detection.
//! * Uses a variable timeout (for situations with high latency such as an SSH connection).
//! * *Correct* perceived lightness calculation.
//! * Works even if all of stderr, stdout and stdin are redirected.
//! * Safely restores the terminal from raw mode even if the library errors or panicks.
//! * Does not send any escape sequences if `TERM=dumb`.
//!
//! ## Example 1: Test If the Terminal Uses a Dark Background
//! ```no_run
//! use terminal_colorsaurus::{color_scheme, QueryOptions};
//!
//! let colors = color_scheme(QueryOptions::default()).unwrap();
//! dbg!(colors.is_dark_on_light());
//! ```
//!
//! ## Example 2: Query for the Terminal's Foreground Color
//! ```no_run
//! use terminal_colorsaurus::{foreground_color, QueryOptions};
//!
//! let fg = foreground_color(QueryOptions::default()).unwrap();
//! println!("rgb({}, {}, {})", fg.red, fg.green, fg.blue);
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
//! * Windows Console (conhost)
//!
//! ### Unsupported
//! * linux
//! * Jetbrains Fleet
//! * Windows Terminal
//!
//! ## Variable Timeout
//! Knowing whether or not a terminal supports querying for the
//! foreground and background colors hard to reliably detect.
//! Employing a fixed timeout is not the best options because the terminal might support the sequence
//! but have a lot of latency (e.g. the user is connected over SSH).
//!
//! This library assumes that the terminal support the [widely supported][`terminal_survey`] `CSI c` sequence.
//! Using this, it measures the latency. This measurement then informs the timeout enforced on the actual query.
//!
//! ## Comparison with Other Crates
//! ### [termbg]
//! * Is hardcoded to use stdin/stderr for communicating with the terminal. \
//!   This means that it does not work if some or all of these streams are redirected.
//! * Pulls in an async runtime for the timeout.
//! * Does not calculate the perceived lightness, but another metric.
//!
//! ### [terminal-light]
//! * Is hardcoded to use stdin/stdout for communicating with the terminal.
//! * Does not report the colors, only the color's luma.
//! * Does not calculate the perceived lightness, but another metric.
//!
//! [termbg]: https://docs.rs/termbg
//! [terminal-light]: https://docs.rs/terminal-light

use std::io;
use std::time::Duration;
use thiserror::Error;

mod color;
mod os;

#[cfg(unix)]
mod terminal;
#[cfg(windows)]
mod winapi;
#[cfg(unix)]
mod xterm;

#[cfg(windows)]
use winapi as imp;
#[cfg(unix)]
use xterm as imp;

#[cfg(not(any(unix, windows)))]
use unsupported as imp;

#[cfg(feature = "__docs")]
#[doc = include_str!("../doc/terminal-survey.md")]
pub mod terminal_survey {}

#[cfg(feature = "__test_readme")]
#[doc = include_str!("../readme.md")]
pub mod readme {}

pub use color::*;

/// The color scheme i.e. foreground and background colors of the terminal.
/// Retrieved by calling [`color_scheme`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ColorScheme {
    pub foreground: Color,
    pub background: Color,
}

impl ColorScheme {
    /// Tests if this color scheme uses dark text on a light background.
    /// This is done by computing and comparing the perceived brightness of the two colors.
    pub fn is_dark_on_light(&self) -> bool {
        self.foreground.perceived_lightness() <= self.background.perceived_lightness()
    }

    /// Tests if this color scheme uses light text on a dark background.
    /// This is done by computing and comparing the perceived brightness of the two colors.
    ///
    /// Note that `is_light_on_dark = !is_dark_on_light`.
    pub fn is_light_on_dark(&self) -> bool {
        !self.is_dark_on_light()
    }
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Queries the terminal for it's color scheme (foreground and background color).
pub fn color_scheme(options: QueryOptions) -> Result<ColorScheme> {
    imp::color_scheme(options)
}

/// Queries the terminal for it's foreground color. \
/// If you also need the foreground color it is more efficient to use [`color_scheme`] instead.
pub fn foreground_color(options: QueryOptions) -> Result<Color> {
    imp::foreground_color(options)
}

/// Queries the terminal for it's background color. \
/// If you also need the foreground color it is more efficient to use [`color_scheme`] instead.
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
