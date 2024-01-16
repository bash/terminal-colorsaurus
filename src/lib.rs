use os::poll_read;
use os::run_in_raw_mode;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write as _};
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::xterm::query;

mod os;
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
    Unsupported,
}

pub fn background_color() -> Result<String> {
    let tty = tty()?;
    let mut tty_clone = tty.try_clone()?;
    let result = run_in_raw_mode(&tty, move || {
        query(&mut tty_clone, "\x1b[c", Duration::from_millis(300))
    })?;
    dbg!(result);
    Ok("".into())
}

// TODO: Re-use already opened tty
fn tty() -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open("/dev/tty")
}

// fn measure_latency(tty: &mut File) -> io::Result<Duration> {
//     let mut buffer: [u8; 1] = Default::default();

//     write!(tty, "\x1b[c")?;
//     tty.flush()?;
//     let start = Instant::now();
//     poll_read(libc::STDIN_FILENO, Duration::from_millis(300))?;
//     tty.read_exact(&mut buffer)?;
//     let duration = start.elapsed();

//     if buffer[0] != b'\x1b' {
//         todo!("Unexpected response: {:X}", buffer[0])
//     }

//     // We don't care about the reponse, drop everything until
//     // we read the terminating 'c'
//     while buffer[0] != b'c' {
//         tty.read_exact(&mut buffer)?;
//     }

//     Ok(duration)
// }
