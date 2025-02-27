//! Raytracing in rust.

use std::time::Instant;

use std::thread;
use std::sync::mpsc;

use rusttracing::color::*;
use rusttracing::vector::*;
use rusttracing::scene::*;
use rusttracing::camera::*;
use rusttracing::sphere::*;
use rusttracing::image::*;

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

    const WIDTH: usize = 192;
    const HEIGHT: usize = 108;
    const SAMPLES: usize = 256;
    const FOV: f64 = 90.0;

    let start = Instant::now();
    println!("Starting render");

    let mut img: Image<WIDTH, HEIGHT> = Image::new();
    let mut counter = 0;
    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();
        s.spawn(|| {
            img = scene.raytrace::<WIDTH, HEIGHT>(SAMPLES, 16, FOV, Some(tx));
        });
        for batch in rx {
            counter += 1;

            println!("{}% complete ({}/{})", counter * 100 / WIDTH, counter, WIDTH);
        }
    });

    let time = start.elapsed();
    println!("Rendering took {}ms", time.as_millis());

    let data = img.to_ppm();
    fs::write("image.ppm", data).unwrap();
}
