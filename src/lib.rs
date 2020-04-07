pub type Float = f64;
pub type Vector = nalgebra::Vector3<Float>;
pub type Point = nalgebra::Point3<Float>;

mod camera;
mod color;
mod hit;
mod hit_list;
mod non_neg_float;
mod ray;
mod sphere;

pub use {
    camera::Camera,
    color::Color,
    hit::{Hit, HitRecord},
    hit_list::HitList,
    non_neg_float::NonNegFloat,
    ray::Ray,
    sphere::Sphere,
};
