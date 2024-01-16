use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let color = terminal_color::background_color()?;
    dbg!(color);
    Ok(())
}
