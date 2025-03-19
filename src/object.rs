use crate::vector::Vec3;
use crate::ray::Ray;
use crate::matrix::Matrix;
use crate::raytrace::Raytrace;
use crate::color::Color;
use crate::tri::Tri;

use std::ops::*;
use std::fmt;

macro_rules! offset_point_tri_helix {
    ( $pos: ident, $size: ident, $col: ident, $noise: ident, $t: ty, $( $p1: expr, $p2: expr, $p3: expr, $p4: expr, $p5: expr, $p6: expr, $p7: expr, $p8: expr, $p9: expr );+ ) => {
        vec![ $( Tri::new($pos + Vec3::new(<_ as Into<$t>>::into($p1) * $size.x, <_ as Into<$t>>::into($p2) * $size.y, <_ as Into<$t>>::into($p3) * $size.z), $pos + Vec3::new(<_ as Into<$t>>::into($p4) * $size.x, <_ as Into<$t>>::into($p5) * $size.y, <_ as Into<$t>>::into($p6) * $size.z), $pos + Vec3::new(<_ as Into<$t>>::into($p7) * $size.x, <_ as Into<$t>>::into($p8) * $size.y, <_ as Into<$t>>::into($p9) * $size.z), $col, $noise) ),+ ]
    }
}

macro_rules! offset_point_tri_cube {
    ( $pos: ident, $size: ident, $col: ident, $noise: ident, $t: ty ) => {
        offset_point_tri_helix!($pos, $size, $col, $noise, $t,
            -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0;
            -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0;
            -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0;
            -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0;
            1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0;
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0;
            -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0;
            -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0;
            1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0;
            -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0;
            1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0;
            -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0
        )
    }
}

macro_rules! fast_transform {
    ($vec: expr, $mat: expr) => {
        Vec3::new(
            $mat[0][0] * $vec.x + $mat[0][1] * $vec.y + $mat[0][2] * $vec.z,
            $mat[1][0] * $vec.x + $mat[1][1] * $vec.y + $mat[1][2] * $vec.z,
            $mat[2][0] * $vec.x + $mat[2][1] * $vec.y + $mat[2][2] * $vec.z,
        )
    }
}

/// An object.
///
/// Which is stored as a collection of triangles,
/// which each are rendered individually,
/// and treated as one single object.
pub struct Object<T> {
    /// The vec of triangles.
    pub tris: Vec<Tri<T>>,

