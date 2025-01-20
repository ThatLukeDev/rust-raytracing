use std::ops::*;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>
    where T: Copy + Add + Sub + Mul + Div {
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
}

impl<T: Copy + Add + Sub + Mul + Div> Index<usize> for Matrix<T> {
    type Output = Vec<T>;

    fn index<'a>(&'a self, i: usize) -> &'a Vec<T> {
        &self.contents[i]
    }
}

impl<T: Copy + Add + Sub + Mul + Div> IndexMut<usize> for Matrix<T> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<T> {
        &mut self.contents[i]
    }
}
