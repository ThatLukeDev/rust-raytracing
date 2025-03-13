use crate::vector::Vec3;
use crate::ray::Ray;
use crate::raytrace::Raytrace;
use crate::color::Color;
use crate::tri::Tri;

use std::ops::*;
use std::fmt;

/// An object.
///
/// Which is stored as a collection of triangles,
/// which each are rendered individually,
/// and treated as one single object.
pub struct Object<T> {
    /// The vec of triangles.
    tris: Vec<Tri<T>>
}

#[derive(Clone, Copy, Debug)]
/// Unsupported stl type.
pub struct UnsupportedError;
impl fmt::Display for UnsupportedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unsupported stl version, please use binary stl")
    }
}

const HEADER_LEN: usize = 80;
const PRECISION_LEN: usize = 4;
const ATTR_LEN: usize = 2;

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Object<T> {
    /// Creates a blank object.
    pub fn new() -> Self {
        Object::<_> { tris: vec![] }
    }

    fn vec3_from_f32(slice: &[u8]) -> Vec3<T> {
        let x = f32::from_le_bytes(slice[(PRECISION_LEN*0)..(PRECISION_LEN*1)].try_into().unwrap()) as f64;
        let y = f32::from_le_bytes(slice[(PRECISION_LEN*1)..(PRECISION_LEN*2)].try_into().unwrap()) as f64;
        let z = f32::from_le_bytes(slice[(PRECISION_LEN*2)..(PRECISION_LEN*3)].try_into().unwrap()) as f64;

        Vec3::new(x.into(), y.into(), z.into())
    }

    /// Creates a new object from a byte array, in the stl format.
    ///
    /// # Errors
    /// Will error when used with ascii stl (old, outdated version).
    pub fn from_stl(bytes: Vec<u8>, color: Color, roughness: f64) -> Result<Self, UnsupportedError> where f64: From<T> {
        // Do not allow ascii stl
        if &bytes[0..5] == "solid".as_bytes() {
            return Err(UnsupportedError);
        }

        let length = u32::from_le_bytes(bytes[(HEADER_LEN)..(HEADER_LEN + PRECISION_LEN)].try_into().unwrap());

        // Starts after header, which is of length 80, and after length, of length 4
        let mut head = HEADER_LEN + PRECISION_LEN;

        let mut out = Self::new();

        for _i in 0..length {
            let _normal = Self::vec3_from_f32(&bytes[(head)..(head + PRECISION_LEN * 3)]).unit();
            head += PRECISION_LEN * 3;

            let p1 = Self::vec3_from_f32(&bytes[(head)..(head + PRECISION_LEN * 3)]).unit();
            head += PRECISION_LEN * 3;

            let p2 = Self::vec3_from_f32(&bytes[(head)..(head + PRECISION_LEN * 3)]).unit();
            head += PRECISION_LEN * 3;

            let p3 = Self::vec3_from_f32(&bytes[(head)..(head + PRECISION_LEN * 3)]).unit();
            head += PRECISION_LEN * 3;

            out.tris.push(
                Tri::new(p1, p2, p3, color, roughness)
            );
            head += ATTR_LEN;
        }

        Ok(out)
    }

    pub fn translate(mut self, offset: Vec3<T>) -> Self where f64: From<T> {
        for i in 0..self.tris.len() {
            self.tris[i] = Tri::new(
                self.tris[i].bounds.x + offset,
                self.tris[i].bounds.y + offset,
                self.tris[i].bounds.z + offset,
                self.tris[i].plane.color,
                self.tris[i].plane.roughness,
            );
        }

        self
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Object<T> {
    fn intersects(&self, ray: &Ray<T>) -> (Option<&Tri<T>>, T) {
        let mut lowest: T = 9999999999.0.into();
        let mut closest_tri = None;

        for tri in &self.tris {
            let distance = tri.intersects_along(&ray);
            match distance {
                Some(x) => {
                    if x < lowest {
                        lowest = x;
                        closest_tri = Some(tri);
                    }
                },
                None => ()
            }
        }

        (closest_tri, lowest)
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Raytrace<T> for Object<T> {
    fn intersects_along(&self, ray: &Ray<T>) -> Option<T> {
        let obj_distance = self.intersects(ray);

        match obj_distance.0 {
            Some(_x) => Some(obj_distance.1),
            None => None
        }
    }

    fn transmit(&self, ray: &Ray<T>) -> Option<Ray<T>> {
        self.intersects(ray).0?.transmit(ray)
    }

    fn recolor(&self, _ray: &Ray<T>, color: Color) -> Color {
        self.intersects(_ray).0.unwrap().recolor(_ray, color)
    }
}
