pub struct Ray {
    origin: crate::Point,
    direction: crate::Vector,
}

impl Ray {
    pub fn new(origin: crate::Point, direction: crate::Vector) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &crate::Point {
        &self.origin
    }

    pub fn direction(&self) -> &crate::Vector {
        &self.direction
    }

    pub fn at(&self, t: crate::Float) -> crate::Point {
        self.origin + t * self.direction
    }
}
