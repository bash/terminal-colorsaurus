#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

//! Determines the background and foreground color of the terminal
//! using the `OSC 10` and `OSC 11` terminal sequence.
//!
//! This crate helps answer the question *"Is this terminal dark or light?"*.
//!
//! ## Features
//! * Background and foreground color detection.
//! * Uses a fast and reliable heuristic to detect if the terminal supports color querying.
//! * *Correct* perceived lightness calculation.
//! * Works on Windows (starting with Windows Terminal v1.22).
//! * Safely restores the terminal from raw mode even if the library errors or panicks.
//! * Does not send any escape sequences if `TERM=dumb`.
//! * Works even if all of stderr, stdout and stdin are redirected.
//! * Supports a timeout (for situations with high latency such as an SSH connection).
//!
//! ## Terminal Support
//! `terminal-colorsaurus` works with most modern terminals and has been [tested extensively](`terminal_survey`).
//! It's also really good at [detecting](`feature_detection`) when querying for the terminal's colors is not supported.
//!
//! ## Example 1: Test If the Terminal Uses a Dark Background
//! ```no_run
//! use terminal_colorsaurus::{color_scheme, QueryOptions, ColorScheme};
//!
//! let color_scheme = color_scheme(QueryOptions::default()).unwrap();
//! dbg!(color_scheme == ColorScheme::Dark);
//! ```
//!
//! ## Example 2: Get the Terminal's Foreground Color
//! ```no_run
//! use terminal_colorsaurus::{foreground_color, QueryOptions};
//!
//! let fg = foreground_color(QueryOptions::default()).unwrap();
//! println!("rgb({}, {}, {})", fg.r, fg.g, fg.b);
//! ```
//!
//! ## Optional Dependencies
//! * [`rgb`] — Enable this feature to convert between [`Color`] and [`rgb::RGB16`] / [`rgb::RGB8`].
//! * [`anstyle`] — Enable this feature to convert [`Color`] to [`anstyle::RgbColor`].

use cfg_if::cfg_if;

mod color;
mod error;
mod fmt;

cfg_if! {
    if #[cfg(all(any(unix, windows), not(terminal_colorsaurus_test_unsupported)))] {
        mod io;
        mod quirks;
        mod xterm;
        use xterm as imp;
    } else {
        mod unsupported;
        use unsupported as imp;
    }
}

cfg_if! {
    if #[cfg(docsrs)] {
        #[doc(cfg(docsrs))]
        #[doc = include_str!("../doc/terminal-survey.md")]
        pub mod terminal_survey {}

        #[doc(cfg(docsrs))]
        #[doc = include_str!("../doc/latency-rustdoc.md")]
        pub mod latency {}

        #[doc(cfg(docsrs))]
        #[doc = include_str!("../doc/feature-detection.md")]
        pub mod feature_detection {}

        #[doc(cfg(docsrs))]
        #[doc = include_str!("../doc/comparison.md")]
        pub mod comparison {}
    }
}

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
///
/// The easiest way to retrieve the color scheme
/// is by calling [`color_scheme`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_enums)]
#[doc(alias = "Theme")]
pub enum ColorScheme {
    /// The terminal uses a dark background with light text.
    #[default]
    Dark,
    /// The terminal uses a light background with dark text.
    Light,
}

impl ColorPalette {
    /// Determines if the terminal uses a dark or light background.
    pub fn color_scheme(&self) -> ColorScheme {
        let fg = self.foreground.perceived_lightness_f32();
        let bg = self.background.perceived_lightness_f32();
        if bg < fg {
            ColorScheme::Dark
        } else if bg > fg || bg > 0.5 {
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
/// You should almost always use the unchanged [`QueryOptions::default`] value.
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
    pub timeout: std::time::Duration,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(1),
        }
    }
}

/// Detects if the terminal is dark or light.
#[doc = include_str!("../doc/caveats.md")]
#[doc(alias = "theme")]
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
#[doc(alias = "fg")]
pub fn foreground_color(options: QueryOptions) -> Result<Color> {
    imp::foreground_color(options)
}

/// Queries the terminal for it's background color. \
/// If you also need the foreground color it is more efficient to use [`color_palette`] instead.
#[doc = include_str!("../doc/caveats.md")]
#[doc(alias = "bg")]
pub fn background_color(options: QueryOptions) -> Result<Color> {
    imp::background_color(options)
}

#[cfg(test)]
#[path = "color_scheme_tests.rs"]
mod tests;
