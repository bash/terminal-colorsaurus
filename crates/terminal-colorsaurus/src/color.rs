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
        (self.perceived_lightness_f32() * 100.) as u8
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

    pub(crate) fn perceived_lightness_f32(&self) -> f32 {
        let color = xterm_color::Color::rgb(self.r, self.g, self.b);
        color.perceived_lightness()
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
impl From<Color> for rgb::RGB8 {
    fn from(value: Color) -> Self {
        let (r, g, b) = value.scale_to_8bit();
        rgb::RGB8 { r, g, b }
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

#[cfg(feature = "anstyle")]
impl From<Color> for anstyle::RgbColor {
    fn from(value: Color) -> Self {
        let (r, g, b) = value.scale_to_8bit();
        anstyle::RgbColor(r, g, b)
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
