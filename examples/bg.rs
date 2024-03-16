//! This example shows how to retrieve the terminal's background color.

use std::error::Error;
use terminal_colorsaurus::{background_color, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let bg = background_color(QueryOptions::default())?;
    let bg_8bit = bg.scale_to_8bit();
    println!("rgb16({}, {}, {})", bg.r, bg.g, bg.b);
    println!("rgb8({}, {}, {})", bg_8bit.0, bg_8bit.1, bg_8bit.2);
    Ok(())
}
