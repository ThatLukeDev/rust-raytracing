use crate::ray::Ray;

pub trait Raytrace<T: Into<f64>> {
    fn intersectsAt(ray: &Ray<T>);
}
