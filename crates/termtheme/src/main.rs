use anstyle::{AnsiColor, Style};
use clap::Parser;
use std::{
    fmt::{self, Display},
    io::{self, stdout, IsTerminal},
    process::exit,
};
use terminal_colorsaurus::{color_scheme, ColorScheme, QueryOptions};

fn main() {
    let args = Args::parse();
    if !stdout().is_terminal() && !args.force {
        display_error("stdout is not connected to a terminal");
        display_help(
            "use '--force' if you're sure that no other process is trying to write to the terminal",
        );
        exit(1);
    }
    match color_scheme(QueryOptions::default()) {
        Ok(s) => display_theme(s, !args.no_newline),
        Err(e) => {
            display_error(e);
            exit(1);
        }
    }
}

fn display_theme(color_scheme: ColorScheme, newline: bool) {
    if newline {
        println!("{}", DisplayName(color_scheme))
    } else {
        print!("{}", DisplayName(color_scheme))
    }
}

fn display_error(e: impl Display) {
    if use_colors(&io::stderr()) {
        let style = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
        eprintln!("{style}error:{style:#} {e}");
    } else {
        eprintln!("error: {e}");
    }
}

fn display_help(e: impl Display) {
    if use_colors(&io::stderr()) {
        let style = Style::new()
            .bold()
            .fg_color(Some(AnsiColor::BrightBlue.into()));
        eprintln!("{style}tip:{style:#} {e}");
    } else {
        eprintln!("tip: {e}");
    }
}

struct DisplayName(ColorScheme);

impl fmt::Display for DisplayName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ColorScheme::Dark => f.write_str("dark"),
            ColorScheme::Light => f.write_str("light"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Do not output a newline.
    #[arg(short = 'n')]
    no_newline: bool,
    /// Always query the terminal even when stdout is redirected.
    #[arg(short = 'f', long)]
    force: bool,
}

trait Stream: io::Write + io::IsTerminal {}

impl<T> Stream for T where T: io::Write + io::IsTerminal {}

// Copied from <https://github.com/rust-cli/anstyle/blob/4b8b9c59901ad5c08191303b59645c0139240acb/crates/anstream/src/auto.rs#L187>
// which is licensed under Apache 2.0 or MIT.
fn use_colors(raw: &dyn Stream) -> bool {
    let clicolor = anstyle_query::clicolor();
    let clicolor_enabled = clicolor.unwrap_or(false);
    let clicolor_disabled = !clicolor.unwrap_or(true);
    if anstyle_query::no_color() {
        false
    } else if anstyle_query::clicolor_force() {
        true
    } else if clicolor_disabled {
        false
    } else {
        raw.is_terminal()
            && (anstyle_query::term_supports_color() || clicolor_enabled || anstyle_query::is_ci())
    }
}
