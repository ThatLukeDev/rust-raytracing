use crate::vector::*;
use crate::ray::*;
use crate::matrix::*;

use std::ops::*;

/// A camera object, with a postion and a rotation matrix.
///
/// Can be instantiated with a position and rotation matrix, in degrees [[Camera::new]].
pub struct Camera<T> {
    /// The position of the camera in 3D space.
    pub position: Vec3<T>,

    /// The rotation of the camera as a 3D Matrix.
    ///
    /// * Column 1 is the direction the camera is right-vector.
    /// * Column 2 is the direction of the camera's up-vector.
    /// * Column 3 is the direction of the camera's facing.
    pub rotation: Matrix<T>,
}

impl<T: Copy + Add + Sub + Mul + Div> Camera<T> {
    /// Creates a new camera instance from a position and a rotation.
    ///
    /// Rotation is in degrees and represents clockwise moments about an axis.
    pub fn new(position: Vec3<T>, rotation: Vec3<T>) -> Self
        where T: From<i32>, Vec3<T>: Into<Matrix<T>> {
        Camera::<T> {
            position: position,

            rotation: <Vec3<T> as Into<Matrix<T>>>::into(rotation)
        }
    }

    /// The direction the camera is facing as a ray.
    pub fn ray(&self) -> Ray<T> {
        Ray::<T> {
            origin: self.position,
            direction: self.rotation[0].clone().try_into().unwrap()
        }
    }

    /// Rotates a point in camera space to world space.
    ///
    /// Does **not** offset by position.
    ///
    /// ```
    /// # use rusttracing::vector::*;
    /// # use rusttracing::camera::*;
    /// let cam = Camera::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 90.0, 0.0));
    /// assert_eq!(
    ///     cam.transform(Vec3::new(1.0, 2.0, 3.0)).round(),
    ///     Vec3::new(3.0, 2.0, -1.0)
    /// );
    /// ```
    /// Note: Round is used as the result from transform is -0.999... not -1
    /// due to floating point precision errors.
    pub fn transform(&self, vec: Vec3<T>) -> Vec3<T>
    where T: Mul<Output = T> + Add<Output = T>, Matrix<T>: Mul<Output = Result<Matrix<T>, SizeMismatch>> {
        Vec3::new(
            self.rotation[0][0] * vec.x + self.rotation[0][1] * vec.x + self.rotation[0][2] * vec.x,
            self.rotation[1][0] * vec.y + self.rotation[1][1] * vec.y + self.rotation[1][2] * vec.y,
            self.rotation[2][0] * vec.z + self.rotation[2][1] * vec.z + self.rotation[2][2] * vec.z,
        )
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
