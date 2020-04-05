type Float = f64;
pub type Vector = nalgebra::Vector3<Float>;

mod color;
mod non_neg_float;

pub use {color::Color, non_neg_float::NonNegFloat};
