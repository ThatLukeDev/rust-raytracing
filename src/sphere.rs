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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raytrace_intersects() {
        assert_eq!(
            Sphere::new(Vec3::new(0.0, 2.0, 0.0), 1.0).intersects_at(&Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))),
            Some(Vec3::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Sphere::new(Vec3::new(6.0, 0.0, 8.0), 1.0).intersects_along(&Ray::new(Vec3::new(3.0, 0.0, 4.0), Vec3::new(3.0, 0.0, 4.0))),
            Some(4.0)
        );
        assert_eq!(
            Sphere::new(Vec3::new(6.0, 0.0, 8.0), 1.0).intersects_along(&Ray::new(Vec3::new(-3.0, 0.0, 4.0), Vec3::new(3.0, 0.0, 4.0))),
            None
        );
    }

    #[test]
    fn raytrace_normal() {
        assert_eq!(
            Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_from(&Ray::new(Vec3::new(1.0, 3.0, 3.0), Vec3::new(0.0, 0.0, 0.0))),
            Some(Vec3::new(0.0, 1.0, 0.0))
        );
        assert_eq!(
            Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_from(&Ray::new(Vec3::new(2.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0))),
            Some(Vec3::new(1.0, 0.0, 0.0))
        );
        assert_eq!(
            Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_from(&Ray::new(Vec3::new(0.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0))),
            Some(Vec3::new(-1.0, 0.0, 0.0))
        );
    }
}
