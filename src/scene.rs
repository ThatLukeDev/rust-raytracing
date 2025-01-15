use crate::raytrace::*;

struct Scene<T> {
    // T represents precision of float used throughout the program
    objects: Vec<Box<dyn Raytrace<T>>>
}
