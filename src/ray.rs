pub struct Ray {
    origin: crate::Point,
    direction: crate::Vector,
}

impl Ray {
    pub fn new(origin: crate::Point, direction: crate::Vector) -> Self {
        Self { origin, direction }
    }

    pub fn direction(&self) -> &crate::Vector {
        &self.direction
    }
}
