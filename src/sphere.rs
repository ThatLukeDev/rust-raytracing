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

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Sphere<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        let offset = ray.origin - self.origin;

        let a = ray.direction * ray.direction;
        let b = ray.direction * offset * (2.0).into();
        let c = offset * offset - self.radius * self.radius;

        let discriminant = b * b - a * c * (4.0).into();

        if discriminant < (0.0).into() {
            return None;
        }

        Some( (b * (-1.0).into() - discriminant.into().sqrt().into()) / (a * (2.0).into()) )
    }

    fn normal_from(&self, ray: &Ray<T>) -> Option<Vec3<T>> {
        Some((ray.origin - self.origin).unit())
    }
}
