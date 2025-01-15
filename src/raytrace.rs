use crate::vector::Vec3;
use crate::ray::Ray;

use std::ops::*;

pub trait Raytrace<T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> {
    fn intersectsAlong(&self, ray: &Ray<T>) -> Option<T>;

    fn intersectsAt(&self, ray: &Ray<T>) -> Option<Vec3<T>> {
        Some(ray.at(self.intersectsAlong(ray)?))
    }
}
