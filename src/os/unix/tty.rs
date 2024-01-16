use super::to_io_result;
use crate::Result;
use libc::termios;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd};
use std::panic::{catch_unwind, panic_any, UnwindSafe};
use std::{io, mem};

pub(crate) fn run_in_raw_mode<F, T>(fd: impl AsFd, f: F) -> Result<T>
where
    F: FnOnce() -> Result<T> + UnwindSafe,
{
    let fd = fd.as_fd();

    let old_terminal = get_terminal_attr(fd)?;
    let mut terminal = old_terminal;
    raw_terminal_attr(&mut terminal);
    set_terminal_attr(fd, &terminal)?;

    let panic_result = catch_unwind(f);
    let _restore_result = set_terminal_attr(fd, &old_terminal); // TODO: log error as warning maybe?
    panic_result.unwrap_or_else(|e| panic_any(e))
}

fn get_terminal_attr(fd: BorrowedFd) -> io::Result<termios> {
    unsafe {
        let mut termios = mem::zeroed();
        to_io_result(libc::tcgetattr(fd.as_raw_fd(), &mut termios))?;
        Ok(termios)
    }
}

fn set_terminal_attr(fd: BorrowedFd, termios: &termios) -> io::Result<()> {
    // From the man page:
    // TCSADRAIN
    //     the change occurs after all output written to fd has been transmitted.
    //     This function should be used when changing parameters that affect output.
    to_io_result(unsafe { libc::tcsetattr(fd.as_raw_fd(), libc::TCSADRAIN, termios) }).and(Ok(()))
}

fn raw_terminal_attr(termios: &mut termios) {
    // We disable two flags:
    // ECHO
    //     to disable input characters from being echoed.
    // ICANON
    //     to disable canonical mode (we want input to be available immediately and not wait for a line terminator).
    termios.c_lflag &= !(libc::ICANON | libc::ECHO);
}
