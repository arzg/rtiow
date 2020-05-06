#[derive(Debug)]
pub struct Lambertian {
    color: crate::Color,
}

impl Lambertian {
    pub fn new(color: crate::Color) -> Self {
        Self { color }
    }
}

impl crate::Material for Lambertian {
    fn scatter(
        &self,
        ray: &crate::Ray,
        hit_record: crate::HitRecord,
    ) -> Option<(crate::Ray, &crate::Color)> {
        let scatter_direction = hit_record.normal() + crate::rand_unit_vector();
        let scattered = crate::Ray::new(*hit_record.position(), scatter_direction, ray.time());

        Some((scattered, &self.color))
    }
}
