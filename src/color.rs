use std::ops::*;

/// Color in RGB space.
///
/// Each colour should be between 0 and 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    /// Red proportion, as a decimal.
    pub r: f64,

    /// Green proportion, as a decimal.
    pub g: f64,

    /// Blue proportion, as a decimal.
    pub b: f64,
}

impl Color {
    /// New colour from floats between 0 and 1.
    pub fn new(r: f64, g: f64, b: f64) -> Self{
        Color { r: r.clamp(0.0, 1.0), g: g.clamp(0.0, 1.0), b: b.clamp(0.0, 1.0) }
    }

    /// New colour from floats between 0 and 1 multiplied by the emission value.
    pub fn new_emission(r: f64, g: f64, b: f64, a: f64) -> Self{
        Color { r: r.clamp(0.0, 1.0) * a, g: g.clamp(0.0, 1.0) * a, b: b.clamp(0.0, 1.0) * a }
    }

    /// Returns the (R, G, B) values for a colour, as bytes from 0 to 255.
    pub fn bytes(&self) -> (u8, u8, u8) {
        ( (self.r * 255.0).floor() as u8, (self.g * 255.0).floor() as u8, (self.b * 255.0).floor() as u8 )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(mut self, val: f64) -> Self::Output {
        self.r *= val;
        self.g *= val;
        self.b *= val;

        self
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(mut self, val: f64) -> Self::Output {
        self.r /= val;
        self.g /= val;
        self.b /= val;

        self
    }
}

impl Add<Color> for Color {
    type Output = Self;

    fn add(mut self, other: Color) -> Self::Output {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;

        self
    }
}
