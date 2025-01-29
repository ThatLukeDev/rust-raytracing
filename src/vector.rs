use std::fmt;

use std::ops::*;

use crate::matrix::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3::<T> { x, y, z }
    }
}

impl<T: From<f64> + Into<f64> + Add<Output = T>> Vec3<T> {
    pub fn round(self) -> Self {
        Vec3::new(
            <T as Into<f64>>::into(self.x + 0.5.into()).floor().into(),
            <T as Into<f64>>::into(self.y + 0.5.into()).floor().into(),
            <T as Into<f64>>::into(self.z + 0.5.into()).floor().into()
        )
    }
}

#[derive(Debug)]
pub struct SizeError;

impl<T: Copy> TryFrom<Vec<T>> for Vec3<T> {
    type Error = SizeError;

    fn try_from(val: Vec<T>) -> Result<Self, SizeError> {
        match val.len() {
            3 => Ok(Vec3::<T> { x: val[0], y: val[1], z: val[2] }),
            _ => Err(SizeError)
        }
    }
}

impl<T: Copy> TryFrom<Matrix<T>> for Vec3<T> {
    type Error = SizeError;

    fn try_from(val: Matrix<T>) -> Result<Self, SizeError> {
        if val.width != 1 {
            return Err(SizeError);
        }
        match val.height {
            3 => Ok(Vec3::<T> { x: val[0][0], y: val[1][0], z: val[2][0] }),
            _ => Err(SizeError)
        }
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T> + From<f64> + Into<f64>> Vec3<T>
    where Self: Div<T, Output = Self> {
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> T {
        self.length_squared().into().sqrt().into()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T: Copy + Add<Output = T>> Add for Vec3<T>
    where Self: Copy {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3::<T> {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vec3<T>
    where Self: Copy {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3::<T> {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vec3<T>
    where Self: Copy {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Vec3::<T> {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vec3<T>
    where Self: Copy {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Vec3::<T> {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl<T: Copy + Mul<Output = T> + Add<Output = T>> Mul for Vec3<T>
    where Self: Copy {
    type Output = T;

    fn mul(self, other: Self) -> T {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z
    }
}
