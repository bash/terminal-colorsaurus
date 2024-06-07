#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! Determines the background and foreground color of the terminal
//! using the `OSC 10` and `OSC 11` terminal sequence. \
//!
//! This crate helps answer the question *"Is this terminal dark or light?"*.
//!
//! Windows is [not supported][windows_unsupported].
//!
//! ## Features
//! * Background and foreground color detection.
//! * Uses a timeout (for situations with high latency such as an SSH connection).
//! * *Correct* perceived lightness calculation.
//! * Works even if all of stderr, stdout and stdin are redirected.
//! * Safely restores the terminal from raw mode even if the library errors or panicks.
//! * Does not send any escape sequences if `TERM=dumb`.
//!
//! ## Example 1: Test If the Terminal Uses a Dark Background
//! ```no_run
//! use terminal_colorsaurus::{color_scheme, QueryOptions, ColorScheme};
//!
//! let color_scheme = color_scheme(QueryOptions::default()).unwrap();
//! dbg!(color_scheme == ColorScheme::Dark);
//! ```
//!
//! ## Example 2: Query for the Terminal's Foreground Color
//! ```no_run
//! use terminal_colorsaurus::{foreground_color, QueryOptions};
//!
//! let fg = foreground_color(QueryOptions::default()).unwrap();
//! println!("rgb({}, {}, {})", fg.r, fg.g, fg.b);
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
//! <details>
//! <summary><strong>Supported</strong></summary>
//!
//! * Alacritty
//! * Contour
//! * foot
//! * GNOME Terminal, (GNOME) Console, MATE Terminal, XFCE Terminal, (elementary) Terminal, LXTerminal
//! * Hyper
//! * The builtin terminal of JetBrains IDEs (i.e. IntelliJ IDEA, …)
//! * iTerm2
//! * kitty
//! * Konsole
//! * macOS Terminal
//! * Rio
//! * st
//! * Terminology
//! * Termux
//! * tmux (next-3.4)
//! * urxvt (rxvt-unicode)
//! * VSCode (xterm.js)
//! * WezTerm
//! * xterm
//!
//! </details>
//!
//! <details>
//! <summary><strong>Unsupported</strong></summary>
//!
//! * linux
//! * Jetbrains Fleet
//! * iSH
//! * GNU Screen
//!
//! </details>
//!
//! ## Optional Dependencies
//! * [`rgb`] — Enable this feature to convert between [`Color`] and [`rgb::RGB16`] / [`rgb::RGB8`].
//! * [`anstyle`] — Enable this feature to convert [`Color`] to [`anstyle::RgbColor`].
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

use std::time::Duration;

mod color;
mod error;
mod fmt;
mod os;
#[cfg(unix)]
mod xparsecolor;

#[cfg(unix)]
mod xterm;

#[cfg(unix)]
use xterm as imp;

#[cfg(not(unix))]
use unsupported as imp;

#[cfg(docsrs)]
#[cfg_attr(docsrs, doc(cfg(docsrs)))]
#[doc = include_str!("../doc/terminal-survey.md")]
pub mod terminal_survey {}

#[cfg(docsrs)]
#[cfg_attr(docsrs, doc(cfg(docsrs)))]
#[doc = include_str!("../doc/windows.md")]
pub mod windows_unsupported {}

#[cfg(docsrs)]
#[cfg_attr(docsrs, doc(cfg(docsrs)))]
#[doc = include_str!("../doc/latency-rustdoc.md")]
pub mod latency {}

#[cfg(docsrs)]
#[cfg_attr(docsrs, doc(cfg(docsrs)))]
#[doc = include_str!("../doc/feature-detection.md")]
pub mod feature_detection {}

#[cfg(doctest)]
#[doc = include_str!("../readme.md")]
pub mod readme_doctests {}

pub use color::*;

/// The color palette i.e. foreground and background colors of the terminal.
/// Retrieved by calling [`color_palette`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ColorPalette {
    /// The foreground color of the terminal.
    pub foreground: Color,
    /// The background color of the terminal.
    pub background: Color,
}

/// The color scheme of the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_enums)]
pub enum ColorScheme {
    /// The terminal uses a dark background with light text.
    #[default]
    Dark,
    /// The terminal uses a light background with dark text.
    Light,
}

const PERCEPTUAL_MIDDLE_GRAY: u8 = 50;

impl ColorPalette {
    /// Determines if the terminal uses a dark or light background.
    pub fn color_scheme(&self) -> ColorScheme {
        let fg = self.foreground.perceived_lightness();
        let bg = self.background.perceived_lightness();
        if bg < fg {
            ColorScheme::Dark
        } else if bg > fg || bg > PERCEPTUAL_MIDDLE_GRAY {
            ColorScheme::Light
        } else {
            ColorScheme::Dark
        }
    }
}

/// Result used by this library.
pub type Result<T> = std::result::Result<T, Error>;
pub use error::Error;

/// Options to be used with [`foreground_color`] and [`background_color`].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct QueryOptions {
    /// The maximum time spent waiting for a response from the terminal. Defaults to 1 s.
    ///
    /// Consider leaving this on a high value as there might be a lot of latency \
    /// between you and the terminal (e.g. when you're connected via SSH).
    ///
    /// Terminals that don't support querying for colors will
    /// almost always be detected as such before this timeout elapses.
    ///
    /// See [Feature Detection](`feature_detection`) for details on how this works.
    pub timeout: Duration,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(1),
        }
    }
}

/// Detects if the terminal is dark or light.
#[doc = include_str!("../doc/caveats.md")]
pub fn color_scheme(options: QueryOptions) -> Result<ColorScheme> {
    color_palette(options).map(|p| p.color_scheme())
}

/// Queries the terminal for it's color scheme (foreground and background color).
#[doc = include_str!("../doc/caveats.md")]
pub fn color_palette(options: QueryOptions) -> Result<ColorPalette> {
    imp::color_palette(options)
}

/// Queries the terminal for it's foreground color. \
/// If you also need the foreground color it is more efficient to use [`color_palette`] instead.
#[doc = include_str!("../doc/caveats.md")]
pub fn foreground_color(options: QueryOptions) -> Result<Color> {
    imp::foreground_color(options)
}

/// Queries the terminal for it's background color. \
/// If you also need the foreground color it is more efficient to use [`color_palette`] instead.
#[doc = include_str!("../doc/caveats.md")]
pub fn background_color(options: QueryOptions) -> Result<Color> {
    imp::background_color(options)
}

#[cfg(not(unix))]
mod unsupported {
    use crate::{Color, ColorPalette, Error, QueryOptions, Result};

    pub(crate) fn color_palette(_options: QueryOptions) -> Result<ColorPalette> {
        Err(Error::UnsupportedTerminal)
    }

    pub(crate) fn foreground_color(_options: QueryOptions) -> Result<Color> {
        Err(Error::UnsupportedTerminal)
    }

    pub(crate) fn background_color(_options: QueryOptions) -> Result<Color> {
        Err(Error::UnsupportedTerminal)
    }
}

#[cfg(test)]
#[path = "color_scheme_tests.rs"]
mod tests;
