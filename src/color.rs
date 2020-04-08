#[derive(Debug, derive_more::AddAssign)]
pub struct Color(crate::Vector);

impl Color {
    pub fn new(r: crate::NonNegFloat, g: crate::NonNegFloat, b: crate::NonNegFloat) -> Self {
        Self(crate::Vector::new(*r, *g, *b))
    }

    pub fn new_unchecked(r: crate::Float, g: crate::Float, b: crate::Float) -> Self {
        Self(crate::Vector::new(
            *crate::NonNegFloat::new_unchecked(r),
            *crate::NonNegFloat::new_unchecked(g),
            *crate::NonNegFloat::new_unchecked(b),
        ))
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

    pub fn as_rgb(&self, samples_per_pixel: u32) -> image::Rgb<u8> {
        // Divide the colour total by the number of samples and gamma-correct for a gamma value of
        // 2.0.
        let scale = 1.0 / crate::Float::from(samples_per_pixel);
        let r = (scale * self.r()).sqrt();
        let g = (scale * self.g()).sqrt();
        let b = (scale * self.b()).sqrt();

        let r255: crate::Float = 255.0 * num::clamp(r, 0.0, 1.0);
        let g255: crate::Float = 255.0 * num::clamp(g, 0.0, 1.0);
        let b255: crate::Float = 255.0 * num::clamp(b, 0.0, 1.0);

        image::Rgb([r255.round() as u8, g255.round() as u8, b255.round() as u8])
    }
}

impl std::ops::Mul<&Color> for Color {
    type Output = Self;

    fn mul(self, rhs: &Color) -> Self::Output {
        Self(self.0.component_mul(&rhs.0))
    }
}

impl std::ops::Mul<crate::Float> for Color {
    type Output = Self;

    fn mul(self, rhs: crate::Float) -> Self::Output {
        Self(self.0 * rhs)
    }
}
