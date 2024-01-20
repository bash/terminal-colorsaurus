use crate::os::poll_read;
use crate::terminal::TerminalKind;
use crate::{Color, ColorScheme, Error, QueryOptions, Result};
use std::cmp::{max, min};
use std::str::from_utf8;
use std::time::{Duration, Instant};
use terminal_trx::{terminal, Transceive};

const MIN_TIMEOUT: Duration = Duration::from_millis(100);

pub(crate) fn foreground_color(options: QueryOptions) -> Result<Color> {
    execute_query(options, TerminalKind::from_env(), query_foreground_color)
}

pub(crate) fn background_color(options: QueryOptions) -> Result<Color> {
    execute_query(options, TerminalKind::from_env(), query_background_color)
}

pub(crate) fn color_scheme(options: QueryOptions) -> Result<ColorScheme> {
    execute_query(options, TerminalKind::from_env(), |tty, timeout| {
        let foreground = query_foreground_color(tty, timeout)?;
        let background = query_background_color(tty, timeout)?;
        Ok(ColorScheme {
            foreground,
            background,
        })
    })
}

fn query_foreground_color(tty: &mut dyn Transceive, timeout: Duration) -> Result<Color> {
    query_color(tty, timeout, "\x1b]10;?\x07", "\x1b]10;")
}

fn query_background_color(tty: &mut dyn Transceive, timeout: Duration) -> Result<Color> {
    query_color(tty, timeout, "\x1b]11;?\x07", "\x1b]11;")
}

fn query_color(
    tty: &mut dyn Transceive,
    timeout: Duration,
    q: &str,
    response_prefix: &str,
) -> Result<Color> {
    query(tty, q, timeout).and_then(|(r, _)| parse_response(r, response_prefix))
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

fn execute_query<T>(
    options: QueryOptions,
    kind: TerminalKind,
    f: impl FnOnce(&mut dyn Transceive, Duration) -> Result<T>,
) -> Result<T> {
    if let TerminalKind::Unsupported = kind {
        return Err(Error::UnsupportedTerminal);
    }

    let mut tty = terminal()?;
    let mut tty = tty.lock()?;
    let mut tty = tty.enable_raw_mode()?;

    match kind {
        TerminalKind::Unsupported => unreachable!(),
        TerminalKind::Supported => f(&mut tty, options.max_timeout),
        TerminalKind::Unknown => {
            // We use a well-supported sequence such as CSI C to measure the latency.
            // this is to avoid mixing up the case where the terminal is slow to respond
            // (e.g. because we're connected via SSH and have a slow connection)
            // with the case where the terminal does not support querying for colors.
            let timeout = estimate_timeout(&mut tty, options.max_timeout)?;
            f(&mut tty, timeout)
        }
    }
}

fn estimate_timeout(tty: &mut dyn Transceive, max_timeout: Duration) -> Result<Duration> {
    let (_, latency) = query(tty, "\x1b[c", max_timeout)?;
    let timeout = latency * 2; // We want to be in the same ballpark as the latency of our test query. Factor 2 is mostly arbitrary.
    Ok(min(max(timeout, MIN_TIMEOUT), max_timeout))
}

fn query(tty: &mut dyn Transceive, query: &str, timeout: Duration) -> Result<(String, Duration)> {
    let mut buffer = vec![0; 100];

    write!(tty, "{}", query)?;
    tty.flush()?;

    let start = Instant::now();
    poll_read(tty, timeout)?;
    let bytes_read = tty.read(&mut buffer)?;
    let duration = start.elapsed();

    let response = from_utf8(&buffer[..bytes_read])?.to_owned();

    Ok((response, duration))
}
