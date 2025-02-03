use std::ops::*;
use std::fmt;

use crate::vector::*;

/// The error type for a mismatch of sizes between matrices.
#[derive(Debug, Clone)]
pub struct SizeMismatch;

impl fmt::Display for SizeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid size(s) for matrix")
    }
}

/// A matrix.
///
/// The contents are stored in the format [m][n],
/// where m and n are the rows and columns respectively.
#[derive(Debug)]
pub struct Matrix<T> {
    /// Height, m, of the matrix.
    pub height: usize,

    /// Width, n, of the matrix.
    pub width: usize,

    /// Contents of the matrix.
    ///
    /// The size of the contents vectors must match height and width.
    pub contents: Vec<Vec<T>>
}

impl<T: PartialEq + Copy> PartialEq for Matrix<T> {
    /// Checks for equality between 2 matrices.
    fn eq(&self, other: &Matrix<T>) -> bool {
        if self.width != other.width {
            return false;
        }
        if self.height != other.height {
            return false;
        }
        if self.contents.len() != self.height {
            return false;
        }
        if other.contents.len() != self.height {
            return false;
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if self[i][j] != other[i][j] {
                    return false;
                }
            }
        }

        true
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! count_args { // recursive helper func
    () => (0usize);
    ($x:expr) => (1usize);
    ($x:expr, $($y:expr),+) => (1usize + count_args!($($y),+));
}

#[macro_export]
#[doc(hidden)]
macro_rules! count_expr { // recursive helper func
    () => (0usize);
    ($($x:expr),+) => (1usize);
    ($($x:expr),+; $($($y:expr),+);+) => (1usize + count_expr!($($($y),+);+));
}

#[macro_export]
#[doc(hidden)]
macro_rules! count_expr_args { // recursive helper func
    () => (0usize);
    ($($x:expr),+) => (count_args!($($x),+));
    ($($x:expr),+; $($($y:expr),+);+ $(;)?) => (count_args!($($x),+) + count_expr_args!($($($y),+);+));
}

#[macro_export]
#[doc(hidden)]
macro_rules! wrap_in_vec {
    ($($x:expr),+ $(,)?) => {
        vec!(vec!($($x),+))
    };

    ($($x:expr),+; $($($y:expr),+);+ $(;)?) => {
        vec!(vec!(vec!($($x),+)), wrap_in_vec!($($($y),+);+)).concat()
    };
}

pub use count_args;
pub use count_expr;
pub use count_expr_args;
pub use wrap_in_vec;

/// Shorthand for creating a matrix.
///
/// ```
/// # use rusttracing::matrix::*;
/// assert_eq!(
///     *matrix![
///         1, 2;
///         3, 4;
///         5, 6;
///     ].at(2, 2),
///     4
/// )
/// ```
#[macro_export]
macro_rules! matrix {
    ( $($($element:expr),+);+ $(;)? ) => {
        Matrix {
            width: count_expr_args!( $($($element),+);+ ) / count_expr!( $($($element),+);+ ),
            height: count_expr!( $($($element),+);+ ),

            contents: wrap_in_vec!($($($element),+);+)
        }
    }
}

pub use matrix;

impl<T: Copy> Clone for Matrix<T> {
    /// Clones a matrix, including contents.
    ///
    /// There is no copy trait due to a potential for large Matrix contents.
    fn clone(&self) -> Self {
        Matrix::<T> {
            height: self.height,
            width: self.width,
            contents: self.contents.clone()
        }
    }
}

impl<T: Copy + Add + Sub + Mul + Div> Matrix<T> {
    /// Creates a new mxn matrix of 0.
    pub fn new(height: usize, width: usize) -> Self
        where T: From<i32> {
        Matrix::<T> {
            width,
            height,
            contents: vec![vec![0.into(); width]; height]
        }
    }

    /// Creates a new mxm matrix of the identity matrix.
    ///
    /// Each diagonal will be 1, all other values will be 0.
    pub fn ident(height: usize) -> Self
        where T: From<i32> {
        let mut result = Matrix::<T> {
            width: height,
            height: height,
            contents: vec![vec![0.into(); height]; height]
        };

        for i in 0..height {
            result[i][i] = 1.into();
        }

        result
    }

