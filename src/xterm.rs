use crate::os::poll_read;
use crate::terminal::TerminalKind;
use crate::{Color, Error, QueryOptions, Result};
use std::cmp::{max, min};
use std::str::from_utf8;
use std::time::{Duration, Instant};
use terminal_trx::{terminal, Transceive};

const MIN_TIMEOUT: Duration = Duration::from_millis(100);

pub(crate) fn foreground_color(options: QueryOptions) -> Result<Color> {
    query_color(
        "\x1b]10;?\x07",
        "\x1b]10;",
        options,
        TerminalKind::from_env(),
    )
}

pub(crate) fn background_color(options: QueryOptions) -> Result<Color> {
    query_color(
        "\x1b]11;?\x07",
        "\x1b]11;",
        options,
        TerminalKind::from_env(),
    )
}

fn query_color(
    query: &str,
    response_prefix: &str,
    options: QueryOptions,
    terminal: TerminalKind,
) -> Result<Color> {
    query_color_raw(query, options, terminal).and_then(|r| parse_response(r, response_prefix))
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

fn query_color_raw(q: &str, options: QueryOptions, kind: TerminalKind) -> Result<String> {
    if let TerminalKind::Unsupported = kind {
        return Err(Error::UnsupportedTerminal);
    }

    let mut tty = terminal()?;
    let mut tty = tty.lock()?;
    let mut tty = tty.enable_raw_mode()?;

    match kind {
        TerminalKind::Unsupported => unreachable!(),
        TerminalKind::Supported => Ok(query(&mut tty, q, options.max_timeout)?.0),
        TerminalKind::Unknown => {
            // We use a well-supported sequence such as CSI C to measure the latency.
            // this is to avoid mixing up the case where the terminal is slow to respond
            // (e.g. because we're connected via SSH and have a slow connection)
            // with the case where the terminal does not support querying for colors.
            let timeout = estimate_timeout(&mut tty, options.max_timeout)?;
            Ok(query(&mut tty, q, timeout)?.0)
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
    poll_read(tty.as_raw_fd(), timeout)?;
    let bytes_read = tty.read(&mut buffer)?;
    let duration = start.elapsed();

    let response = from_utf8(&buffer[..bytes_read])?.to_owned();

    Ok((response, duration))
}
