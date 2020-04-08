const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;
const ASPECT: rtiow::Float = WIDTH as rtiow::Float / HEIGHT as rtiow::Float;
const VERTICAL_FOV: rtiow::Float = 27.0;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_RAY_BOUNCES: u32 = 50;

fn main() -> anyhow::Result<()> {
    use rand::Rng;

    let progress_bar = indicatif::ProgressBar::new(u64::from(HEIGHT));

    let mut img: image::RgbImage =
        image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgb([0, 0, 0]));

    let camera = rtiow::Camera::new(
        rtiow::Vector::new(0.0, 0.5, 2.0),
        rtiow::Vector::new(0.0, 0.0, -1.0),
        rtiow::Vector::new(0.0, 1.0, 0.0),
        VERTICAL_FOV,
        ASPECT,
    );
    let mut rng = rand::thread_rng();

    let mut world = rtiow::HitList::new();

    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(rtiow::Lambertian::new(rtiow::Color::new_unchecked(
            0.7, 0.3, 0.3,
        ))),
    )));
    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(rtiow::Lambertian::new(rtiow::Color::new_unchecked(
            0.8, 0.8, 0.0,
        ))),
    )));
    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(rtiow::Metal::new(rtiow::Color::new_unchecked(
            0.8, 0.6, 0.2,
        ))),
    )));
    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(rtiow::Metal::new(rtiow::Color::new_unchecked(
            0.8, 0.8, 0.8,
        ))),
    )));

    for (y, row) in img.rows_mut().rev().enumerate() {
        for (x, pixel) in row.enumerate() {
            let mut color = rtiow::Color::new_unchecked(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as rtiow::Float + rng.gen_range(0.0, 1.0)) / rtiow::Float::from(WIDTH);
                let v = (y as rtiow::Float + rng.gen_range(0.0, 1.0)) / rtiow::Float::from(HEIGHT);
                let r = camera.ray(u, v);

                color += ray_color(&r, &world, MAX_RAY_BOUNCES);
            }

            *pixel = color.as_rgb(SAMPLES_PER_PIXEL);
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
    img.save("output.png")?;

    Ok(())
}

fn ray_color(ray: &rtiow::Ray, world: &rtiow::HitList, depth: u32) -> rtiow::Color {
    use rtiow::Hit;

    // If we have arrived at the ray bounce limit, then don’t contribute any more light.
    if depth == 0 {
        return rtiow::Color::new_unchecked(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, rtiow::float::INFINITY) {
        if let Some((scattered, attenuation)) = hit_record.material().scatter(ray, hit_record) {
            ray_color(&scattered, world, depth - 1) * attenuation
        } else {
            rtiow::Color::new_unchecked(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        let gradient =
            (1.0 - t) * rtiow::Vector::new(1.0, 1.0, 1.0) + t * rtiow::Vector::new(0.5, 0.7, 1.0);

        rtiow::Color::new_unchecked(gradient.x, gradient.y, gradient.z)
    }
}
