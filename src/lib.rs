#![warn(missing_docs)]

#![allow(dead_code)]

//! A library consisting of raytracing utility functions.

/// A vector in 3 dimension, and utility functions.
pub mod vector;

/// A ray in 3 dimensions, and utility functions.
pub mod ray;

/// A trait implemented for all scene objects, taking a ray and an object.
pub mod raytrace;

/// A sphere, with the Raytrace trait.
pub mod sphere;

/// A plane, with the Raytrace trait.
pub mod plane;

/// A triangle, storing a plane, with the Raytrace trait.
pub mod tri;

/// A camera, consisting of an origin and a rotation matrix.
pub mod camera;

/// A scene, containing a list of objects, and configurations.
///
/// Supports raytracing of the scene, with async recieve.
pub mod scene;

/// A matrix, of any size, and utility functions.
pub mod matrix;

/// A color, in RGB.
pub mod color;

/// A 2D array of colors.
pub mod image;
