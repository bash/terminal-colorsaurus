use crate::{Error, Result};
use mio::unix::SourceFd;
use mio::{Events, Interest, Poll, Token};
use std::time::Duration;
use terminal_trx::Transceive;

pub(crate) fn poll_read(terminal: &dyn Transceive, timeout: Duration) -> Result<()> {
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
    Err(Error::Timeout(timeout))
}
