use crate::vector::Vec3;

use std::fmt;

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>
}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + From<f64> + Into<f64>> Ray<T>
    where Vec3<T>: Div<T, Output = Vec3<T>> {
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Self {
        Ray::<T> { origin: origin, direction: direction.unit() }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Ray<T> {
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
