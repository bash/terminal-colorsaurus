#![allow(clippy::use_debug)]

//! This example shows how to heuristically avoid having a race condition with a pager (e.g. `less`).
//! The race condition occurs because the pager and colorsaurus simultaneously
//! enable/disable raw mode and read/write to the same terminal.
//!
//! The heuristic checks if stdout is connected to a pipe which is a strong
//! indicator that the output is redirected to another process, for instance a pager.
//! Note that this heuristic has both
//! false negatives (output not piped to a pager) and
//! false positives (stderr piped to a pager).
//!
//! Test this example as follows:
//! 1. `cargo run --example pager`—should print the color scheme.
//! 2. `cargo run --example pager | less`—should not print the color scheme.
//! 3. `cargo run --example pager > file.txt`—should print the color scheme.
//! 4. `cargo run --example pager > /dev/null`—should print the color scheme.

use std::error::Error;
use terminal_colorsaurus::{color_scheme, Preconditions, QueryOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let mut options = QueryOptions::default();
    options.preconditions = Preconditions::stdout_not_piped();

    eprintln!("Here's the color scheme: {:#?}", color_scheme(options)?);

    Ok(())
}
