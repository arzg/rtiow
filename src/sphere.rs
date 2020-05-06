pub struct Sphere {
    center0: crate::Point,
    center1: crate::Point,
    radius: crate::Float,
    material: Box<dyn crate::Material>,
    time0: crate::Float,
    time1: crate::Float,
}

impl Sphere {
    pub fn new(
        center: crate::Point,
        radius: crate::Float,
        material: Box<dyn crate::Material>,
    ) -> Self {
        Self {
            center0: center,
            center1: center,
            radius,
            material,
            time0: crate::float::MIN,
            time1: crate::float::MAX,
        }
    }

    pub fn new_moving(
        center0: crate::Point,
        center1: crate::Point,
        radius: crate::Float,
        material: Box<dyn crate::Material>,
        time0: crate::Float,
        time1: crate::Float,
    ) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        }
    }

    fn center(&self, time: crate::Float) -> crate::Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl crate::Hit for Sphere {
    fn hit(
        &self,
        ray: &crate::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<crate::HitRecord> {
        let center = self.center(ray.time());
        let oc = ray.origin() - center;

        let a = ray.direction().norm_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let t = {
                let t1 = (-half_b - root) / a;
                let t2 = (-half_b + root) / a;

                if t1 < t_max && t1 > t_min {
                    t1
                } else if t2 < t_max && t2 > t_min {
                    t2
                } else {
                    return None;
                }
            };

            let hit_position = ray.at(t);
            let outward_normal = (hit_position - center) / self.radius;

            Some(crate::HitRecord::new(
                ray,
                hit_position,
                t,
                outward_normal,
                &*self.material,
            ))
        } else {
            None
        }
    }
}
