pub struct Color(crate::Vec3);

impl Color {
    pub fn new(
        r: crate::NonNegFloat,
        g: crate::NonNegFloat,
        b: crate::NonNegFloat,
    ) -> Self {
        Self(crate::Vec3::new(*r, *g, *b))
    }

    fn r(&self) -> crate::Float {
        self.0[0]
    }

    fn g(&self) -> crate::Float {
        self.0[1]
    }

    fn b(&self) -> crate::Float {
        self.0[2]
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
