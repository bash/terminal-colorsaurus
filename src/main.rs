use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let color = terminal_color::background_color()?;
    dbg!(color.perceived_lightness());
    dbg!(color.perceived_lightness() <= 50);
    dbg!(color);
    Ok(())
}
