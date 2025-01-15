use crate::vector::Vec3;
use crate::ray::Ray;
use crate::raytrace::Raytrace;

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere<T> {
    origin: Vec3<T>,
    radius: T
}

impl<T> Sphere<T> {
    pub fn new(origin: Vec3<T>, radius: T) -> Self {
        Sphere::<T> { origin, radius }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Sphere<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        todo!()
    }
}
