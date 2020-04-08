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

fn rand_in_unit_disk() -> Vector {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    loop {
        let p = Vector::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);

        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vector, normal: &Vector) -> Vector {
    v - 2.0 * v.dot(normal) * normal
}

fn refract(uv: &Vector, normal: &Vector, etai_over_etat: Float) -> Vector {
    let cos_theta = -uv.dot(normal);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * normal);
    let r_out_perp = -(1.0 - r_out_parallel.norm_squared()).sqrt() * normal;

    r_out_parallel + r_out_perp
}

fn schlick(cosine: Float, refraction_idx: Float) -> Float {
    let r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

mod camera;
mod color;
mod dielectric;
mod hit;
mod hit_list;
mod lambertian;
mod material;
mod metal;
mod non_neg_float;
mod ray;
mod sphere;

use material::Material;

pub use {
    camera::Camera,
    color::Color,
    dielectric::Dielectric,
    hit::{Hit, HitRecord},
    hit_list::HitList,
    lambertian::Lambertian,
    metal::Metal,
    non_neg_float::NonNegFloat,
    ray::Ray,
    sphere::Sphere,
};
