use crate::Vec3;
use crate::Ray;

use std::ops::*;

struct Camera<T> {
    position: Vec3<T>,
    rotation: Vec3<T>,

    direction: Vec3<T>,
    up_direction: Vec3<T>,
    right_direction: Vec3<T>,
}

impl<T: Copy> Camera<T> {
    pub fn new(position: Vec3<T>, rotation: Vec3<T>) -> Self {
        todo!();
        Camera::<T> {
            position: position,
            rotation: rotation,

            direction: position, // todo
            up_direction: position, // todo
            right_direction: position // todo
        }
    }

    pub fn ray(&self) -> Ray<T> {
        Ray::<T> { origin: self.position, direction: self.direction }
    }

    pub fn transform(self, vec: Vec3<T>) -> Vec3<T>
    where T: Mul<Output = T> {
        todo!()
    }
}
