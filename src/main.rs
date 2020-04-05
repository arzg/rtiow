const WIDTH: u32 = 200;
const HEIGHT: u32 = 100;

fn main() -> anyhow::Result<()> {
    let progress_bar = indicatif::ProgressBar::new(u64::from(WIDTH));

    let mut img: image::RgbImage =
        image::ImageBuffer::from_pixel(WIDTH, HEIGHT, image::Rgb([0, 0, 0]));

    for (y, row) in img.rows_mut().rev().enumerate() {
        for (x, pixel) in row.enumerate() {
            let r = x as f64 / f64::from(WIDTH);
            let g = y as f64 / f64::from(HEIGHT);
            let b: f64 = 0.2;

            *pixel = image::Rgb([
                (255.999 * r).round() as u8,
                (255.999 * g).round() as u8,
                (255.999 * b).round() as u8,
            ]);
        }

        progress_bar.inc(1);
    }

    progress_bar.finish();
    img.save("output.png")?;

    Ok(())
}
