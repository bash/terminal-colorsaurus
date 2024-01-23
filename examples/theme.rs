//! This example shows how to detect if the terminal uses
//! a dark-on-light or a light-on-dark theme.

use std::error::Error;
use terminal_colorsaurus::{color_scheme, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let colors = color_scheme(QueryOptions::default())?;

    let theme = if colors.is_light_on_dark() {
        "light on dark"
    } else {
        "dark on light"
    };

    println!(
        "{theme}, fg: {}, bg: {}",
        colors.foreground.perceived_lightness(),
        colors.background.perceived_lightness()
    );

    Ok(())
}
