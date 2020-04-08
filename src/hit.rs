#[derive(Debug)]
pub struct HitRecord<'mat> {
    position: crate::Point,
    t: crate::Float,
    normal: crate::Vector,
    front_face: bool,
    material: &'mat dyn crate::Material,
}

impl<'mat> HitRecord<'mat> {
    pub(crate) fn new(
        ray: &crate::Ray,
        position: crate::Point,
        t: crate::Float,
        outward_normal: crate::Vector,
        material: &'mat dyn crate::Material,
    ) -> Self {
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
            material,
        }
    }

    pub fn position(&self) -> &crate::Point {
        &self.position
    }

    pub(crate) fn t(&self) -> crate::Float {
        self.t
    }

    pub fn normal(&self) -> &crate::Vector {
        &self.normal
    }

    pub(crate) fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> &'mat dyn crate::Material {
        self.material
    }
}

pub trait Hit {
    fn hit(
        &self,
        ray: &crate::Ray,
        t_min: crate::Float,
        t_max: crate::Float,
    ) -> Option<HitRecord>;
}
