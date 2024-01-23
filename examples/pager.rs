use std::error::Error;
use std::io::stdout;
use std::{io, mem};
use terminal_colorsaurus::{color_scheme, QueryOptions};

/// This example is best in a couple of different ways:
/// 1. `cargo run --example pager`—should print the color scheme.
/// 2. `cargo run --example pager | less`—should not print the color scheme.
/// 3. `cargo run --example pager > file.txt`—should print the color scheme.
/// 4. `cargo run --example pager > /dev/null`—should print the color scheme.
fn main() -> Result<(), Box<dyn Error>> {
    if should_auto_detect() {
        eprintln!(
            "Here's the color scheme: {:#?}",
            color_scheme(QueryOptions::default())?
        );
    } else {
        eprintln!("You're likely using a pager, doing nothing");
    }
    Ok(())
}

// Our stdout might be piped to a pager (e.g. `less`),
// in which case we have a race condition for enabling/disabling the raw mode
// and for reading/writing to the terminal.
//
// Note that this is just heuristic with both
// false negatives (output not piped to a pager) and
// false positives (stderr piped to a pager).
#[cfg(unix)]
fn should_auto_detect() -> bool {
    use std::os::fd::AsFd;
    !is_pipe(stdout().as_fd()).unwrap_or_default()
}

#[cfg(not(unix))]
fn should_auto_detect() -> bool {
    true
}

// The mode can be bitwise AND-ed with S_IFMT to extract the file type code, and compared to the appropriate constant
// Source: https://www.gnu.org/software/libc/manual/html_node/Testing-File-Type.html
#[cfg(unix)]
fn is_pipe(fd: std::os::fd::BorrowedFd) -> io::Result<bool> {
    use libc::{S_IFIFO, S_IFMT};
    Ok(fstat(fd)?.st_mode & S_IFMT == S_IFIFO)
}

#[cfg(unix)]
fn fstat(fd: std::os::fd::BorrowedFd) -> io::Result<libc::stat> {
    use std::os::fd::AsRawFd as _;
    // SAFETY:
    // 1. File descriptor is valid (we have a borrowed fd for the lifetime of this function)
    // 2. fstat64 fills the stat structure for us (if successful).
    unsafe {
        let mut stat = mem::zeroed();
        let ret = libc::fstat(fd.as_raw_fd(), &mut stat);
        if ret == 0 {
            Ok(stat)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}
