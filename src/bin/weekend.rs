const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const ASPECT: rtiow::Float = WIDTH as rtiow::Float / HEIGHT as rtiow::Float;
const VERTICAL_FOV: rtiow::Float = 20.0;
const APERTURE: rtiow::Float = 0.2;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_RAY_BOUNCES: u32 = 50;

fn main() -> anyhow::Result<()> {
    use rand::Rng;

    let progress_bar = indicatif::ProgressBar::new(u64::from(HEIGHT));

    let mut img: image::RgbImage =
        image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgb([0, 0, 0]));

    let look_from = rtiow::Vector::new(13.0, 2.0, 3.0);
    let look_at = rtiow::Vector::new(0.0, 0.0, 0.0);
    let vup = rtiow::Vector::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let camera = rtiow::Camera::new(
        look_from,
        look_at,
        vup,
        VERTICAL_FOV,
        ASPECT,
        APERTURE,
        dist_to_focus,
    );

    let mut rng = rand::thread_rng();

    let world = random_scene();

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

fn random_scene() -> rtiow::World {
    use rand::Rng;

    let mut world = rtiow::World::new();
    let mut rng = rand::thread_rng();

    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(rtiow::Lambertian::new(rtiow::Color::new_unchecked(
            0.5, 0.5, 0.5,
        ))),
    )));

    for a in (-11..11).map(rtiow::Float::from) {
        for b in (-11..11).map(rtiow::Float::from) {
            let choose_material = rng.gen_range(0.0, 1.0);
            let center = rtiow::Point::new(
                a + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b + 0.9 * rng.gen_range(0.0, 1.0),
            );

            if (center.coords - rtiow::Vector::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                match choose_material {
                    _ if choose_material < 0.4 => {
                        // diffuse
                        let rand_r = rng.gen_range(0.0, 1.0);
                        let rand_g = rng.gen_range(0.0, 1.0);
                        let rand_b = rng.gen_range(0.0, 1.0);

                        let color = rtiow::Color::new_unchecked(rand_r, rand_g, rand_b);
                        world.push(Box::new(rtiow::Sphere::new(
                            center,
                            0.2,
                            Box::new(rtiow::Lambertian::new(color)),
                        )));
                    }
                    _ if choose_material < 0.9 => {
                        // metal
                        let rand_r = rng.gen_range(0.5, 1.0);
                        let rand_g = rng.gen_range(0.5, 1.0);
                        let rand_b = rng.gen_range(0.5, 1.0);

                        let color = rtiow::Color::new_unchecked(rand_r, rand_g, rand_b);
                        let fuzz = rng.gen_range(0.0, 0.5);

                        world.push(Box::new(rtiow::Sphere::new(
                            center,
                            0.2,
                            Box::new(rtiow::Metal::new(color, fuzz)),
                        )));
                    }
                    _ => {
                        // glass
                        world.push(Box::new(rtiow::Sphere::new(
                            center,
                            0.2,
                            Box::new(rtiow::Dielectric::new(1.5)),
                        )));
                    }
                }
            }
        }
    }

    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(rtiow::Dielectric::new(1.5)),
    )));

    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(rtiow::Lambertian::new(rtiow::Color::new_unchecked(
            0.4, 0.2, 0.1,
        ))),
    )));

    world.push(Box::new(rtiow::Sphere::new(
        rtiow::Point::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(rtiow::Metal::new(
            rtiow::Color::new_unchecked(0.7, 0.6, 0.5),
            0.0,
        )),
    )));

    world
}

fn ray_color(ray: &rtiow::Ray, world: &rtiow::World, depth: u32) -> rtiow::Color {
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
