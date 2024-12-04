use std::str::from_utf8;

/// Parses a subset of X11 [Color Strings](https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Color_Strings).
/// The main goal is supporting all the formats that terminals emit as a response to `OSC` queries (e.g. `OSC 11 ; ? ST`).
///
/// ## Accepted Formats:
/// * `#<red><green><blue>`
/// * `rgb:<red>/<green>/<blue>`
/// * `rgba:<red>/<green>/<blue>/<alpha>` (rxvt-unicode extension)
pub fn xparsecolor(input: &[u8]) -> Option<Rgba> {
    if let Some(stripped) = input.strip_prefix(b"#") {
        parse_sharp(from_utf8(stripped).ok()?)
    } else if let Some(stripped) = input.strip_prefix(b"rgb:") {
        parse_rgb(from_utf8(stripped).ok()?)
    } else if let Some(stripped) = input.strip_prefix(b"rgba:") {
        parse_rgba(from_utf8(stripped).ok()?)
    } else {
        None
    }
}

/// An RGB color with 16 bits per channel.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct Rgba {
    /// The red channel. [0, 0xffff]
    pub red: u16,
    /// The green channel. [0, 0xffff]
    pub green: u16,
    /// The blue channel. [0, 0xffff]
    pub blue: u16,
    /// The alpha channel. [0, 0xffff]
    pub alpha: u16,
}

/// From the `xparsecolor` man page:
/// > For backward compatibility, an older syntax for RGB Device is supported,
/// > but its continued use is not encouraged. The syntax is an initial sharp sign character
/// > followed by a numeric specification, in one of the following formats:
/// >
/// > The R, G, and B represent single hexadecimal digits.
/// > When fewer than 16 bits each are specified, they represent the most significant bits of the value
/// > (unlike the `rgb:` syntax, in which values are scaled).
/// > For example, the string `#3a7` is the same as `#3000a0007000`.
fn parse_sharp(input: &str) -> Option<Rgba> {
    const NUM_COMPONENTS: usize = 3;
    let len = input.len();
    if len % NUM_COMPONENTS == 0 && len <= NUM_COMPONENTS * 4 {
        let chunk_size = input.len() / NUM_COMPONENTS;
        let red = parse_channel_shifted(&input[0..chunk_size])?;
        let green = parse_channel_shifted(&input[chunk_size..chunk_size * 2])?;
        let blue = parse_channel_shifted(&input[chunk_size * 2..])?;
        Some(Rgba {
            red,
            green,
            blue,
            alpha: u16::MAX,
        })
    } else {
        None
    }
}

fn parse_channel_shifted(input: &str) -> Option<u16> {
    let value = u16::from_str_radix(input, 16).ok()?;
    Some(value << ((4 - input.len()) * 4))
}

/// From the `xparsecolor` man page:
/// > An RGB Device specification is identified by the prefix `rgb:` and conforms to the following syntax:
/// > ```text
/// > rgb:<red>/<green>/<blue>
/// >
/// >     <red>, <green>, <blue> := h | hh | hhh | hhhh
/// >     h := single hexadecimal digits (case insignificant)
/// > ```
/// > Note that *h* indicates the value scaled in 4 bits,
/// > *hh* the value scaled in 8 bits, *hhh* the value scaled in 12 bits,
/// > and *hhhh* the value scaled in 16 bits, respectively.
fn parse_rgb(input: &str) -> Option<Rgba> {
    let mut parts = input.split('/');
    let red = parse_channel_scaled(parts.next()?)?;
    let green = parse_channel_scaled(parts.next()?)?;
    let blue = parse_channel_scaled(parts.next()?)?;
    if parts.next().is_none() {
        Some(Rgba {
            red,
            green,
            blue,
            alpha: u16::MAX,
        })
    } else {
        None
    }
}

