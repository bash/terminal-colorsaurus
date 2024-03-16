use crate::Color;

impl Color {
    /// Parses an X11 color (see `man xparsecolor`).
    pub(crate) fn parse_x11(input: &str) -> Option<Self> {
        let raw_parts = input.strip_prefix("rgb:")?;
        let mut parts = raw_parts.split('/');
        let r = parse_channel(parts.next()?)?;
        let g = parse_channel(parts.next()?)?;
        let b = parse_channel(parts.next()?)?;
        Some(Color { r, g, b })
    }

    // Some terminals (only Terminology found so far) respond with a
    // CSS-like hex color code.
    pub(crate) fn parse_css_like(input: &str) -> Option<Self> {
        let raw_parts = input.strip_prefix('#')?;
        let len = raw_parts.len();
        if len == 6 {
            let r = parse_channel(&raw_parts[..2])?;
            let g = parse_channel(&raw_parts[2..4])?;
            let b = parse_channel(&raw_parts[4..])?;
            Some(Color { r, g, b })
        } else {
            None
        }
    }
}

#[cfg(unix)]
fn parse_channel(input: &str) -> Option<u16> {
    let len = input.len();
    // From the xparsecolor man page:
    //   h indicates the value scaled in 4 bits,
    //   hh the value scaled in 8 bits,
    //   hhh the value scaled in 12 bits, and
    //   hhhh the value scaled in 16 bits, respectively.
    let shift = (1..=4).contains(&len).then_some(16 - 4 * len as u16)?;
    Some(u16::from_str_radix(input, 16).ok()? << shift)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn parses_css_like_color() {
        assert_eq!(
            Color {
                r: 171 << 8,
                g: 205 << 8,
                b: 239 << 8
            },
            Color::parse_css_like("#ABCDEF").unwrap()
        )
    }
}
