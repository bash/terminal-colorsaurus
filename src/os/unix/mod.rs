use libc::c_int;
use std::io;

mod tty;
pub(crate) use tty::*;
mod raw_tty;
pub(crate) use raw_tty::*;
#[cfg(not(target_os = "macos"))]
mod poll;
#[cfg(not(target_os = "macos"))]
pub(crate) use poll::*;

pub(super) fn to_io_result(value: c_int) -> io::Result<c_int> {
    if value == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(value)
    }
}
