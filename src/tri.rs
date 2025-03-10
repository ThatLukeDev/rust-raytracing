use rand::Rng;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::raytrace::Raytrace;
use crate::color::Color;
use crate::plane::Plane;

use std::ops::*;

/// Triangle.
///
/// Can be constructed from 3 points.
///
/// Stored as a plane, and the 3 point bound.
pub struct Tri<T> {
    /// The 3 point bound.
    pub bounds: Vec3<Vec3<T>>,

    /// The plane constructed from the 3 points.
    pub plane: Plane<T>,
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Tri<T> {
    /// Default constructor.
    pub fn new(p1: Vec3<T>, p2: Vec3<T>, p3: Vec3<T>, color: Color, roughness: f64) -> Tri<T> where T: From<f64>, f64: From<T> {
        Tri::<T> {
            bounds: Vec3::new(p1, p2, p3),
            plane: Plane::from_points(p1, p2, p3, color, roughness),
        }
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Tri<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        todo!()
    }

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        self.plane.transmit(ray)
    }

    fn recolor(&self, _ray: &Ray<T>, color: Color) -> Color {
        self.plane.recolor(_ray, color)
    }
}
