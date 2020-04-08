#[derive(Debug)]
pub struct Metal {
    color: crate::Color,
    fuzz: crate::Float,
}

impl Metal {
    pub fn new(color: crate::Color, fuzz: crate::Float) -> Self {
        Self { color, fuzz }
    }
}

impl crate::Material for Metal {
    fn scatter(
        &self,
        ray: &crate::Ray,
        hit_record: crate::HitRecord,
    ) -> Option<(crate::Ray, &crate::Color)> {
        let reflected = crate::reflect(&ray.direction().normalize(), hit_record.normal());
        let scattered = crate::Ray::new(*hit_record.position(), reflected + self.fuzz * crate::rand_unit_vector());

        if scattered.direction().dot(hit_record.normal()) > 0.0 {
            Some((scattered, &self.color))
        } else {
            None
        }
    }
}
