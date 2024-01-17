use std::error::Error;

use term_color::QueryOptions;

fn main() -> Result<(), Box<dyn Error>> {
    let color = term_color::background_color(QueryOptions::default())?;
    dbg!(color.perceived_lightness());
    dbg!(color.perceived_lightness() <= 50);
    dbg!(color);
    Ok(())
}
