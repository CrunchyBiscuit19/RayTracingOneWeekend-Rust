mod utils;

use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;
    let mut imgbuf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
        let g: f64 = (IMAGE_HEIGHT - 1 - j) as f64 / (IMAGE_HEIGHT - 1) as f64;
        let b = 0.25;

        let ir = (r* 255.0) as u8;
        let ig = (g * 255.0) as u8;
        let ib = (b * 255.0) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    imgbuf.save("1.png").unwrap();
}
