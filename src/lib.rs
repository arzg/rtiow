type Float = f64;
pub type Vector = nalgebra::Vector3<Float>;
pub type Point = nalgebra::Point3<Float>;

mod color;
mod non_neg_float;
mod ray;

pub use {color::Color, non_neg_float::NonNegFloat, ray::Ray};
