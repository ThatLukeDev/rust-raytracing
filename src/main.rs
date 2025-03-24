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
            // Ground
            Box::new(Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0, Color::new(0.1, 0.9, 0.1))),

            // Light
            Box::new(Sphere::<_> {
                origin: Vec3::new(-1.0, 16.0, 1.0),
                radius: 2.0,
                color: Color::new_emission(0.2, 0.2, 0.9, 100.0),
                roughness: 0.2,
            }),

            // Cube
            Box::new(Object::new_box(Vec3::new(2.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0), Color::new(0.9, 0.1, 0.9), 1.0)),

            // Sphere
            Box::new(Sphere::<_> {
                origin: Vec3::new(0.0, 1.0, -1.0),
                radius: 1.0,
                color: Color::new(0.2, 0.2, 0.9),
                roughness: 0.2,
            }),

            // Head
            Box::new(
                Object::from_stl(
                    include_bytes!("../head.stl").to_vec(),
                    Color::new(0.9, 0.9, 0.9),
                    1.0,
                ).unwrap()
                    .unit()
                    .rotate(Vec3::new(-90.0, 0.0, 180.0))
                    .translate(Vec3::new(0.0, 3.0, 1.0))
            ),

            // Mirror
            Box::new(Tri::new(
                Vec3::new(-2.0, 0.0, -5.0),
                Vec3::new(-2.0, 0.0, 5.0),
                Vec3::new(-2.0, 5.0, 5.0),
                Color::new(0.9, 0.9, 0.9),
                0.01,
            )),
            Box::new(Tri::new(
                Vec3::new(-2.0, 5.0, -5.0),
                Vec3::new(-2.0, 0.0, -5.0),
                Vec3::new(-2.0, 5.0, 5.0),
                Color::new(0.9, 0.9, 0.9),
                0.01,
            )),
        ],

        environment: Color::new_emission(0.9, 0.8, 1.0, 1000.0),

        camera: Camera::new(Vec3::new(2.0, 4.0, -2.0), Vec3::new(-45.0, -45.0, 0.0)),
    };

    const WIDTH: usize = 192*2;
    const HEIGHT: usize = 108*2;
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
