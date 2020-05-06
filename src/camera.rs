pub struct Camera {
    lower_left_corner: crate::Vector,
    horizontal: crate::Vector,
    vertical: crate::Vector,
    origin: crate::Point,
    u: crate::Vector,
    v: crate::Vector,
    lens_radius: crate::Float,
    time0: crate::Float,
    time1: crate::Float,
}

impl Camera {
    pub fn new(
        look_from: crate::Vector,
        look_at: crate::Vector,
        vup: crate::Vector,
        vertical_fov: crate::Float,
        aspect: crate::Float,
        aperture: crate::Float,
        focus_dist: crate::Float,
        time0: crate::Float,
        time1: crate::Float,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        Self {
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from.into(),
            u,
            v,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }

    pub fn ray(&self, s: crate::Float, t: crate::Float, rng: &mut impl rand::Rng) -> crate::Ray {
        let rd = self.lens_radius * crate::rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        crate::Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin.coords
                - offset,
            rng.gen_range(self.time0, self.time1),
        )
    }
}
