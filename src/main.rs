//! Raytracing in rust.

use rusttracing::image::*;
use rusttracing::color::*;
use rusttracing::vector::*;
use rusttracing::ray::*;
use rusttracing::scene::*;
use rusttracing::camera::*;

use std::fs;

/// Command line raytracer
fn main() {
    let scene = Scene::<f64> {
        objects: vec![
        ],

        camera: Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0))
    };

    let mut img: Image<192, 108> = Image::new();

    for x in 0..img.data.len() {
        for y in 0..img.data[x].len() {
            let percent_x = x as f64 / img.data.len() as f64;
            let percent_y = y as f64 / img.data[x].len() as f64;
            img[x][y] = match scene.trace(Ray::new(
                scene.camera.position,
                scene.camera.transform(Vec3::new(0.5 - percent_x, 0.5 - percent_y, 1.0).unit())
            )).0 {
                Some(x) => x.into_raw(),
                None => Color::new(0.0, 0.0, 0.0)
            };
        }
    }

    let data = img.to_ppm();

    fs::write("image.ppm", data).unwrap();
}
