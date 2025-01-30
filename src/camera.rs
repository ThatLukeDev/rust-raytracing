use crate::vector::*;
use crate::ray::*;
use crate::matrix::*;

use std::ops::*;

pub struct Camera<T> {
    pub position: Vec3<T>,
    pub rotation: Matrix<T>,
}

impl<T: Copy + Add + Sub + Mul + Div> Camera<T> {
    pub fn new(position: Vec3<T>, rotation: Vec3<T>) -> Self
        where T: From<i32>, Vec3<T>: Into<Matrix<T>> {
        Camera::<T> {
            position: position,

            rotation: <Vec3<T> as Into<Matrix<T>>>::into(rotation)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform() {
        let rotright = Camera::<f64> {
            position: Vec3::new(0.0, 10.0, 0.0),
            rotation: matrix![
                0.0, 0.0, 1.0;
                0.0, 1.0, 0.0;
                -1.0, 0.0, 0.0;
            ]
        };

        assert_eq!(
            rotright.transform(Vec3::new(1.0, 0.0, 0.0)),
            Vec3::new(0.0, 0.0, -1.0)
        );

        assert_eq!(
            rotright.transform(Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(3.0, 2.0, -1.0)
        );

        assert_eq!(
            Camera::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0)).transform(Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 2.0, 3.0)
        );

        assert_eq!(
            Camera::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 90.0, 0.0)).transform(Vec3::new(1.0, 2.0, 3.0)).round(),
            Vec3::new(3.0, 2.0, -1.0)
        );
    }
}
