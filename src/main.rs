const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() -> anyhow::Result<()> {
    let progress_bar = indicatif::ProgressBar::new(u64::from(WIDTH));

    let mut img: image::RgbImage =
        image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgb([0, 0, 0]));

    let lower_left_corner = rtiow::Vector::new(-2.0, -1.0, -1.0);
    let horizontal = rtiow::Vector::new(4.0, 0.0, 0.0);
    let vertical = rtiow::Vector::new(0.0, 2.0, 0.0);
    let origin = rtiow::Point::origin();

    for (y, row) in img.rows_mut().rev().enumerate() {
        for (x, pixel) in row.enumerate() {
            let u = x as f64 / f64::from(WIDTH);
            let v = y as f64 / f64::from(HEIGHT);
            let r = rtiow::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let color = background(&r);
            *pixel = color.into();
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
    img.save("output.png")?;

    Ok(())
}

fn background(ray: &rtiow::Ray) -> rtiow::Color {
    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y + 1.0);

    let vec = (1.0 - t) * rtiow::Vector::new(1.0, 1.0, 1.0) + t * rtiow::Vector::new(0.5, 0.7, 1.0);

    rtiow::Color::new_unchecked(vec.x, vec.y, vec.z)
}
