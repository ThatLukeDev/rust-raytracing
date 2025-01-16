use crate::Vec3;
use crate::Ray;

use std::ops::*;

struct Camera<T>(Ray<T>);

impl<T: Copy> Camera<T> {
    pub fn new(position: Vec3<T>, rotation: Vec3<T>) -> Self {
        todo!();
        Camera::<T> { 0: Ray::<T> {
            origin: position,
            direction: rotation,
        } }
    }

    pub fn ray(&self) -> Ray<T> {
        self.0
    }

    pub fn transform(self, vec: Vec3<T>) -> Vec3<T>
    where T: Mul<Output = T> {
        Vec3::<T> { x: self.ray().direction.x * vec.x, y: self.ray().direction.y * vec.y, z: self.ray().direction.z * vec.z }
    }
}
