use std::ops::Deref;

pub struct NonNegFloat(crate::Float);

impl NonNegFloat {
    pub fn new(x: crate::Float) -> Option<Self> {
        if x >= 0.0 {
            Some(Self(x))
        } else {
            None
        }
    }

    pub fn new_unchecked(x: crate::Float) -> Self {
        Self::new(x).unwrap()
    }
}

impl Deref for NonNegFloat {
    type Target = crate::Float;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
