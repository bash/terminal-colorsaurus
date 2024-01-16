use crate::terminal::TerminalKind;
use crate::xterm::estimate_timeout;
use os::run_in_raw_mode;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::time::Duration;
use thiserror::Error;

mod color;
mod os;
mod terminal;
mod xterm;

pub use color::*;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] io::Error),
    #[error("the terminal responed with invalid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("failed to parse response {0:?}")]
    Parse(String),
    #[error("operation did not complete within {0:?}")]
    Timeout(Duration),
    #[error("unsupported terminal")]
    UnsupportedTerminal,
}

/// Determines the terminal's foreground color.
pub fn foreground_color() -> Result<Color> {
    query_color("\x1b]10;?\x07", TerminalKind::from_env())
}

/// Determines the terminal's background color.
pub fn background_color() -> Result<Color> {
    query_color("\x1b]11;?\x07", TerminalKind::from_env())
}

fn query_color(query: &str, terminal: TerminalKind) -> Result<Color> {
    query_color_raw(query, terminal).and_then(parse_response)
}

fn parse_response(response: String) -> Result<Color> {
    response
        .strip_prefix("\x1b]11;")
        .and_then(|response| {
            response
                .strip_suffix("\x07")
                .or(response.strip_suffix("\x1b\\"))
        })
        .and_then(|response| Color::parse_x11(&response))
        .ok_or_else(|| Error::Parse(response))
}

fn query_color_raw(query: &str, terminal: TerminalKind) -> Result<String> {
    if let TerminalKind::Unsupported = terminal {
        return Err(Error::UnsupportedTerminal);
    }

    let mut tty = tty()?;
    run_in_raw_mode(tty.as_raw_fd(), move || match terminal {
        TerminalKind::Unsupported => unreachable!(),
        TerminalKind::Supported => Ok(xterm::query(&mut tty, query, xterm::MAX_TIMEOUT)?.0),
        // TerminalKind::Passthrough(Passthrough::Screen) => {
        //     let timeout = estimate_timeout(&mut tty)?;
        //     Ok(xterm::query(&mut tty, &format!("{ESC}P{query}"), timeout)?.0)
        // }
        TerminalKind::Unknown => {
            // We use a well-supported sequence such as CSI C to measure the latency.
            // this is to avoid mixing up the case where the terminal is slow to respond
            // (e.g. because we're connected via SSH and have a slow connection)
            // with the case where the terminal does not support querying for colors.
            let timeout = estimate_timeout(&mut tty)?;
            Ok(xterm::query(&mut tty, query, timeout)?.0)
        }
    })
}

// TODO: Re-use already opened tty
fn tty() -> io::Result<File> {
    // Ok(unsafe { File::from_raw_fd(libc::STDIN_FILENO) })
    OpenOptions::new().read(true).write(true).open("/dev/tty")
}
