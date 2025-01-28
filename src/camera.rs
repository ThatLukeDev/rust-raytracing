use crate::Vec3;
use crate::Ray;
use crate::matrix::*;

use std::ops::*;

pub struct Camera<T> {
    pub position: Vec3<T>,
    pub rotation: Matrix<T>,
}

impl<T: Copy + Add + Sub + Mul + Div> Camera<T> {
    pub fn new(position: Vec3<T>, rotation: Vec3<T>) -> Self
        where T: From<i32> {
        Camera::<T> {
            position: position,

            rotation: Matrix::new(3, 3)
        }
    }

    pub fn ray(&self) -> Ray<T> {
        Ray::<T> {
            origin: self.position,
            direction: self.rotation[0].clone().try_into().unwrap()
        }
    }

    pub fn transform(&self, vec: Vec3<T>) -> Vec3<T>
    where T: Mul<Output = T> + Add<Output = T>, Matrix<T>: Mul<Output = Result<Matrix<T>, SizeMismatch>> {
        (self.rotation.clone() * matrix![
            vec.x;
            vec.y;
            vec.z;
        ]).unwrap().clone().try_into().unwrap()
    }
}