    /// Getter for height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Getter for width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the mxnth element, where m and n start at 1.
    pub fn at(&self, m: usize, n: usize) -> &T {
        &self.contents[m-1][n-1]
    }

    /// Returns a mutable reference to the mxnth element, where m and n start at 1.
    pub fn mut_at(&mut self, m: usize, n: usize) -> &T {
        &mut self.contents[m-1][n-1]
    }
}

impl<T: Copy + From<f64> + Into<f64>
    + Add + Div + Sub + Mul + Neg<Output = T>> From<Vec3<T>> for Matrix<T>
    where Matrix<T>: Mul<Output = Result<Matrix<T>, SizeMismatch>> {
    /// Converts a Vec3 as a rotation vector to a 3x3 rotation matrix.
    fn from(val: Vec3<T>) -> Self {
        let sinx: T = <T as Into<f64>>::into(val.x).to_radians().sin().into();
        let cosx: T = <T as Into<f64>>::into(val.x).to_radians().cos().into();
        let siny: T = <T as Into<f64>>::into(val.y).to_radians().sin().into();
        let cosy: T = <T as Into<f64>>::into(val.y).to_radians().cos().into();
        let sinz: T = <T as Into<f64>>::into(val.z).to_radians().sin().into();
        let cosz: T = <T as Into<f64>>::into(val.z).to_radians().cos().into();

        ((matrix![
            cosz,       sinz,       0.0.into();
            -sinz,      cosz,       0.0.into();
            0.0.into(), 0.0.into(), 1.0.into();
        ] * matrix![
            cosy,       0.0.into(), siny;
            0.0.into(), 1.0.into(), 0.0.into();
            -siny,      0.0.into(), cosy;
        ]).unwrap() * matrix![
            1.0.into(), 0.0.into(), 0.0.into();
            0.0.into(), cosx,       sinx;
            0.0.into(), -sinx,      cosx;
        ]).unwrap()
    }
}

impl<T: Copy> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    /// Returns a reference to the mxnth element, where m and n start at 0.
    fn index<'a>(&'a self, i: usize) -> &'a Vec<T> {
        &self.contents[i]
    }
}

impl<T: Copy> IndexMut<usize> for Matrix<T> {
    /// Returns a mutable reference to the mxnth element, where m and n start at 0.
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<T> {
        &mut self.contents[i]
    }
}

impl<T: Copy> fmt::Display for Matrix<T> {
    /// Displys the height and width of a matrix, in the format `[ m n ]`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} {} ]", self.height, self.width)
    }
}

impl<T: Copy + Add<Output = T>> Add for Matrix<T> {
    type Output = Result<Self, SizeMismatch>;

