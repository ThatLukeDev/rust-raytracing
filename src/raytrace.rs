use crate::vector::Vec3;
use crate::ray::Ray;

use std::ops::*;

/// The trait for all objects within the raytracer.
///
/// This trait must be implemented for every object within the scene.
pub trait Raytrace<T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> {
    /// Gives the distance at which a ray intersects the object.
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T>;

    /// Reflects, refracts, or otherwise transforms the ray
    /// in accordance to how the object should behave.
    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>>;

    /// Gives the position of intersection between a ray and an object.
    fn intersects_at(&self, ray: &Ray<T>) -> Option<Vec3<T>> {
        Some(ray.at(self.intersects_along(ray)?))
    }
}
