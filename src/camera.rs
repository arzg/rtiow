pub struct Camera {
    lower_left_corner: crate::Vector,
    horizontal: crate::Vector,
    vertical: crate::Vector,
    origin: crate::Point,
}

impl Camera {
    pub fn new(vertical_fov: crate::Float, aspect: crate::Float) -> Self {
        let theta = vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        Self {
            lower_left_corner: crate::Vector::new(-half_width, -half_height, -1.0),
            horizontal: crate::Vector::new(2.0 * half_width, 0.0, 0.0),
            vertical: crate::Vector::new(0.0, 2.0 * half_height, 0.0),
            origin: crate::Point::origin(),
        }
    }

    pub fn ray(&self, u: crate::Float, v: crate::Float) -> crate::Ray {
        crate::Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin.coords,
        )
    }
}
