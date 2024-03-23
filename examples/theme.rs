//! This example shows how to detect if the terminal uses
//! a dark-on-light or a light-on-dark theme.

use std::error::Error;
use terminal_colorsaurus::{color_palette, ColorScheme, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let colors = color_palette(QueryOptions::default())?;

    let theme = match colors.color_scheme() {
        ColorScheme::Dark => "dark",
        ColorScheme::Light => "light",
    };

    println!(
        "{theme}, fg: {}, bg: {}",
        colors.foreground.perceived_lightness(),
        colors.background.perceived_lightness()
    );

    Ok(())
}
