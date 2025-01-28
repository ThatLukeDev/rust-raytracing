use std::ops::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SizeMismatch;

impl fmt::Display for SizeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid size(s) for matrix")
    }
}

#[derive(Debug)]
pub struct Matrix<T> {
    pub height: usize,
    pub width: usize,

    pub contents: Vec<Vec<T>>
}

impl<T: PartialEq + Copy> PartialEq for Matrix<T> {
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

macro_rules! count_args { // recursive helper func
    () => (0usize);
    ($x:expr) => (1usize);
    ($x:expr, $($y:expr),+) => (1usize + count_args!($($y),+));
}

macro_rules! count_expr { // recursive helper func
    () => (0usize);
    ($($x:expr),+) => (1usize);
    ($($x:expr),+; $($($y:expr),+);+) => (1usize + count_expr!($($($y),+);+));
}

macro_rules! count_expr_args { // recursive helper func
    () => (0usize);
    ($($x:expr),+) => (count_args!($($x),+));
    ($($x:expr),+; $($($y:expr),+);+ $(;)?) => (count_args!($($x),+) + count_expr_args!($($($y),+);+));
}

macro_rules! wrap_in_vec {
    ($($x:expr),+ $(,)?) => {
        vec!(vec!($($x),+))
    };

    ($($x:expr),+; $($($y:expr),+);+ $(;)?) => {
        vec!(vec!(vec!($($x),+)), wrap_in_vec!($($($y),+);+)).concat()
    };
}

pub(crate) use count_args;
pub(crate) use count_expr;
pub(crate) use count_expr_args;
pub(crate) use wrap_in_vec;

macro_rules! matrix {
    ( $($($element:expr),+);+ $(;)? ) => {
        Matrix {
            width: count_expr_args!( $($($element),+);+ ) / count_expr!( $($($element),+);+ ),
            height: count_expr!( $($($element),+);+ ),

            contents: wrap_in_vec!($($($element),+);+)
        }
    }
}

pub(crate) use matrix;

impl<T: Copy> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Matrix::<T> {
            height: self.height,
            width: self.width,
            contents: self.contents.clone()
        }
    }
}

impl<T: Copy + Add + Sub + Mul + Div> Matrix<T> {
    pub fn new(height: usize, width: usize) -> Self
        where T: From<i32> {
        Matrix::<T> {
            width,
            height,
            contents: vec![vec![0.into(); width]; height]
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn at(&self, m: usize, n: usize) -> &T {
        &self.contents[m-1][n-1]
    }

    pub fn mut_at(&mut self, m: usize, n: usize) -> &T {
        &mut self.contents[m-1][n-1]
    }
}

impl<T: Copy> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index<'a>(&'a self, i: usize) -> &'a Vec<T> {
        &self.contents[i]
    }
}

impl<T: Copy> IndexMut<usize> for Matrix<T> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<T> {
        &mut self.contents[i]
    }
}

impl<T: Copy> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {} {} ]", self.height, self.width)
    }
}

impl<T: Copy + Add<Output = T>> Add for Matrix<T> {
    type Output = Result<Self, SizeMismatch>;

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
    pub fn cofactors(&mut self) -> &Self {
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

    pub fn det(&self) -> Result<T, SizeMismatch> {
        if self.height != self.width {
            return Err(SizeMismatch);
        }

        if self.height == 1 {
            return Ok(self[0][0]);
        }

        let mut det: T = (0).into();

        for i in 0..self.width {
            det = det + self.minor(1, i + 1).det().unwrap() * (((i as i32 + 1) % 2) * 2 - 1).into(); // cofactors
        }

        Ok(det)
    }

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
}
