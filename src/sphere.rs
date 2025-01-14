use crate::vector::Vec3;
use crate::ray::Ray;

pub struct Sphere<T> {
    origin: Vec3<T>,
    radius: T
}

impl<T> Sphere<T> {
    pub fn new(origin: Vec3<T>, radius: T) -> Self {
        Sphere::<T> { origin, radius }
    }
}
