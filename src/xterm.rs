use self::io_utils::{read_until2, TermReader};
use crate::{Color, ColorScheme, Error, Preconditions, QueryOptions, Result};
use std::env;
use std::io::{self, stdout, BufRead, BufReader, Write as _};
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd};
use std::str::from_utf8;
use std::time::Duration;
use terminal_trx::{terminal, RawModeGuard};

mod io_utils;

const QUERY_FG: &str = "\x1b]10;?\x07";
const FG_RESPONSE_PREFIX: &str = "\x1b]10;";
const QUERY_BG: &str = "\x1b]11;?\x07";
const BG_RESPONSE_PREFIX: &str = "\x1b]11;";

#[allow(clippy::redundant_closure)]
pub(crate) fn foreground_color(options: QueryOptions) -> Result<Color> {
    let response = query(
        &options,
        |w| write!(w, "{QUERY_FG}"),
        |r| read_color_response(r),
    )
    .map_err(map_timed_out_err(options.timeout))?;
    parse_response(response, FG_RESPONSE_PREFIX)
}

#[allow(clippy::redundant_closure)]
pub(crate) fn background_color(options: QueryOptions) -> Result<Color> {
    let response: String = query(
        &options,
        |w| write!(w, "{QUERY_BG}"),
        |r| read_color_response(r),
    )
    .map_err(map_timed_out_err(options.timeout))?;
    parse_response(response, BG_RESPONSE_PREFIX)
}

pub(crate) fn color_scheme(options: QueryOptions) -> Result<ColorScheme> {
    let (fg_response, bg_response) = query(
        &options,
        |w| write!(w, "{QUERY_FG}{QUERY_BG}"),
        |r| Ok((read_color_response(r)?, read_color_response(r)?)),
    )
    .map_err(map_timed_out_err(options.timeout))?;
    let foreground = parse_response(fg_response, FG_RESPONSE_PREFIX)?;
    let background = parse_response(bg_response, BG_RESPONSE_PREFIX)?;
    Ok(ColorScheme {
        foreground,
        background,
    })
}

fn map_timed_out_err(timeout: Duration) -> impl Fn(Error) -> Error {
    move |e| match e {
        Error::Io(io) if io.kind() == io::ErrorKind::TimedOut => Error::Timeout(timeout),
        e => e,
    }
}

// We don't want to send any escape sequences to
// terminals that don't support them.
fn ensure_capable_terminal() -> Result<()> {
    match env::var("TERM") {
        Ok(term) if term == "dumb" => Err(Error::UnsupportedTerminal),
        Ok(_) | Err(_) => Ok(()),
    }
}

fn ensure_preconditions(preconditions: &Preconditions) -> Result<()> {
    if preconditions.stdout_not_piped && is_pipe(stdout().as_fd()).unwrap_or_default() {
        Err(Error::UnmetPrecondition("stdout is not piped".to_string()))
    } else {
        Ok(())
    }
}

fn parse_response(response: String, prefix: &str) -> Result<Color> {
    response
        .strip_prefix(prefix)
        .and_then(|response| {
            response
                .strip_suffix('\x07')
                .or(response.strip_suffix("\x1b\\"))
        })
        .and_then(Color::parse_x11)
        .ok_or_else(|| Error::Parse(response))
}

// We detect terminals that don't support the color query in quite a smart way:
// First, we send the color query and then a query that we know is well-supported (DA1).
// Since queries are answered sequentially, if a terminal answers to DA1 first, we know that
// it does not support querying for colors.
//
// Source: https://gitlab.freedesktop.org/terminal-wg/specifications/-/issues/8#note_151381
fn query<T>(
    options: &QueryOptions,
    write_query: impl FnOnce(&mut dyn io::Write) -> io::Result<()>,
    read_response: impl FnOnce(&mut BufReader<TermReader<RawModeGuard<'_>>>) -> Result<T>,
) -> Result<T> {
    ensure_capable_terminal()?;
    ensure_preconditions(&options.preconditions)?;

    let mut tty = terminal()?;
    let mut tty = tty.lock();
    let mut tty = tty.enable_raw_mode()?;

    write_query(&mut tty)?;
    write!(tty, "{DA1}")?;
    tty.flush()?;

    let mut reader = BufReader::with_capacity(32, TermReader::new(tty, options.timeout));

    let response = read_response(&mut reader)?;

    // We still need to consume the reponse to DA1
    // Let's ignore errors, they are not that important.
    _ = consume_da1_response(&mut reader, true);

    Ok(response)
}

const ESC: u8 = b'\x1b';
const BEL: u8 = b'\x07';
const DA1: &str = "\x1b[c";

fn read_color_response<R: io::Read>(r: &mut BufReader<R>) -> Result<String> {
    let mut buf = Vec::new();
    r.read_until(ESC, &mut buf)?; // Both responses start with ESC

    // If we get the response for DA1 back first, then we know that
    // the terminal does not recocgnize the color query.
    if !r.buffer().starts_with(&[b']']) {
        _ = consume_da1_response(r, false);
        return Err(Error::UnsupportedTerminal);
    }

    // Some terminals like iTerm2 always respond with ST (= ESC \)
    read_until2(r, BEL, ESC, &mut buf)?;
    if buf.last() == Some(&ESC) {
        r.read_until(b'\\', &mut buf)?;
    }

    Ok(from_utf8(&buf)?.to_owned())
}

fn consume_da1_response(r: &mut impl BufRead, consume_esc: bool) -> io::Result<()> {
    let mut buf = Vec::new();
    if consume_esc {
        r.read_until(ESC, &mut buf)?;
    }
    r.read_until(b'[', &mut buf)?;
    r.read_until(b'c', &mut buf)?;
    Ok(())
}

// The mode can be bitwise AND-ed with S_IFMT to extract the file type code, and compared to the appropriate constant
// Source: https://www.gnu.org/software/libc/manual/html_node/Testing-File-Type.html
fn is_pipe(fd: BorrowedFd) -> std::io::Result<bool> {
    use libc::{S_IFIFO, S_IFMT};
    Ok(fstat(fd)?.st_mode & S_IFMT == S_IFIFO)
}

fn fstat(fd: BorrowedFd) -> std::io::Result<libc::stat> {
    // SAFETY:
    // 1. File descriptor is valid (we have a borrowed fd for the lifetime of this function)
    // 2. fstat64 fills the stat structure for us (if successful).
    unsafe {
        let mut stat = std::mem::zeroed();
        let ret = libc::fstat(fd.as_raw_fd(), &mut stat);
        if ret == 0 {
            Ok(stat)
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}
