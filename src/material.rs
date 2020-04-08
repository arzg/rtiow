pub trait Material: std::fmt::Debug {
    fn scatter(
        &self,
        ray: &crate::Ray,
        hit_record: crate::HitRecord,
    ) -> Option<(crate::Ray, &crate::Color)>;
}
