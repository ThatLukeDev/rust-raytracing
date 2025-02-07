//! Raytracing in rust.

use rusttracing::image::*;
use rusttracing::color::*;
use rusttracing::vector::*;
use rusttracing::ray::*;
use rusttracing::scene::*;
use rusttracing::camera::*;
use rusttracing::sphere::*;

use std::fs;

/// Command line raytracer
fn main() {
    let scene = Scene::<f64> {
        objects: vec![
            Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Color::new(0.9, 0.1, 0.1))),
            Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Color::new(0.2, 0.8, 0.1)))
        ],

        camera: Camera::new(Vec3::new(0.0, 2.0, -2.0), Vec3::new(-20.0, 0.0, 0.0))
    };

    let mut img: Image<192, 108> = Image::new();

    let aspect_ratio = img.data.len() as f64 / img.data[0].len() as f64;

    let fov: f64 = 90.0;

    let fov_distance = (fov / 2.0).to_radians().tan();

    for x in 0..img.data.len() {
        for y in 0..img.data[x].len() {
            let abs_x = 1.0 - (x as f64 / img.data.len() as f64) * 2.0;
            let abs_y = 1.0 - (y as f64 / img.data[x].len() as f64) * 2.0;

            let camera_ray = Ray::new(
                scene.camera.position,
                scene.camera.transform(Vec3::new(abs_x * aspect_ratio * fov_distance, abs_y * fov_distance, 1.0).unit())
            );

            let camera_color = Color::new_emission(0.9, 0.8, 1.0, 1000.0);

            img[x][y] = scene.raytrace(camera_ray, camera_color, 4, 16) * 0.001;
        }
    }

    let data = img.to_ppm();

    fs::write("image.ppm", data).unwrap();
}
