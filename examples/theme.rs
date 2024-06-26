//! This example shows how to detect if the terminal uses
//! a dark-on-light or a light-on-dark theme.

use terminal_colorsaurus::{color_palette, ColorScheme, Error, QueryOptions};

fn main() -> Result<(), display::DisplayAsDebug<Error>> {
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

#[path = "../examples-utils/display.rs"]
mod display;
