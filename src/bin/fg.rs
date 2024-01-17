use term_color::{foreground_color, QueryOptions, Result};

fn main() -> Result<()> {
    println!("{:?}", foreground_color(QueryOptions::default())?);
    Ok(())
}
