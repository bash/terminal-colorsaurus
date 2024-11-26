//! This example shows how to detect if the terminal uses
//! a dark-on-light or a light-on-dark theme.

#![allow(clippy::use_debug)]

use terminal_colorsaurus::{color_palette, Error};

fn main() -> Result<(), display::DisplayAsDebug<Error>> {
    let colors = color_palette()?;
    let theme = colors.color_scheme();

    println!(
        "{theme:?}, fg: {}, bg: {}",
        colors.foreground.perceived_lightness(),
        colors.background.perceived_lightness()
    );

    Ok(())
}

#[path = "../examples-utils/display.rs"]
mod display;
