use crate::vector::Vec3;

use std::ops::*;

/// A 3D ray object, that has an origin, and direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray<T> {
    /// The start position of the ray.
    pub origin: Vec3<T>,

    /// The direction of the ray as a unit vector.
    pub direction: Vec3<T>
}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + From<f64> + Into<f64>> Ray<T>
    where Vec3<T>: Div<T, Output = Vec3<T>> {
    /// A safe ray creation function,
    /// which transforms the direction into a unit vector.
    ///
    /// ```
    /// # use rusttracing::vector::*;
    /// # use rusttracing::ray::*;
    /// assert_eq!(
    ///     Ray::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(1000.0, 0.0, 0.0)),
    ///     Ray::<_> { origin: Vec3::new(0.0, 1.0, 2.0), direction: Vec3::new(1.0, 0.0, 0.0) }
    /// );
    /// ```
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Ray::<T> { origin: origin, direction: direction.unit() }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Ray<T> {
    /// The position of a ray at a distance along its path.
    ///
    /// ```
    /// # use rusttracing::vector::*;
    /// # use rusttracing::ray::*;
    /// assert_eq!(
    ///     Ray::new(Vec3::new(0.0, 1.0, 2.0), Vec3::new(1000.0, 0.0, 0.0)).at(1.0),
    ///     Vec3::new(1.0, 1.0, 2.0)
    /// );
    /// ```
    pub fn at<U: Into<T>>(&self, k: U) -> Vec3<T>{
        self.origin + self.direction * k.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at() {
        assert_eq!(
            Ray::new(Vec3::new(1.0,2.0,3.0),Vec3::new(1.0,0.0,0.0)).at(0),
            Vec3::new(1.0,2.0,3.0)
        );
    }
}
