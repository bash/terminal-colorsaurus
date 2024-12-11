use crate::xparsecolor::xparsecolor;
use crate::{Color, Error, Result};

const FG_RESPONSE_PREFIX: &[u8] = b"\x1b]10;";
const BG_RESPONSE_PREFIX: &[u8] = b"\x1b]11;";

const ST: &[u8] = b"\x1b\\";
const BEL: u8 = 0x07;

/// Parses a response to an `OSC 10` (foreground color) query.
///
/// The response may be terminated with either `ST` or `BEL`.
pub fn parse_fg_color_response(response: &[u8]) -> Result<Color> {
    parse_response(response, FG_RESPONSE_PREFIX)
}

/// Parses a response to an `OSC 11` (background color) query.
///
/// The response may be terminated with either `ST` or `BEL`.
pub fn parse_bg_color_response(response: &[u8]) -> Result<Color> {
    parse_response(response, BG_RESPONSE_PREFIX)
}

fn parse_response(response: &[u8], prefix: &[u8]) -> Result<Color> {
    response
        .strip_prefix(prefix)
        .and_then(|r| r.strip_suffix(ST).or(r.strip_suffix(&[BEL])))
        .and_then(xparsecolor)
        .ok_or_else(|| Error::Parse(response.to_owned()))
}
