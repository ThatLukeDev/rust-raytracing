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

    /// The offset of the plane.
    ///
    /// k in `x+y+z = k`
    pub offset: T,

    /// Colour of the plane.
    pub color: Color,

    /// The uniformity of transmission.
    pub roughness: f64,
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Plane<T> {
    /// Default constructor.
    pub fn new(normal: Vec3<T>, offset: T, color: Color) -> Plane<T> {
        Plane::<T> { normal, offset, color, roughness: 1.0 }
    }

    /// Constructor from 3 points.
    pub fn from_points(p1: Vec3<T>, p2: Vec3<T>, p3: Vec3<T>, color: Color, roughness: f64) -> Plane<T> where T: From<f64>, f64: From<T> {
        // Forms normal from cross product between two plane direction vectors.
        let normal: Vec3<T> = (p1 - p2).cross(&(p1 - p3)).unit();

        // r.n = d
        Plane::<T> { normal, offset: p1 * normal, color, roughness }
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Plane<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        let divisor = ray.direction * self.normal;

        if divisor == 0.0.into() {
            return None;
        }

        let distance = (self.offset - (ray.origin * self.normal)) / divisor;

        if distance < (0.01).into() {
            return None;
        }

        Some(distance)
    }

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        // Uses ThreadRng::Default() so is not re-seeded.
        let mut rng = rand::thread_rng();

        let pos: Vec3<T> = self.intersects_at(ray)?;

        // negative normal cancels itself out in the next line
        let direction = ray.direction - (self.normal * (self.normal * ray.direction) * T::from(2.0));

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
    use super::*;

    #[test]
    fn from_points() {
        assert_eq!(
            Plane::from_points(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 4.0),
                Color::new(1.0, 1.0, 1.0),
                1.0
            ),
            Plane::new(
                Vec3::new(0.0, 1.0, 0.0),
                0.0,
                Color::new(1.0, 1.0, 1.0)
            )
        );
        assert_eq!(
            Plane::from_points(
                Vec3::new(1.0, 2.0, 0.0),
                Vec3::new(0.0, 2.0, 1.0),
                Vec3::new(1.0, 2.0, 4.0),
                Color::new(1.0, 1.0, 1.0),
                1.0
            ),
            Plane::new(
                Vec3::new(0.0, 1.0, 0.0),
                2.0,
                Color::new(1.0, 1.0, 1.0)
            )
        );
    }
}
