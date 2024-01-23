use self::io_utils::{read_until2, TermReader};
use crate::{Color, ColorScheme, Error, QueryOptions, Result};
use std::io::{self, BufRead, BufReader, Write as _};
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
        options.max_timeout,
        |w| write!(w, "{QUERY_FG}"),
        |r| read_color_response(r),
    )?;
    parse_response(response, FG_RESPONSE_PREFIX)
}

#[allow(clippy::redundant_closure)]
pub(crate) fn background_color(options: QueryOptions) -> Result<Color> {
    let response = query(
        options.max_timeout,
        |w| write!(w, "{QUERY_BG}"),
        |r| read_color_response(r),
    )?;
    parse_response(response, BG_RESPONSE_PREFIX)
}

pub(crate) fn color_scheme(options: QueryOptions) -> Result<ColorScheme> {
    let (fg_response, bg_response) = query(
        options.max_timeout,
        |w| write!(w, "{QUERY_FG}{QUERY_BG}"),
        |r| Ok((read_color_response(r)?, read_color_response(r)?)),
    )?;
    let foreground = parse_response(fg_response, FG_RESPONSE_PREFIX)?;
    let background = parse_response(bg_response, BG_RESPONSE_PREFIX)?;
    Ok(ColorScheme {
        foreground,
        background,
    })
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

fn query<T>(
    timeout: Duration,
    write_query: impl FnOnce(&mut dyn io::Write) -> io::Result<()>,
    read_response: impl FnOnce(&mut BufReader<TermReader<RawModeGuard<'_>>>) -> Result<T>,
) -> Result<T> {
    let mut tty = terminal()?;
    let mut tty = tty.lock()?;
    let mut tty = tty.enable_raw_mode()?;

    write_query(&mut tty)?;
    write!(tty, "{DA1}")?;
    tty.flush()?;

    let mut reader = BufReader::with_capacity(32, TermReader::new(tty, timeout));

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
