use rand::Rng;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::raytrace::Raytrace;
use crate::color::Color;

use std::ops::*;

/// Plane.
///
/// Stored by a normal vector and bounds.
///
/// Implements the Raytrace trait.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane<T> {
    /// Normal vector of the plane.
    pub normal: Vec3<T>,

    /// The offset of the plane
    ///
    /// k in `x+y+z = k`
    pub offset: T,

    /// Colour of the plane.
    pub color: Color,
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Plane<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        todo!();
    }

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        todo!();
    }

    fn recolor(&self, _ray: &Ray<T>, color: Color) -> Color {
        let mut out = color;

        out.r *= self.color.r;
        out.g *= self.color.g;
        out.b *= self.color.b;

        out
    }
}
