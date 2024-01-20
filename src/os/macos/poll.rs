use crate::{Error, Result};
use libc::{c_int, timespec, FD_ISSET};
use std::io;
use std::time::Duration;
use std::{mem, ptr};
use terminal_trx::Transceive;

// macOS does not support polling /dev/tty using kqueue, so we have to
// resort to pselect/select. See https://nathancraddock.com/blog/macos-dev-tty-polling/.
pub(crate) fn poll_read(terminal: &dyn Transceive, timeout: Duration) -> Result<()> {
    let fd = terminal.as_raw_fd();
    let mut readfds = unsafe { std::mem::zeroed() };
    let timespec = to_timespec(timeout);
    unsafe { libc::FD_SET(fd, &mut readfds) };
    to_io_result(unsafe {
        libc::pselect(
            fd + 1,
            &mut readfds,
            ptr::null_mut(),
            ptr::null_mut(),
            &timespec,
            ptr::null(),
        )
    })?;
    if unsafe { FD_ISSET(fd, &readfds) } {
        Ok(())
    } else {
        Err(Error::Timeout(timeout))
    }
}

fn to_timespec(duration: Duration) -> timespec {
    let mut ts: timespec = unsafe { mem::zeroed() };
    ts.tv_sec = duration.as_secs() as libc::time_t;
    #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
    {
        ts.tv_nsec = duration.subsec_nanos() as i64;
    }
    #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
    {
        ts.tv_nsec = duration.subsec_nanos() as libc::c_long;
    }
    ts
}

pub(super) fn to_io_result(value: c_int) -> io::Result<c_int> {
    if value == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(value)
    }
}
