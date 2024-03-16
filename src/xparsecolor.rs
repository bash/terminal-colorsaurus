use crate::Color;

/// Parses a color value that follows the `XParseColor` format.
/// See https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Color_Strings
/// for a reference of what `XParseColor` supports.
///
/// Not all formats are supported, just the ones that are returned
/// by the tested terminals. Feel free to open a PR if you encounter
/// a terminal that returns a different format.
pub(crate) fn xparsecolor(input: &str) -> Option<Color> {
    if let Some(stripped) = input.strip_prefix('#') {
        parse_sharp(stripped)
    } else if let Some(stripped) = input.strip_prefix("rgb:") {
        parse_rgb(stripped)
    } else {
        None
    }
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
fn parse_sharp(input: &str) -> Option<Color> {
    const NUM_COMPONENTS: usize = 3;
    let len = input.len();
    if len % NUM_COMPONENTS == 0 && len <= NUM_COMPONENTS * 4 {
        let chunk_size = input.len() / NUM_COMPONENTS;
        let r = parse_channel_shifted(&input[0..chunk_size])?;
        let g = parse_channel_shifted(&input[chunk_size..chunk_size * 2])?;
        let b = parse_channel_shifted(&input[chunk_size * 2..])?;
        Some(Color { r, g, b })
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
fn parse_rgb(input: &str) -> Option<Color> {
    let mut parts = input.split('/');
    let r = parse_channel_scaled(parts.next()?)?;
    let g = parse_channel_scaled(parts.next()?)?;
    let b = parse_channel_scaled(parts.next()?)?;
    if parts.next().is_none() {
        Some(Color { r, g, b })
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
            xparsecolor("rgb:f/e/d"),
            Some(Color {
                r: 0xffff,
                g: 0xeeee,
                b: 0xdddd,
            })
        );
        assert_eq!(
            xparsecolor("rgb:11/aa/ff"),
            Some(Color {
                r: 0x1111,
                g: 0xaaaa,
                b: 0xffff
            })
        );
        assert_eq!(
            xparsecolor("rgb:f/ed1/cb23"),
            Some(Color {
                r: 0xffff,
                g: 0xed1d,
                b: 0xcb23,
            })
        );
        assert_eq!(
            xparsecolor("rgb:ffff/0/0"),
            Some(Color {
                r: 0xffff,
                g: 0x0,
                b: 0x0
            })
        );
    }

    #[test]
    fn fails_for_invalid_rgb_color() {
        assert!(xparsecolor("rgb:").is_none()); // Empty
        assert!(xparsecolor("rgb:f/f").is_none()); // Not enough channels
        assert!(xparsecolor("rgb:f/f/f/f").is_none()); // Too many channels
        assert!(xparsecolor("rgb:f//f").is_none()); // Empty channel
        assert!(xparsecolor("rgb:ffff/ffff/fffff").is_none()); // Too many digits for one channel
    }

    // Tests adapted from alacritty/vte:
    // https://github.com/alacritty/vte/blob/ed51aa19b7ad060f62a75ec55ebb802ced850b1a/src/ansi.rs#L2142
    #[test]
    fn parses_valid_sharp_color() {
        assert_eq!(
            xparsecolor("#1af"),
            Some(Color {
                r: 0x1000,
                g: 0xa000,
                b: 0xf000,
            })
        );
        assert_eq!(
            xparsecolor("#1AF"),
            Some(Color {
                r: 0x1000,
                g: 0xa000,
                b: 0xf000,
            })
        );
        assert_eq!(
            xparsecolor("#11aaff"),
            Some(Color {
                r: 0x1100,
                g: 0xaa00,
                b: 0xff00
            })
        );
        assert_eq!(
            xparsecolor("#110aa0ff0"),
            Some(Color {
                r: 0x1100,
                g: 0xaa00,
                b: 0xff00
            })
        );
        assert_eq!(
            xparsecolor("#1100aa00ff00"),
            Some(Color {
                r: 0x1100,
                g: 0xaa00,
                b: 0xff00
            })
        );
        assert_eq!(
            xparsecolor("#123456789ABC"),
            Some(Color {
                r: 0x1234,
                g: 0x5678,
                b: 0x9ABC
            })
        );
    }

    #[test]
    fn fails_for_invalid_sharp_color() {
        assert!(xparsecolor("#").is_none()); // Empty
        assert!(xparsecolor("#1234").is_none()); // Not divisible by three
        assert!(xparsecolor("#123456789ABCDEF").is_none()); // Too many components
    }
}
