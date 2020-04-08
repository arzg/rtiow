pub struct Camera {
    lower_left_corner: crate::Vector,
    horizontal: crate::Vector,
    vertical: crate::Vector,
    origin: crate::Point,
}

impl Camera {
    pub fn new(look_from: crate::Vector, look_at: crate::Vector, vup: crate::Vector, vertical_fov: crate::Float, aspect: crate::Float) -> Self {
        let theta = vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        Self {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from.into(),
        }
    }

    pub fn ray(&self, u: crate::Float, v: crate::Float) -> crate::Ray {
        crate::Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin.coords,
        )
    }
}
