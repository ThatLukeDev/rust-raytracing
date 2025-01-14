use crate::vector::Vec3;

use std::fmt;

use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>
}
