use crate::color::*;

use std::ops::*;

/// An image struct.
///
/// Stores a 2D array, defined by constant sizes WIDTH and HEIGHT, of Color.
///
/// WIDTH and HEIGHT must be constant at compile time.
pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    /// 2D array of pixels.
    pub data: [[Color; HEIGHT]; WIDTH]
}

impl<const WIDTH: usize, const HEIGHT: usize> Image<WIDTH, HEIGHT> {
    /// Creates a new black image based on context.
    pub fn new() -> Self {
        Image::<WIDTH, HEIGHT> {
            data: [[Color::new(0.0, 0.0, 0.0); HEIGHT]; WIDTH]
        }
    }

    /// Turns the image into a PPM compatible byte vec.
    pub fn to_ppm(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];

        // PPM header
        out.extend_from_slice("P8".as_bytes());
        out.push(b' ');
        out.extend_from_slice(WIDTH.to_string().as_bytes());
        out.push(b' ');
        out.extend_from_slice(HEIGHT.to_string().as_bytes());
        out.push(b' ');
        out.extend_from_slice("255".as_bytes());
        out.push(b'\n');

        todo!();

        out
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Index<usize> for Image<WIDTH, HEIGHT> {
    type Output = [Color; HEIGHT];

    /// Returns the inner array, which can then be indexed seperately.
    fn index<'a>(&'a self, i: usize) -> &'a Self::Output {
        &self.data[i]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IndexMut<usize> for Image<WIDTH, HEIGHT> {
    /// Returns the inner array mutably, which can then be indexed seperately.
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut [Color; HEIGHT] {
        &mut self.data[i]
    }
}
