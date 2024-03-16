/// An RGB color with 16 bits per channel.
/// You can use [`Color::scale_to_8bit`] to convert to an 8bit RGB color.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct Color {
    /// Red
    pub r: u16,
    /// Green
    pub g: u16,
    /// Blue
    pub b: u16,
}

impl Color {
    /// The perceived lightness of the color
    /// as a value between `0` (black) and `100` (white)
    /// where `50` is the perceptual "middle grey".
    /// ```
    /// # use terminal_colorsaurus::Color;
    /// # let color = Color::default();
    /// let is_dark = color.perceived_lightness() <= 50;
    /// ```
    pub fn perceived_lightness(&self) -> u8 {
        luminance_to_perceived_lightness(luminance(self))
    }

    /// Converts the color to 8 bit precision per channel by scaling each channel.
    ///
    /// ```
    /// # use terminal_colorsaurus::Color;
    /// let white = Color { r: u16::MAX, g: u16::MAX, b: u16::MAX };
    /// assert_eq!((u8::MAX, u8::MAX, u8::MAX), white.scale_to_8bit());
    ///
    /// let black = Color { r: 0, g: 0, b: 0 };
    /// assert_eq!((0, 0, 0), black.scale_to_8bit());
    /// ```
    pub fn scale_to_8bit(&self) -> (u8, u8, u8) {
        (
            scale_to_u8(self.r),
            scale_to_u8(self.g),
            scale_to_u8(self.b),
        )
    }
}

fn scale_to_u8(channel: u16) -> u8 {
    (channel as u32 * (u8::MAX as u32) / (u16::MAX as u32)) as u8
}

#[cfg(feature = "rgb")]
impl From<Color> for rgb::RGB16 {
    fn from(value: Color) -> Self {
        rgb::RGB16 {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

#[cfg(feature = "rgb")]
impl From<rgb::RGB16> for Color {
    fn from(value: rgb::RGB16) -> Self {
        Color {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

// Implementation of determining the perceived lightness
// follows this excellent answer: https://stackoverflow.com/a/56678483

fn srgb_to_lin(channel: f64) -> f64 {
    if channel < 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}

// Luminance (Y)
fn luminance(color: &Color) -> f64 {
    let r = f64::from(color.r) / f64::from(u16::MAX);
    let g = f64::from(color.g) / f64::from(u16::MAX);
    let b = f64::from(color.b) / f64::from(u16::MAX);
    0.2126 * srgb_to_lin(r) + 0.7152 * srgb_to_lin(g) + 0.0722 * srgb_to_lin(b)
}

// Perceptual lightness (L*)
fn luminance_to_perceived_lightness(luminance: f64) -> u8 {
    if luminance < 216. / 24389. {
        (luminance * (24389. / 27.)) as u8
    } else {
        (luminance.powf(1. / 3.) * 116. - 16.) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_has_perceived_lightness_zero() {
        let black = Color::default();
        assert_eq!(0, black.perceived_lightness())
    }

    #[test]
    fn white_has_perceived_lightness_100() {
        let white = Color {
            r: u16::MAX,
            g: u16::MAX,
            b: u16::MAX,
        };
        assert_eq!(100, white.perceived_lightness())
    }
}
