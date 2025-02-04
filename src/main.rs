//! Raytracing in rust.

use rusttracing::image::*;
use rusttracing::color::*;

use std::fs;

/// Command line raytracer
fn main() {
    let mut img: Image<192, 108> = Image::new();

    for x in 0..img.data.len() {
        for y in 0..img.data[x].len() {
            img[x][y] = Color::new(x as f64, y as f64, 0.0);
        }
    }

    let data = img.to_ppm();

    fs::write("image.ppm", data).unwrap();
}
