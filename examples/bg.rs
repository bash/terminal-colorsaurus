use std::error::Error;
use terminal_colorsaurus::{background_color, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let fg = background_color(QueryOptions::default()).unwrap();
    println!("rgb({}, {}, {})", fg.red >> 8, fg.green >> 8, fg.blue >> 8);
    Ok(())
}
