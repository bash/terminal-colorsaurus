use crate::os::poll_read;
use std::io::{self, BufRead};
use std::os::fd::AsFd;
use std::time::{Duration, Instant};

// Copied from the standard library with modification
// to support searching for two bytes.
// https://github.com/rust-lang/rust/blob/e35a56d96f7d9d4422f2b7b00bf0bf282b2ec782/library/std/src/io/mod.rs#L2067
pub(super) fn read_until2<R: BufRead + ?Sized>(
    r: &mut R,
    delim1: u8,
    delim2: u8,
    buf: &mut Vec<u8>,
) -> io::Result<usize> {
    let mut read = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            match memchr::memchr2(delim1, delim2, available) {
                Some(i) => {
                    buf.extend_from_slice(&available[..=i]);
                    (true, i + 1)
                }
                None => {
                    buf.extend_from_slice(available);
                    (false, available.len())
                }
            }
        };
        r.consume(used);
        read += used;
        if done || used == 0 {
            return Ok(read);
        }
    }
}

#[derive(Debug)]
pub(super) struct TermReader<R> {
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
    pub(super) fn new(inner: R, timeout: Duration) -> Self {
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
