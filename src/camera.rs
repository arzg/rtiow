pub struct Camera {
    lower_left_corner: crate::Vector,
    horizontal: crate::Vector,
    vertical: crate::Vector,
    origin: crate::Point,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left_corner: crate::Vector::new(-2.0, -1.0, -1.0),
            horizontal: crate::Vector::new(4.0, 0.0, 0.0),
            vertical: crate::Vector::new(0.0, 2.0, 0.0),
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

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
