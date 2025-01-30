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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Vec3::new(2,4,6) + Vec3::new(3,6,9), Vec3::new(5,10,15));
        assert_eq!(Vec3::new(3,9,3) + Vec3::new(2,7,1), Vec3::new(5,16,4));
        assert_eq!(Vec3::new(5,9,5) + Vec3::new(5,4,2), Vec3::new(10,13,7));
        assert_eq!(Vec3::new(7,8,5) + Vec3::new(3,3,3), Vec3::new(10,11,8));
        assert_eq!(Vec3::new(0,1,2) + Vec3::new(1,2,4), Vec3::new(1,3,6));
    }

    #[test]
    fn sub() {
        assert_eq!(Vec3::new(2,4,6) - Vec3::new(3,6,9), Vec3::new(-1,-2,-3));
        assert_eq!(Vec3::new(3,9,3) - Vec3::new(2,7,1), Vec3::new(1,2,2));
        assert_eq!(Vec3::new(5,9,5) - Vec3::new(5,4,2), Vec3::new(0,5,3));
        assert_eq!(Vec3::new(7,8,5) - Vec3::new(3,3,3), Vec3::new(4,5,2));
        assert_eq!(Vec3::new(0,1,2) - Vec3::new(1,2,4), Vec3::new(-1,-1,-2));
    }

    #[test]
    fn scale() {
        assert_eq!(Vec3::new(2,4,6) * 3, Vec3::new(6,12,18));
        assert_eq!(Vec3::new(3,9,3) * 2, Vec3::new(6,18,6));
        assert_eq!(Vec3::new(5,9,5) * 5, Vec3::new(25,45,25));
        assert_eq!(Vec3::new(7,8,5) * 3, Vec3::new(21,24,15));
        assert_eq!(Vec3::new(0,1,2) * 1, Vec3::new(0,1,2));
    }

    #[test]
    fn overscale() {
        assert_eq!(Vec3::new(2.0,4.0,6.0) / 4.0, Vec3::new(0.5,1.0,1.5));
        assert_eq!(Vec3::new(3.0,9.0,3.0) / 2.0, Vec3::new(1.5,4.5,1.5));
        assert_eq!(Vec3::new(5.0,9.0,5.0) / 5.0, Vec3::new(1.0,1.8,1.0));
        assert_eq!(Vec3::new(7.0,8.0,5.0) / 10.0, Vec3::new(0.7,0.8,0.5));
        assert_eq!(Vec3::new(0.0,1.0,2.0) / 1.0, Vec3::new(0.0,1.0,2.0));
    }

    #[test]
    fn dot() {
        assert_eq!(Vec3::new(2,4,6) * Vec3::new(3,6,9), 84);
        assert_eq!(Vec3::new(3,9,3) * Vec3::new(2,7,1), 72);
        assert_eq!(Vec3::new(5,9,5) * Vec3::new(5,4,2), 71);
        assert_eq!(Vec3::new(7,8,5) * Vec3::new(3,3,3), 60);
        assert_eq!(Vec3::new(0,1,2) * Vec3::new(1,2,4), 10);
    }

    #[test]
    fn unit() {
        assert_eq!(Vec3::new(2.0,0.0,0.0).unit(), Vec3::new(1.0,0.0,0.0));
        assert_eq!(Vec3::new(0.0,5.0,0.0).unit(), Vec3::new(0.0,1.0,0.0));
        assert_eq!(Vec3::new(0.0,0.0,4.0).unit(), Vec3::new(0.0,0.0,1.0));
    }
}
