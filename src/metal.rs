#[derive(Debug)]
pub struct Metal {
    color: crate::Color,
}

impl Metal {
    pub fn new(color: crate::Color) -> Self {
        Self { color }
    }
}

impl crate::Material for Metal {
    fn scatter(
        &self,
        ray: &crate::Ray,
        hit_record: crate::HitRecord,
    ) -> Option<(crate::Ray, &crate::Color)> {
        let reflected = crate::reflect(&ray.direction().normalize(), hit_record.normal());
        let scattered = crate::Ray::new(*hit_record.position(), reflected);
        let attenuation = &self.color;

        if scattered.direction().dot(hit_record.normal()) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
