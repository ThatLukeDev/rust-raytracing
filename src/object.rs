use crate::ray::Ray;
use crate::raytrace::Raytrace;
use crate::color::Color;
use crate::tri::Tri;

use std::ops::*;

/// An object.
///
/// Which is stored as a collection of triangles,
/// which each are rendered individually,
/// and treated as one single object.
pub struct Object<T> {
    /// The vec of triangles.
    tris: Vec<Tri<T>>
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Object<T> {
    pub fn from_stl(bytes: Vec<u8>) -> Self {
        todo!()
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Object<T> {
    fn intersects(&self, ray: &Ray<T>) -> (Option<&Tri<T>>, T) {
        let mut lowest: T = 9999999999.0.into();
        let mut closest_tri = None;

        for tri in &self.tris {
            let distance = tri.intersects_along(&ray);
            match distance {
                Some(x) => {
                    if x < lowest {
                        lowest = x;
                        closest_tri = Some(tri);
                    }
                },
                None => ()
            }
        }

        (closest_tri, lowest)
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Object<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        let obj_distance = self.intersects(ray);

        match obj_distance.0 {
            Some(_x) => Some(obj_distance.1),
            None => None
        }
    }

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        self.intersects(ray).0?.transmit(ray)
    }

    fn recolor(&self, _ray: &Ray<T>, color: Color) -> Color {
        self.intersects(_ray).0.unwrap().recolor(_ray, color)
    }
}
