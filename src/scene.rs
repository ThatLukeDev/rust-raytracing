use crate::raytrace::*;
use crate::ray::*;
use crate::camera::*;
use crate::color::*;

use std::ops::*;

/// A collection of objects, camera, and background color.
pub struct Scene<T> {
    /// All of the objects throughout the scene.
    ///
    /// Every object must implement the Raytrace trait.
    ///
    /// T represents precision of float used throughout the program.
    pub objects: Vec<Box<dyn Raytrace<T>>>,

    /// The camera of the scene.
    pub camera: Camera<T>
}

impl<T: Copy + From<f64> + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Scene<T> {
    /// Raytracing.
    ///
    /// Traces a ray through the scene, deciding on whether it intersects,
    /// and what it intersects with.
    ///
    /// Returns a tuple of the intersecting object and distance along ray.
    pub fn trace(&self, ray: Ray<T>) -> (Option<&Box<dyn Raytrace<T>>>, T) {
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

    /// Runs the trace function multiple times, and aggregates the corresponding colour.
    pub fn raytrace(&self, ray: Ray<T>, camera_color: Color, rays: usize, depth: usize) -> Color {
        if depth == 0 {
            return Color::new(1.0, 1.0, 1.0);
        }

        let mut color = Color::new(0.0, 0.0, 0.0);
        let mut bounces = 0.0 as f64;

        for _i in 0..rays {
            color = color + match self.trace(ray).0 {
                Some(obj) => {
                    (*obj).recolor(&ray, self.raytrace(obj.transmit(&ray).unwrap(), camera_color, rays, depth - 1))
                },
                None => camera_color
            };

            bounces += 1.0;
        }

        color /= bounces;

        color
    }
}
