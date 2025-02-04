use crate::color::*;

use std::ops::*;

/// An image struct.
///
/// Stores a 2D array, defined by constant sizes [WIDTH] and [HEIGHT], of [Color].
///
/// [WIDTH] and [HEIGHT] must be constant at compile time.
pub struct Image<const WIDTH: usize, const HEIGHT: usize> {
    /// 2D array of pixels.
    pub data: [[Color; HEIGHT]; WIDTH]
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
