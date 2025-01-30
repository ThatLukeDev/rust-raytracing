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

    pub fn normal_at(&self, &pos: &Vec3<T>) -> Vec3<T>
        where T: Copy + Add<Output = T> + Mul<Output = T> + From<f64> + Into<f64> + Div<Output = T>, Vec3<T>: Copy + Sub<Output = Vec3<T>> {
        (pos - self.origin).unit()
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

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        let pos: Vec3<T> = self.intersects_at(ray)?;
        let normal: Vec3<T> = self.normal_at(&pos);

        Some(Ray::new(pos, ray.direction - (normal * (normal * ray.direction) * T::from(2.0))))
    }
}

#[cfg(test)]
mod tests {
    mod raytrace {
        use super::super::*;

        #[test]
        fn intersects() {
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
        fn normal() {
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_at(&Vec3::new(1.0, 3.0, 3.0)),
                Vec3::new(0.0, 1.0, 0.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_at(&Vec3::new(2.0, 2.0, 3.0)),
                Vec3::new(1.0, 0.0, 0.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0).normal_at(&Vec3::new(0.0, 2.0, 3.0)),
                Vec3::new(-1.0, 0.0, 0.0)
            );
        }
    }
}
