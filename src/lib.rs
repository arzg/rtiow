pub type Float = f64;
pub use std::f64 as float;
pub type Vector = nalgebra::Vector3<Float>;
pub type Point = nalgebra::Point3<Float>;

fn rand_unit_vector() -> Vector {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let a = rng.gen_range(0.0, 2.0 * float::consts::PI);
    let z = rng.gen_range(-1.0, 1.0);
    let r = (1.0 as Float - z * z).sqrt();

    Vector::new(r * a.cos(), r * a.sin(), z)
}

fn reflect(v: &Vector, n: &Vector) -> Vector {
    v - 2.0 * v.dot(n) * n
}

mod camera;
mod color;
mod hit;
mod metal;
mod hit_list;
mod lambertian;
mod material;
mod non_neg_float;
mod ray;
mod sphere;

use material::Material;

pub use {
    camera::Camera,
    color::Color,
    hit::{Hit, HitRecord},
    hit_list::HitList,
    lambertian::Lambertian,
    non_neg_float::NonNegFloat,
    ray::Ray,
    sphere::Sphere,
    metal::Metal,
};
