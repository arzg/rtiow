pub struct Ray {
    origin: crate::Point,
    direction: crate::Vector,
    time: crate::Float,
}

impl Ray {
    pub fn new(origin: crate::Point, direction: crate::Vector, time: crate::Float) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &crate::Point {
        &self.origin
    }

    pub fn direction(&self) -> &crate::Vector {
        &self.direction
    }

    pub(crate) fn time(&self) -> crate::Float {
        self.time
    }

    pub fn at(&self, t: crate::Float) -> crate::Point {
        self.origin + t * self.direction
    }
}
