//! Raytracing in rust.

use std::time::Instant;

use rusttracing::color::*;
use rusttracing::vector::*;
use rusttracing::scene::*;
use rusttracing::camera::*;
use rusttracing::sphere::*;

use std::fs;

/// Command line raytracer
fn main() {
    let scene = Scene::<f64> {
        objects: vec![
            // Light
            Box::new(Sphere::new(Vec3::new(2.0, 10.0, 2.0), 2.0, Color::new_emission(0.9, 0.9, 1.0, 20.0))),

            // Ball
            Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Color::new(0.9, 0.1, 0.1))),

            // Ground
            Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Color::new(0.2, 0.8, 0.1))),
        ],

        environment: Color::new_emission(0.9, 0.8, 1.0, 500.0),

        camera: Camera::new(Vec3::new(0.0, 2.0, -2.0), Vec3::new(-20.0, 0.0, 0.0)),
    };

    let start = Instant::now();
    println!("Starting render");

    let img = scene.raytrace::<192, 108>(256, 16, 90.0);

    let time = start.elapsed();
    println!("Rendering took {}ms", time.as_millis());

    let data = img.to_ppm();
    fs::write("image.ppm", data).unwrap();
}
