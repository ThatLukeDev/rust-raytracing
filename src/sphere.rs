use rand::Rng;

use crate::vector::Vec3;
use crate::ray::Ray;
use crate::raytrace::Raytrace;
use crate::color::Color;

use std::ops::*;

/// Sphere.
///
/// Has an origin, and radius.
///
/// Implements the Raytrace trait.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere<T> {
    /// Origin, as a Vec3.
    origin: Vec3<T>,

    /// Radius.
    radius: T,

    /// Colour of sphere.
    color: Color,

    /// The uniformity of transmission.
    roughness: f64,
}

impl<T> Sphere<T> {
    /// Default constructor.
    pub fn new(origin: Vec3<T>, radius: T, color: Color) -> Self {
        Sphere::<T> { origin, radius, color, roughness: 1.0 }
    }

    /// Gives the normal to a point on the sphere.
    pub fn normal_at(&self, &pos: &Vec3<T>) -> Vec3<T>
        where T: Copy + Add<Output = T> + Mul<Output = T> + From<f64> + Into<f64> + Div<Output = T>, Vec3<T>: Copy + Sub<Output = Vec3<T>> {
        (pos - self.origin).unit()
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Sphere<T> {
    /// Gives the distance along a ray that a sphere lies.
    ///
    /// Returns None if no solutions exist.
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        let offset = ray.origin - self.origin;

        let a = ray.direction * ray.direction;
        let b = ray.direction * offset * (2.0).into();
        let c = offset * offset - self.radius * self.radius;

        let discriminant = b * b - a * c * (4.0).into();

        if discriminant < (0.0).into() {
            return None;
        }

        let distance = (b * (-1.0).into() - discriminant.into().sqrt().into()) / (a * (2.0).into());

        if distance < (0.01).into() {
            return None;
        }

        Some(distance)
    }

    /// Reflects a ray along the normal.
    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        // Uses ThreadRng::Default() so is not re-seeded.
        let mut rng = rand::thread_rng();

        let pos: Vec3<T> = self.intersects_at(ray)?;
        let normal: Vec3<T> = self.normal_at(&pos);

        let direction = ray.direction - (normal * (normal * ray.direction) * T::from(2.0));

        let random: Vec3<T> = Vec3::new(rng.gen_range(-1.0..1.0).into(), rng.gen_range(-1.0..1.0).into(), rng.gen_range(-1.0..1.0).into());

        Some(Ray::new(pos, direction + random * <f64 as Into<T>>::into(self.roughness)))
    }

    fn recolor(&self, _ray: &Ray<T>, color: Color) -> Color {
        let mut out = color;

        out.r *= self.color.r;
        out.g *= self.color.g;
        out.b *= self.color.b;

        out
    }
}

#[cfg(test)]
mod tests {
    mod raytrace {
        use super::super::*;

        #[test]
        fn intersects() {
            assert_eq!(
                Sphere::new(Vec3::new(0.0, 2.0, 0.0), 1.0, Color::new(0.0, 0.0, 0.0)).intersects_at(&Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))),
                Some(Vec3::new(0.0, 1.0, 0.0))
            );
            assert_eq!(
                Sphere::new(Vec3::new(6.0, 0.0, 8.0), 1.0, Color::new(0.0, 0.0, 0.0)).intersects_along(&Ray::new(Vec3::new(3.0, 0.0, 4.0), Vec3::new(3.0, 0.0, 4.0))),
                Some(4.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(6.0, 0.0, 8.0), 1.0, Color::new(0.0, 0.0, 0.0)).intersects_along(&Ray::new(Vec3::new(-3.0, 0.0, 4.0), Vec3::new(3.0, 0.0, 4.0))),
                None
            );
        }

        #[test]
        fn normal() {
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0, Color::new(0.0, 0.0, 0.0)).normal_at(&Vec3::new(1.0, 3.0, 3.0)),
                Vec3::new(0.0, 1.0, 0.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0, Color::new(0.0, 0.0, 0.0)).normal_at(&Vec3::new(2.0, 2.0, 3.0)),
                Vec3::new(1.0, 0.0, 0.0)
            );
            assert_eq!(
                Sphere::new(Vec3::new(1.0, 2.0, 3.0), 1.0, Color::new(0.0, 0.0, 0.0)).normal_at(&Vec3::new(0.0, 2.0, 3.0)),
                Vec3::new(-1.0, 0.0, 0.0)
            );
        }
    }
}
