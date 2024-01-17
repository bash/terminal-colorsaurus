use std::fs::{File, OpenOptions};
use std::io;
use std::io::{stderr, stdin, stdout, StderrLock, StdinLock, StdoutLock};
use std::mem::ManuallyDrop;
use std::os::fd::{AsRawFd, FromRawFd, RawFd};

macro_rules! try_tty {
    ($name:ident ($fd:ident, $fn:expr)) => {
        if isatty(libc::$fd) {
            // Stderr, stdout and stdin are actually bidirectional if
            // they're a tty, but Rust's built-in types don't support that, so we
            // wrap it in a file.
            return Ok(Tty::Borrowed(TtyLock::$name($fn().lock()), unsafe {
                ManuallyDrop::new(File::from_raw_fd(libc::$fd))
            }));
        }
    };
}

/// Obtains a handle on the TTY.
/// We try to find an already open tty in the same order as `tput` (See `man tput`).
pub(crate) fn tty() -> io::Result<Tty> {
    try_tty!(Stderr(STDERR_FILENO, stderr));
    try_tty!(Stdout(STDOUT_FILENO, stdout));
    try_tty!(Stdin(STDIN_FILENO, stdin));

    Ok(Tty::Owned(
        OpenOptions::new().read(true).write(true).open("/dev/tty")?,
    ))
}

fn isatty(fd: RawFd) -> bool {
    unsafe { libc::isatty(fd) == 1 }
}

#[derive(Debug)]
pub(crate) enum Tty {
    Owned(File),
    // We're using ManuallyDrop here because we don't want
    // to close stdin / stdout / stderr once we're done.
    Borrowed(TtyLock, ManuallyDrop<File>),
}

#[derive(Debug)]
pub(crate) enum TtyLock {
    Stdin(StdinLock<'static>),
    Stdout(StdoutLock<'static>),
    Stderr(StderrLock<'static>),
}

impl io::Write for Tty {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.as_file_mut().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.as_file_mut().flush()
    }
}

impl io::Read for Tty {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.as_file_mut().read(buf)
    }
}

impl Tty {
    fn as_file_mut(&mut self) -> &mut File {
        match self {
            Tty::Owned(f) => f,
            Tty::Borrowed(_, f) => f,
        }
    }
}

impl AsRawFd for Tty {
    fn as_raw_fd(&self) -> RawFd {
        match self {
            Tty::Owned(f) => f.as_raw_fd(),
            Tty::Borrowed(_, f) => f.as_raw_fd(),
        }
    }
}