    /// Cache for the bounding box of the object.
    ///
    /// # Errors
    ///
    /// Bounds cache must be recalculated or cleared upon mutating the object through a non built-in method:
    /// `obj.recalculate_bounds()`
    /// `obj.bounds_cache = None` (SLOW)
    pub bounds_cache: Option<Vec<Tri<T>>>
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
        Object::<_> { tris: vec![], bounds_cache: None }
    }

    /// Recalculates the bounds of the object.
    fn recalculate_bounds(&mut self) where f64: From<T> {
        let bounds = self.bounds();

        let centre = (bounds.0 + bounds.1) * <_ as Into<T>>::into(0.5);
        let size = bounds.1 - bounds.0;

        let col = Color::new(0.0, 0.0, 0.0);
        let noi = 0.0;

        self.bounds_cache = Some(offset_point_tri_cube!(centre, size, col, noi, T));
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

        out.recalculate_bounds();

        Ok(out)
    }

    /// Returns a box, with center at origin.
    pub fn new_box(origin: Vec3<T>, size: Vec3<T>, color: Color, roughness: f64) -> Self where T: Neg<Output = T>, f64: From<T> {
        let size = size * <_ as Into<T>>::into(0.5);

        Object::<_> {
            tris: offset_point_tri_cube!(origin, size, color, roughness, T),
            bounds_cache: None
        }
    }

    /// Returns the bounding box of the object.
    pub fn bounds(&self) -> (Vec3<T>, Vec3<T>) {
        let mut min: Vec3<T> = Vec3::new(<_ as Into<T>>::into(1000000.0), <_ as Into<T>>::into(1000000.0), <_ as Into<T>>::into(1000000.0));
        let mut max: Vec3<T> = Vec3::new(<_ as Into<T>>::into(-1000000.0), <_ as Into<T>>::into(-1000000.0), <_ as Into<T>>::into(-1000000.0));

        for tri in &self.tris {
            for i in 0..3 {
                let point = match i {
                    0 => tri.bounds.x,
                    1 => tri.bounds.y,
                    _ => tri.bounds.z,
                };

                if point.x < min.x {
                    min.x = point.x;
                }
                if point.y < min.y {
                    min.y = point.y;
                }
                if point.z < min.z {
                    min.z = point.z;
                }

                if point.x > max.x {
                    max.x = point.x;
                }
                if point.y > max.y {
                    max.y = point.y;
                }
                if point.z > max.z {
                    max.z = point.z;
                }
            }
        }

        (min, max)
    }

    /// Recenters an object and scales it such that it is 1 unit high on its largest axis.
    pub fn unit(mut self) -> Self where f64: From<T>, T: From<i32> + std::cmp::PartialOrd + Neg<Output = T> {
        let bounds = self.bounds();

        let negative_centre = (bounds.0 + bounds.1) * <_ as Into<T>>::into(-0.5);

        self = self.translate(negative_centre);

        let mut lengths = bounds.1 - bounds.0;

        let zero: T = <_ as Into<T>>::into(0.0);

        if lengths.x < zero {
            lengths.x = -lengths.x;
        }
        if lengths.y < zero {
            lengths.y = -lengths.y;
        }
        if lengths.z < zero {
            lengths.z = -lengths.z;
        }

        let mut length = lengths.x;
        if lengths.y > length {
            length = lengths.y;
        }
        if lengths.z > length {
            length = lengths.z;
        }

        self = self.scale(Vec3::new(<_ as Into<T>>::into(1.0) / length, <_ as Into<T>>::into(1.0) / length, <_ as Into<T>>::into(1.0) / length));

        self
    }

    /// Moves all tris in an object by a set vector.
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

        self.recalculate_bounds();

        self
    }

    /// Transforms the object with a Matrix transformation.
    pub fn transform(mut self, mat: Matrix<T>) -> Self where f64: From<T> {
        for i in 0..self.tris.len() {
            self.tris[i] = Tri::new(
                fast_transform!(self.tris[i].bounds.x, mat),
                fast_transform!(self.tris[i].bounds.y, mat),
                fast_transform!(self.tris[i].bounds.z, mat),
                self.tris[i].plane.color,
                self.tris[i].plane.roughness,
            );
        }

        self.recalculate_bounds();

        self
    }

    /// Scale all tris in an object by a set vector from the origin.
    pub fn scale(mut self, factor: Vec3<T>) -> Self where f64: From<T>, T: From<i32> {
        let mut mat = Matrix::ident(3);
        mat[0][0] = factor.x;
        mat[1][1] = factor.y;
        mat[2][2] = factor.z;

        self = self.transform(mat);

        self
    }

    /// Rotates an object by a rotation vector.
    pub fn rotate(mut self, rot: Vec3<T>) -> Self where f64: From<T>, T: From<i32> + Neg<Output = T> {
        let mat = <Vec3<T> as Into<Matrix<T>>>::into(rot);

        self = self.transform(mat);

        self
    }
}

impl<T: PartialOrd + From<f64> + Into<f64> + Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T>> Object<T> {
    fn intersects(&self, ray: &Ray<T>) -> (Option<&Tri<T>>, T) {
        match &self.bounds_cache {
            Some(bound) => {
                let mut within_bounds = false;
                for tri in bound {
                    if tri.intersects_along(&ray).is_some() {
                        within_bounds = true;
                        break;
                    }
                }
                if !within_bounds {
                    return (None, <_ as Into<T>>::into(0.0));
                }
            },
            None => ()
        }

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
