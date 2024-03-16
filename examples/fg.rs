//! This example shows how to retrieve the terminal's foreground color.

use std::error::Error;
use terminal_colorsaurus::{foreground_color, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let fg = foreground_color(QueryOptions::default())?;
    let fg_8bit = fg.scale_to_8bit();
    println!("rgb16({}, {}, {})", fg.r, fg.g, fg.b);
    println!("rgb8({}, {}, {})", fg_8bit.0, fg_8bit.1, fg_8bit.2);
    Ok(())
}
