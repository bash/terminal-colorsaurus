//! This example shows how to detect if the terminal uses
//! a dark-on-light or a light-on-dark theme.

use terminal_colorsaurus::{color_scheme, ColorScheme, Error, QueryOptions};

fn main() -> Result<(), display::DisplayAsDebug<Error>> {
    let theme = color_scheme(QueryOptions::default())?;
    let theme_name = match theme {
        ColorScheme::Dark => "dark",
        ColorScheme::Light => "light",
    };

    println!("{theme_name}");

    Ok(())
}

#[path = "../examples-utils/display.rs"]
mod display;
