type Float = f64;
type Vec3 = nalgebra::Vector3<Float>;

mod color;
mod non_neg_float;

pub use {color::Color, non_neg_float::NonNegFloat};
