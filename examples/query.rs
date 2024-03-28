//! TODO

use std::env;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{stdout, Write as _};
use std::thread::sleep;
use std::time::Duration;
use terminal_colorsaurus::{color_palette, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let result_file_path = env::args().nth(1).unwrap();
    let mut result_file = OpenOptions::new().write(true).open(result_file_path)?;

    println!("\x1b]10;#ffffff\x1b\\");
    println!("\x1b]11;#000000\x1b\\");
    stdout().flush()?;
    sleep(Duration::from_millis(200));

    match color_palette(QueryOptions::default()) {
        Ok(palette) => {
            let fg = palette.foreground.scale_to_8bit();
            let bg = palette.background.scale_to_8bit();

            writeln!(result_file, "{:x}/{:x}/{:x}", fg.0, fg.1, fg.2)?;
            writeln!(result_file, "{:x}/{:x}/{:x}", bg.1, bg.1, bg.2)?;
        }
        Err(e) => {
            writeln!(result_file, "Failed to query terminal colors: {}", e)?;
        }
    };

    Ok(())
}
