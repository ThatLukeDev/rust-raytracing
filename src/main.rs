//! Raytracing in rust.

use rusttracing::image::*;
use rusttracing::color::*;

use std::fs;

/// Command line raytracer
fn main() {
    let mut img: Image<192, 108> = Image::new();

    for x in 0..img.data.len() {
        for y in 0..img.data[x].len() {
            let percent_x = x as f64 / img.data.len() as f64;
            let percent_y = y as f64 / img.data[x].len() as f64;
            img[x][y] = Color::new(
                1.0 - percent_x,
                percent_x,
                percent_y
            );
        }
    }

    let data = img.to_ppm();

    fs::write("image.ppm", data).unwrap();
}
