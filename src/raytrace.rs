use crate::vector::Vec3;
use crate::ray::Ray;

use std::ops::*;

pub trait Raytrace<T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T>;

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>>;

    fn intersects_at(&self, ray: &Ray<T>) -> Option<Vec3<T>> {
        Some(ray.at(self.intersects_along(ray)?))
    }
}
