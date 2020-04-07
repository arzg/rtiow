const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() -> anyhow::Result<()> {
    let progress_bar = indicatif::ProgressBar::new(u64::from(HEIGHT));

    let mut img: image::RgbImage =
        image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgb([0, 0, 0]));

    let lower_left_corner = rtiow::Vector::new(-2.0, -1.0, -1.0);
    let horizontal = rtiow::Vector::new(4.0, 0.0, 0.0);
    let vertical = rtiow::Vector::new(0.0, 2.0, 0.0);
    let origin = rtiow::Point::origin();

    let mut world = rtiow::HitList::new();

    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(0.0, -100.5, -1.0),
        100.0,
    )));

    for (y, row) in img.rows_mut().rev().enumerate() {
        for (x, pixel) in row.enumerate() {
            let u = x as rtiow::Float / rtiow::Float::from(WIDTH);
            let v = y as rtiow::Float / rtiow::Float::from(HEIGHT);
            let r = rtiow::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let color = ray_color(&r, &world);
            *pixel = color.into();
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
    img.save("output.png")?;

    Ok(())
}

fn ray_color(ray: &rtiow::Ray, world: &rtiow::HitList) -> rtiow::Color {
    use {num::Float, rtiow::Hit};

    let color =
        if let Some(hit_record) = world.hit(ray, 0.0, rtiow::Float::infinity()) {
            0.5 * (hit_record.normal() + rtiow::Vector::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = ray.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);

            (1.0 - t) * rtiow::Vector::new(1.0, 1.0, 1.0) + t * rtiow::Vector::new(0.5, 0.7, 1.0)
        };

    rtiow::Color::new_unchecked(color.x, color.y, color.z)
}
