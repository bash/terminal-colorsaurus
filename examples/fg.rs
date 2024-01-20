use std::error::Error;
use terminal_colorsaurus::{foreground_color, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let fg = foreground_color(QueryOptions::default()).unwrap();
    println!("rgb({}, {}, {})", fg.red >> 8, fg.green >> 8, fg.blue >> 8);
    Ok(())
}
