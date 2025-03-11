//! Raytracing in rust.

use std::time::Instant;

use std::thread;
use std::sync::mpsc;

use rusttracing::color::*;
use rusttracing::vector::*;
use rusttracing::scene::*;
use rusttracing::camera::*;
use rusttracing::sphere::*;
use rusttracing::plane::*;
use rusttracing::tri::*;
use rusttracing::object::*;
use rusttracing::image::*;

use std::fs;

/// Command line raytracer
fn main() {
    let scene = Scene::<f64> {
        objects: vec![
            // Ball
            Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Color::new(0.9, 0.1, 0.1))),

            // Ground
            Box::new(Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0, Color::new(0.1, 0.9, 0.1))),

            // Mirror
            Box::new(Tri::new(
                Vec3::new(-2.0, 0.0, -5.0),
                Vec3::new(-2.0, 0.0, 5.0),
                Vec3::new(-2.0, 5.0, 5.0),
                Color::new(0.9, 0.9, 0.9),
                0.01,
            )),

            // Head
            /*
            Box::new(Object::from_stl(
                include_bytes!("head.stl")
            )),
            */
        ],

        environment: Color::new_emission(0.9, 0.8, 1.0, 1000.0),

        camera: Camera::new(Vec3::new(0.0, 2.0, -2.0), Vec3::new(-20.0, 0.0, 0.0)),
    };

    const WIDTH: usize = 192;
    const HEIGHT: usize = 108;
    const SAMPLES: usize = 256;
    const FOV: f64 = 110.0;

    let start = Instant::now();
    println!("Starting render");

    let mut img: Image<WIDTH, HEIGHT> = Image::new();
    let mut counter = 0;
    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();
        s.spawn(|| {
            img = scene.raytrace::<WIDTH, HEIGHT>(SAMPLES, 16, FOV, Some(tx));
        });
        for _batch in rx {
            counter += 1;

            println!("{}% complete ({}/{})", counter * 100 / WIDTH, counter, WIDTH);
        }
    });

    let time = start.elapsed();
    println!("Rendering took {}ms", time.as_millis());

    let data = img.to_ppm();
    fs::write("image.ppm", data).unwrap();
}
