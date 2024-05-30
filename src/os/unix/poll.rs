use crate::os::unix_common::timed_out;
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use std::io;
use std::os::fd::{AsRawFd as _, BorrowedFd};
use std::time::Duration;

pub(crate) fn poll_read(terminal: BorrowedFd, timeout: Duration) -> io::Result<()> {
    if timeout.is_zero() {
        return Err(timed_out());
    }

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let token = Token(0);
    poll.registry().register(
        &mut SourceFd(&terminal.as_raw_fd()),
        token,
        Interest::READABLE,
    )?;
    poll.poll(&mut events, Some(timeout))?;
    for event in &events {
        if event.token() == token {
            return Ok(());
        }
    }
    Err(timed_out())
}
