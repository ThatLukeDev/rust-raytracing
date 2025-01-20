use std::ops::*;
use std::fmt;
use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct SizeMismatch;

impl fmt::Display for SizeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid size(s) for matrix")
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T: Copy> {
    height: usize,
    width: usize,

    contents: Vec<Vec<T>>
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
