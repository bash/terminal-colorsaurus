use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use std::io::{self};
use std::os::fd::RawFd;
use std::time::Duration;

pub(crate) fn poll_read(fd: RawFd, timeout: Duration) -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1024);
    let token = Token(0);
    poll.registry()
        .register(&mut SourceFd(&fd), token, Interest::READABLE)?;
    poll.poll(&mut events, Some(timeout))?;
    for event in &events {
        if event.token() == token {
            return Ok(());
        }
    }
    todo!("timeout")
}
