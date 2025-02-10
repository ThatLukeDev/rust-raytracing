use crate::raytrace::*;
use crate::vector::*;
use crate::ray::*;
use crate::camera::*;
use crate::color::*;
use crate::image::*;

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
    pub camera: Camera<T>,

    /// The color of the environment
    pub environment: Color,
}

impl<T: Copy + From<f64> + From<i32> + Into<f64> + PartialOrd + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>> Scene<T> {
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

    /// Runs the trace function recurring.
    pub fn trace_bounce(&self, ray: Ray<T>, depth: usize) -> Color {
        if depth == 0 {
            return Color::new(1.0, 1.0, 1.0);
        }

        match self.trace(ray).0 {
            Some(obj) => {
                (*obj).recolor(&ray, self.trace_bounce(obj.transmit(&ray).unwrap(), depth - 1))
            },
            None => self.environment
        }
    }

    /// Runs the trace_bounce function multiple times for each pixel.
    pub fn raytrace<const WIDTH: usize, const HEIGHT: usize>(&self, rays: usize, depth: usize, fov: f64) -> Image<WIDTH, HEIGHT> {
        let mut img: Image<WIDTH, HEIGHT> = Image::new();

        let aspect_ratio = img.data.len() as f64 / img.data[0].len() as f64;
        let fov_distance = (fov / 2.0).to_radians().tan();

        for x in 0..img.data.len() {
            for y in 0..img.data[x].len() {
                let abs_x = 1.0 - (x as f64 / img.data.len() as f64) * 2.0;
                let abs_y = 1.0 - (y as f64 / img.data[x].len() as f64) * 2.0;

                let mut color = Color::new(0.0, 0.0, 0.0);
                let mut bounces = 0.0 as f64;

                for _i in 0..rays {
                    let camera_ray = Ray::new(
                        self.camera.position,
                        self.camera.transform(Vec3::new((abs_x * aspect_ratio * fov_distance).into(), (abs_y * fov_distance).into(), (1.0).into()).unit())
                    );

                    color = color + self.trace_bounce(camera_ray, depth) * 0.001;
                    bounces += 1.0;
                }

                color = color / bounces;

                img[x][y] = color;
            }
        }

        img
    }
}
