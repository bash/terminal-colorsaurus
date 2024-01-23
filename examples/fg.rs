//! This example shows how to retrieve the terminal's foreground color.

use std::error::Error;
use terminal_colorsaurus::{foreground_color, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let fg = foreground_color(QueryOptions::default())?;
    println!("rgb({}, {}, {})", fg.r >> 8, fg.g >> 8, fg.b >> 8);
    Ok(())
}
