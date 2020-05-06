#[derive(Debug)]
pub struct Dielectric {
    color: crate::Color,
    refraction_idx: crate::Float,
}

impl Dielectric {
    pub fn new(refraction_idx: crate::Float) -> Self {
        Self {
            color: crate::Color::new_unchecked(1.0, 1.0, 1.0),
            refraction_idx,
        }
    }
}

impl crate::Material for Dielectric {
    fn scatter(
        &self,
        ray: &crate::Ray,
        hit_record: crate::HitRecord,
    ) -> Option<(crate::Ray, &crate::Color)> {
        use rand::Rng;

        let etai_over_etat = if hit_record.front_face() {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // We put this in a closure to avoid having to compute it in case we don’t need it.
        let reflect_probability = || crate::schlick(cos_theta, etai_over_etat);
        let mut rng = rand::thread_rng();

        let scattered_vec = if etai_over_etat * sin_theta > 1.0
            || rng.gen_range(0.0, 1.0) < reflect_probability()
        {
            crate::reflect(&unit_direction, hit_record.normal())
        } else {
            crate::refract(&unit_direction, hit_record.normal(), etai_over_etat)
        };

        let scattered = crate::Ray::new(*hit_record.position(), scattered_vec, ray.time());

        Some((scattered, &self.color))
    }
}
