use crate::raytrace::*;
use crate::ray::*;

use std::ops::*;

struct Scene<T> {
    /// All of the objects throughout the scene.
    ///
    /// Every object must implement the Raytrace trait.
    ///
    /// T represents precision of float used throughout the program.
    objects: Vec<Box<dyn Raytrace<T>>>
}

impl<T: Copy + From<f64> + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Scene<T> {
    /// Raytracing.
    ///
    /// Traces a ray through the scene, deciding on whether it intersects,
    /// and what it intersects with.
    ///
    /// Returns a tuple of the intersecting object and distance along ray.
    fn trace(&self, ray: Ray<T>) -> (Option<&Box<dyn Raytrace<T>>>, T) {
        let mut lowest: T = 9999999999.0.into();
        let mut closest_obj: Option<&Box<dyn Raytrace<T>>> = None;

        for obj in &self.objects {
            let distance = obj.intersects_along(&ray);
            match distance {
                Some(x) => {
                    if x < lowest {
                        lowest = x;
                        closest_obj = Some(obj);
                    }
                },
                None => ()
            }
        }

        (closest_obj, lowest)
    }
}
