use term_color::background_color;

fn main() {
    let theme = match background_color() {
        Ok(color) if color.perceived_lightness() <= 50 => "dark",
        Ok(_) => "light",
        Err(e) => {
            eprintln!("warning: failed to detect terminal color: {:?}", e);
            std::process::exit(1)
        }
    };
    print!("{}", theme);
}
