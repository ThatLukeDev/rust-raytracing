use std::ops::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SizeMismatch;

impl fmt::Display for SizeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid size(s) for matrix")
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    pub height: usize,
    pub width: usize,

    pub contents: Vec<Vec<T>>
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
    ($($x:expr),+; $($($y:expr),+);+) => (count_args!($($x),+) + count_expr!($($($y),+);+));
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
    ( $($($element:expr),+);+ ) => {
        Matrix {
            width: count_expr_args!( $($($element),+);+ ),
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
        &self.contents[m][n]
    }

    pub fn mut_at(&mut self, m: usize, n: usize) -> &T {
        &mut self.contents[m][n]
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
                    result[i][j] = result[i][j] + self[i][k] * other[k][i];
                }
            }
        }

        Ok(result)
    }
}
