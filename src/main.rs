use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _color = terminal_color::background_color()?;
    Ok(())
}
