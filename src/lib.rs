use crate::terminal::TerminalKind;
use crate::xterm::estimate_timeout;
use os::run_in_raw_mode;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::time::Duration;
use thiserror::Error;

mod os;
mod terminal;
mod xterm;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("I/O Error")]
    Io(#[from] io::Error),
    #[error("the terminal responed with invalid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("an operation did not complete within {0:?}")]
    Timeout(Duration),
    #[error("this terminal is not supported")]
    UnsupportedTerminal,
}

pub fn foreground_color() -> Result<String> {
    color_query_raw("\x1b]10;?\x07", TerminalKind::from_env())
}

pub fn background_color() -> Result<String> {
    color_query_raw("\x1b]11;?\x07", TerminalKind::from_env())
}

fn color_query_raw(query: &str, terminal: TerminalKind) -> Result<String> {
    if let TerminalKind::Unsupported = terminal {
        return Err(Error::UnsupportedTerminal);
    }

    let mut tty = tty()?;
    run_in_raw_mode(tty.as_raw_fd(), move || match terminal {
        TerminalKind::Unsupported => unreachable!(),
        TerminalKind::Supported => Ok(xterm::query(&mut tty, query, xterm::MAX_TIMEOUT)?.0),
        TerminalKind::Passthrough(_) => todo!(),
        TerminalKind::Unknown => {
            let timeout = estimate_timeout(&mut tty)?;
            Ok(xterm::query(&mut tty, query, timeout)?.0)
        }
    })
}

// TODO: Re-use already opened tty
fn tty() -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open("/dev/tty")
}
