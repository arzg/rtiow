#[derive(Debug)]
pub struct Color(crate::Vector);

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
            (255.0 * color.r()).round() as u8,
            (255.0 * color.g()).round() as u8,
            (255.0 * color.b()).round() as u8,
        ])
    }
}
