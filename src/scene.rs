use crate::raytrace::*;

use std::ops::*;

struct Scene<T> {
    /// All of the objects throughout the scene.
    ///
    /// Every object must implement the Raytrace trait.
    ///
    /// T represents precision of float used throughout the program.
    objects: Vec<Box<dyn Raytrace<T>>>
}

impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Scene<T> {
}
