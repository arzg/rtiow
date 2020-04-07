#[derive(Default)]
pub struct HitList {
    items: Vec<Box<dyn crate::Hit>>,
}

impl HitList {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, item: Box<dyn crate::Hit>) {
        self.items.push(item);
    }
}

impl crate::Hit for HitList {
    fn hit(
        &self,
        ray: &crate::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for item in &self.items {
            if let Some(item_record) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = item_record.t();
                hit_record = Some(item_record);
            }
        }

        hit_record
    }
}