/// Some terminals such as urxvt (rxvt-unicode) optionally support
/// an alpha channel and sometimes return colors in the format `rgba:<red>/<green>/<blue>/<alpha>`.
///
/// Dropping the alpha channel is a best-effort thing as
/// the effective color (when combined with a background color)
/// could have a completely different perceived lightness value.
///
/// Test with `urxvt -depth 32 -fg grey90 -bg rgba:0000/0000/4444/cccc`
fn parse_rgba(input: &str) -> Option<Rgba> {
    let mut parts = input.split('/');
    let red = parse_channel_scaled(parts.next()?)?;
    let green = parse_channel_scaled(parts.next()?)?;
    let blue = parse_channel_scaled(parts.next()?)?;
    let alpha = parse_channel_scaled(parts.next()?)?;
    if parts.next().is_none() {
        Some(Rgba {
            red,
            green,
            blue,
            alpha,
        })
    } else {
        None
    }
}

fn parse_channel_scaled(input: &str) -> Option<u16> {
    let len = input.len();
    if (1..=4).contains(&len) {
        let max = u32::pow(16, len as u32) - 1;
        let value = u32::from_str_radix(input, 16).ok()?;
        Some((u16::MAX as u32 * value / max) as u16)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests adapted from alacritty/vte:
    // https://github.com/alacritty/vte/blob/ed51aa19b7ad060f62a75ec55ebb802ced850b1a/src/ansi.rs#L2134
    #[test]
    fn parses_valid_rgb_color() {
        assert_eq!(
            xparsecolor(b"rgb:f/e/d"),
            Some(Rgba {
                red: 0xffff,
                green: 0xeeee,
                blue: 0xdddd,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"rgb:11/aa/ff"),
            Some(Rgba {
                red: 0x1111,
                green: 0xaaaa,
                blue: 0xffff,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"rgb:f/ed1/cb23"),
            Some(Rgba {
                red: 0xffff,
                green: 0xed1d,
                blue: 0xcb23,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"rgb:ffff/0/0"),
            Some(Rgba {
                red: 0xffff,
                green: 0x0,
                blue: 0x0,
                alpha: 0xffff,
            })
        );
    }

    #[test]
    fn parses_valid_rgba_color() {
        assert_eq!(
            xparsecolor(b"rgba:0000/0000/4443/cccc"),
            Some(Rgba {
                red: 0x0000,
                green: 0x0000,
                blue: 0x4443,
                alpha: 0xcccc,
            })
        );
    }

    #[test]
    fn fails_for_invalid_rgb_color() {
        assert!(xparsecolor(b"rgb:").is_none()); // Empty
        assert!(xparsecolor(b"rgb:f/f").is_none()); // Not enough channels
        assert!(xparsecolor(b"rgb:f/f/f/f").is_none()); // Too many channels
        assert!(xparsecolor(b"rgb:f//f").is_none()); // Empty channel
        assert!(xparsecolor(b"rgb:ffff/ffff/fffff").is_none()); // Too many digits for one channel
    }

    // Tests adapted from alacritty/vte:
    // https://github.com/alacritty/vte/blob/ed51aa19b7ad060f62a75ec55ebb802ced850b1a/src/ansi.rs#L2142
    #[test]
    fn parses_valid_sharp_color() {
        assert_eq!(
            xparsecolor(b"#1af"),
            Some(Rgba {
                red: 0x1000,
                green: 0xa000,
                blue: 0xf000,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"#1AF"),
            Some(Rgba {
                red: 0x1000,
                green: 0xa000,
                blue: 0xf000,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"#11aaff"),
            Some(Rgba {
                red: 0x1100,
                green: 0xaa00,
                blue: 0xff00,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"#110aa0ff0"),
            Some(Rgba {
                red: 0x1100,
                green: 0xaa00,
                blue: 0xff00,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"#1100aa00ff00"),
            Some(Rgba {
                red: 0x1100,
                green: 0xaa00,
                blue: 0xff00,
                alpha: 0xffff,
            })
        );
        assert_eq!(
            xparsecolor(b"#123456789ABC"),
            Some(Rgba {
                red: 0x1234,
                green: 0x5678,
                blue: 0x9ABC,
                alpha: 0xffff,
            })
        );
    }

    #[test]
    fn fails_for_invalid_sharp_color() {
        assert!(xparsecolor(b"#").is_none()); // Empty
        assert!(xparsecolor(b"#1234").is_none()); // Not divisible by three
        assert!(xparsecolor(b"#123456789ABCDEF").is_none()); // Too many components
    }
}
