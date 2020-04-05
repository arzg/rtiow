pub struct Color(crate::Vec3);

impl Color {
    pub fn new(r: crate::NonNegFloat, g: crate::NonNegFloat, b: crate::NonNegFloat) -> Self {
        Self(crate::Vector::new(*r, *g, *b))
    }

    fn r(&self) -> crate::Float {
        self.0.x
    }

    fn g(&self) -> crate::Float {
        self.0.y
    }

    fn b(&self) -> crate::Float {
        self.0.z
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        Self([
            color.r().round() as u8,
            color.g().round() as u8,
            color.b().round() as u8,
        ])
    }
}
