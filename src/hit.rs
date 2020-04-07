#[derive(Debug)]
pub struct HitRecord {
    position: crate::Point,
    t: crate::Float,
    normal: crate::Vector,
    front_face: bool,
}

impl HitRecord {
    pub(crate) fn new(ray: &crate::Ray, position: crate::Point, t: crate::Float, outward_normal: crate::Vector) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            position,
            t,
            normal,
            front_face,
        }
    }

    pub(crate) fn t(&self) -> crate::Float {
        self.t
    }

    pub fn normal(&self) -> &crate::Vector {
        &self.normal
    }
}

pub trait Hit {
    fn hit(&self, ray: &crate::Ray, t_min: crate::Float, t_max: crate::Float) -> Option<HitRecord>;
}
