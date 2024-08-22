use super::poll_read;
use std::io;
use std::os::fd::AsFd;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub(crate) struct TermReader<R> {
    inner: R,
    timeout: Duration,
    first_read: Option<Instant>,
}

impl<R> io::Read for TermReader<R>
where
    R: io::Read + AsFd,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let timeout = self.remaining_timeout();
        poll_read(self.inner.as_fd(), timeout)?;
        self.inner.read(buf)
    }
}

impl<R> TermReader<R> {
    pub(crate) fn new(inner: R, timeout: Duration) -> Self {
        Self {
            inner,
            timeout,
            first_read: None,
        }
    }

    fn remaining_timeout(&mut self) -> Duration {
        let first_read = self.first_read.get_or_insert_with(Instant::now);
        self.timeout.saturating_sub(first_read.elapsed())
    }
}