    /// Adds each element in a matrix to the corresponding element.
    ///
    /// Will fail if height and width do not match.
    fn add(mut self, other: Self) -> Self::Output {
        if self.width != other.width {
            return Err(SizeMismatch);
        }
        if self.height != other.height {
            return Err(SizeMismatch);
        }

        for i in 0..self.height {
            for j in 0..self.width {
                self[i][j] = self[i][j] + other[i][j];
            }
        }

        Ok(self)
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Result<Self, SizeMismatch>;

    /// Subtracts each element in a matrix from the corresponding element.
    ///
    /// Will fail if height and width do not match.
    fn sub(mut self, other: Self) -> Self::Output {
        if self.width != other.width {
            return Err(SizeMismatch);
        }
        if self.height != other.height {
            return Err(SizeMismatch);
        }

        for i in 0..self.height {
            for j in 0..self.width {
                self[i][j] = self[i][j] - other[i][j];
            }
        }

        Ok(self)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Matrix<T> {
    type Output = Self;

    /// Scales each element in a matrix by a factor.
    fn mul(mut self, other: T) -> Self::Output {
        for i in 0..self.height {
            for j in 0..self.width {
                self[i][j] = self[i][j] * other;
            }
        }

        self
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Matrix<T> {
    type Output = Self;

    /// Scales each element in a matrix by 1 divided by the factor.
    fn div(mut self, other: T) -> Self::Output {
        for i in 0..self.height {
            for j in 0..self.width {
                self[i][j] = self[i][j] / other;
            }
        }

        self
    }
}

impl<T: Copy + From<i32> + Sub + Div + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Result<Self, SizeMismatch>;

    /// Matrix multiplication.
    ///
    /// Takes two matrices: m1xn1; m2xn2,
    /// and returns the matrix multiplication of the two matrices,
    /// in the form m1xn2.
    ///
    /// ```
    /// # use rusttracing::matrix::*;
    /// assert_eq!(
    ///     (matrix![
    ///         1, 2;
    ///         3, 4;
    ///         5, 6;
    ///     ] * matrix![
    ///         1, 0;
    ///         0, 1;
    ///     ]).unwrap(),
    ///     matrix![
    ///         1, 2;
    ///         3, 4;
    ///         5, 6;
    ///     ]
    /// );
    /// ```
    fn mul(self, other: Self) -> Self::Output {
        if self.width != other.height {
            return Err(SizeMismatch);
        }

        let mut result = Matrix::new(self.height, other.width);

        for i in 0..result.height {
            for j in 0..result.width {
                for k in 0..other.height {
                    result[i][j] = result[i][j] + self[i][k] * other[k][j];
                }
            }
        }

        Ok(result)
    }
}

impl<T: Copy + Add + Sub + Mul + Div + From<i32>> Matrix<T> {
    /// Transposes a matrix.
    ///
    /// mxn -> nxm.
    ///
    /// ```
    /// # use rusttracing::matrix::*;
    /// assert_eq!(
    ///     matrix![
    ///         1, 2;
    ///         3, 4;
    ///         5, 6;
    ///     ].transpose(),
    ///     matrix![
    ///         1, 3, 5;
    ///         2, 4, 6;
    ///     ]
    /// );
    /// ```
    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(self.width, self.height);

        for i in 0..self.height {
            for j in 0..self.width {
                result[j][i] = self[i][j];
            }
        }

        result
    }
}

impl<T: Copy + Add + Sub + Mul<Output = T> + Div + From<i32>> Matrix<T> {
    /// Returns the matrix of cofactors.
    ///
    /// ```
    /// # use rusttracing::matrix::*;
    /// assert_eq!(
    ///     matrix![
    ///         1, 2, 3;
    ///         3, 4, 5;
    ///         5, 6, 7;
    ///     ].cofactors(),
    ///     matrix![
    ///         1, -2, 3;
    ///         -3, 4, -5;
    ///         5, -6, 7;
    ///     ]
    /// );
    /// ```
    pub fn cofactors(mut self) -> Self {
        for i in 0..self.height {
            for j in 0..self.width {
                if i % 2 + j % 2 == 1 { // either or
                    self[i][j] = self[i][j] * (-1).into();
                }
            }
        }

        self
    }
}

impl<T: Copy + Add<Output = T> + Sub + Mul<Output = T> + Div + From<i32>> Matrix<T> {
    /// Returns the minor of a matrix.
    ///
    /// The minor of a matrix is that matrix without the row and column of the specified element.
    ///
    /// ```
    /// # use rusttracing::matrix::*;
    /// assert_eq!(
    ///     matrix![
    ///         1, 2, 7;
    ///         3, 4, 8;
    ///         5, 6, 9;
    ///     ].minor(1, 2),
    ///     matrix![
    ///         3, 8;
    ///         5, 9;
    ///     ]
    /// );
    /// ```
    pub fn minor(&self, m: usize, n: usize) -> Self {
        let mut result = Matrix::new(self.height - 1, self.width - 1);

        let mut working_i = 0;
        for i in 0..self.height {
            if i == m - 1 {
                continue;
            }

            let mut working_j = 0;
            for j in 0..self.width {
                if j == n - 1 {
                    continue;
                }

                result[working_i][working_j] = self[i][j];

                working_j += 1;
            }

            working_i += 1;
        }

        result
    }

    /// Returns the determinant of aa matrix.
    ///
    /// The determinant of a matrix is the minor matrix's determinant
    /// from each element in one row or column,
    /// multiplied by the matrix of cofactors for that element or column.
    ///
    /// ```
    /// # use rusttracing::matrix::*;
    /// assert_eq!(
    ///     matrix![
    ///         1, 2;
    ///         3, 4;
    ///     ].det().unwrap(),
    ///     -2
    /// );
    /// ```
    pub fn det(&self) -> Result<T, SizeMismatch> {
        if self.height != self.width {
            return Err(SizeMismatch);
        }

        if self.height == 1 {
            return Ok(self[0][0]);
        }

        let mut det: T = (0).into();

        for i in 0..self.width {
            det = det + self.minor(1, i + 1).det().unwrap() * (((i as i32 + 1) % 2) * 2 - 1).into() * self[0][i]; // cofactors
        }

        Ok(det)
    }

    /// Returns the matrix of minors.
    ///
    /// Where each element becomes the determinant of its minor matrix.
    pub fn minors(&self) -> Result<Self, SizeMismatch> {
        if self.height != self.width {
            return Err(SizeMismatch);
        }

        let mut result = Matrix::new(self.height, self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                result[i][j] = self.minor(i + 1, j + 1).det()?;
            }
        }

        Ok(result)
    }

    /// Inverses a matrix.
    ///
    /// Such that a matrix multiplied by its inverse becomes the identity.
    ///
    /// WIll fail if the determinant of the matrix is zero, and thus has no inverse.
    ///
    /// ```
    /// # use rusttracing::matrix::*;
    /// assert_eq!(
    ///     matrix![
    ///         1, 1, 1;
    ///         1, 2, -3;
    ///         1, -3, 18;
    ///     ].inverse().unwrap(),
    ///     matrix![
    ///         27, -21, -5;
    ///         -21, 17, 4;
    ///         -5, 4, 1;
    ///     ]
    /// );
    /// ```
    pub fn inverse(&self) -> Result<Self, SizeMismatch>
        where Matrix<T>: Div<T, Output = Matrix<T>> {
        if self.height != self.width {
            return Err(SizeMismatch);
        }

        let result = self.minors()?.cofactors() / self.det()?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_count() {
        assert_eq!(count_args!(), 0);
        assert_eq!(count_args!(1+1), 1);
        assert_eq!(count_args!(4, 3), 2);
    }

    #[test]
    fn expr_count() {
        assert_eq!(count_expr!(), 0);
        assert_eq!(count_expr!(1+1), 1);
        assert_eq!(count_expr!(4, 3; 2, 3, 4), 2);
        assert_eq!(count_expr!(4, 3; 2, 3; 4), 3);
    }

    #[test]
    fn new() {
        assert_eq!(
            matrix![
                0, 0, 0;
                0, 0, 0;
                0, 0, 0;
            ],
            Matrix::new(3, 3)
        );
        assert_eq!(
            matrix![
                0, 0;
                0, 0;
                0, 0;
            ],
            Matrix::new(3, 2)
        );
        assert_eq!(
            matrix![
                0, 0, 0;
                0, 0, 0;
            ],
            Matrix::new(2, 3)
        );
    }

    #[test]
    fn at() {
        assert_eq!(
            *matrix![
                0, 2, 0;
                0, 0, 4;
            ].at(2, 3),
            4
        );
        assert_eq!(
            *matrix![
                0, 2, 0;
                0, 0, 4;
            ].at(1, 2),
            2
        );
    }

    #[test]
    fn add() {
        assert_eq!(
            (matrix![
                1, 2, 3;
                4, 5, 6;
            ] + matrix![
                1, 3, 5;
                2, 4, 6;
            ]).unwrap(),
            matrix![
                2, 5, 8;
                6, 9, 12;
            ]
        );
    }

    #[test]
    #[should_panic]
    fn add_mismatch() {
        (matrix![
            1, 2;
            4, 5;
            7, 8;
        ] + matrix![
            1, 3, 5;
            2, 4, 6;
        ]).unwrap();
    }

    #[test]
    fn sub() {
        assert_eq!(
            (matrix![
                1, 2, 3;
                4, 5, 6;
            ] - matrix![
                1, 3, 5;
                2, 4, 6;
            ]).unwrap(),
            matrix![
                0, -1, -2;
                2, 1, 0;
            ]
        );
    }

    #[test]
    #[should_panic]
    fn sub_mismatch() {
        (matrix![
            1, 2;
            4, 5;
            7, 8;
        ] - matrix![
            1, 3, 5;
            2, 4, 6;
        ]).unwrap();
    }

    #[test]
    fn scale() {
        assert_eq!(
            matrix![
                1, 2, 3;
                4, 5, 6;
            ] * 2,
            matrix![
                2, 4, 6;
                8, 10, 12;
            ]
        );
        assert_eq!(
            matrix![
                1, 2, 4;
                4, 8, 6;
            ] / 2,
            matrix![
                0, 1, 2;
                2, 4, 3;
            ]
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            (matrix![
                1, 2;
                3, 4;
                5, 6;
            ] * matrix![
                1, 0;
                0, 1;
            ]).unwrap(),
            matrix![
                1, 2;
                3, 4;
                5, 6;
            ]
        );
        assert_eq!(
            (matrix![
                1, 2, 7;
                3, 4, 8;
                5, 6, 9;
            ] * matrix![
                1, 0, 0;
                0, 1, 0;
                0, 0, 1;
            ]).unwrap(),
            matrix![
                1, 2, 7;
                3, 4, 8;
                5, 6, 9;
            ]
        );
        assert_eq!(
            (matrix![
                1, 2;
                3, 4;
                5, 6;
            ] * matrix![
                1, 0;
                1, 1;
            ]).unwrap(),
            matrix![
                3, 2;
                7, 4;
                11, 6;
            ]
        );
    }

    #[test]
    #[should_panic]
    fn mul_mismatch() {
        (matrix![
            1, 2;
            4, 5;
            7, 8;
        ] * matrix![
            1, 2;
            4, 5;
            7, 8;
        ]).unwrap();
    }

    #[test]
    fn transpose() {
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
                5, 6;
            ].transpose(),
            matrix![
                1, 3, 5;
                2, 4, 6;
            ]
        );
    }

    #[test]
    fn cofactor() {
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
            ].cofactors(),
            matrix![
                1, -2;
                -3, 4;
            ]
        );
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
                5, 6;
            ].cofactors(),
            matrix![
                1, -2;
                -3, 4;
                5, -6;
            ]
        );
    }

    #[test]
    fn minor() {
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
            ].minor(1, 2),
            matrix![
                3;
            ]
        );
        assert_eq!(
            matrix![
                1, 2, 7;
                3, 4, 8;
                5, 6, 9;
            ].minor(1, 2),
            matrix![
                3, 8;
                5, 9;
            ]
        );
    }

    #[test]
    fn det() {
        assert_eq!(
            matrix![
                1, 0;
                0, 1;
            ].det().unwrap(),
            1
        );
        assert_eq!(
            matrix![
                1, 2;
                3, 4;
            ].det().unwrap(),
            -2
        );
        assert_eq!(
            matrix![
                1, 2, 3;
                3, 2, 1;
                2, 1, 3;
            ].det().unwrap(),
            -12
        );
    }

    #[test]
    fn minors() {
        assert_eq!(
            matrix![
                1, 2, 3;
                3, 2, 1;
                2, 1, 3;
            ].minors().unwrap(),
            matrix![
                5, 7, -1;
                3, -3, -3;
                -4, -8, -4;
            ]
        );
    }

    #[test]
    fn inverse() {
        assert_eq!(
            matrix![
                1, 0, 0;
                0, 1, 0;
                0, 0, 1;
            ].inverse().unwrap(),
            matrix![
                1, 0, 0;
                0, 1, 0;
                0, 0, 1;
            ]
        );
        assert_eq!(
            matrix![
                1, 1, 1;
                1, 2, -3;
                1, -3, 18;
            ].inverse().unwrap(),
            matrix![
                27, -21, -5;
                -21, 17, 4;
                -5, 4, 1;
            ]
        );
    }
}
