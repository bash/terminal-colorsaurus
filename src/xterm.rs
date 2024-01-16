use crate::os::poll_read;
use crate::Result;
use std::env;
use std::fs::File;
use std::io::{Read, Write as _};
use std::str::from_utf8;
use std::time::{Duration, Instant};

pub(crate) fn query(tty: &mut File, query: &str, timeout: Duration) -> Result<(String, Duration)> {
    let mut buffer = vec![0; 100];

    write!(tty, "{}", query)?;
    tty.flush()?;

    let start = Instant::now();
    poll_read(libc::STDIN_FILENO, timeout)?;
    let bytes_read = tty.read(&mut buffer)?;
    let duration = start.elapsed();

    let response = from_utf8(&buffer[..bytes_read])?.to_owned();

    Ok((response, duration))
}

fn is_known_unsupported_terminal() -> bool {
    if let Some(term) = env::var_os("TERM") {
        if term == "linux" {
            return true;
        }
    }

    if let Some(term_program) = env::var_os("TERM_PROGRAM") {
        if term_program == "Jetbrains.Fleet" {
            return true;
        }
    }

    false
}
