mod utils;

use image::{ImageBuffer, Rgb, RgbImage};
use utils::ray::Ray;
use utils::vec3::{Color, Point3, Vec3};

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(
        &Point3 {
            e: [0.0, 0.0, -1.0],
        },
        0.5,
        &r,
    ) {
        return Color { e: [1.0, 0.0, 0.0] };
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] }
}

fn write_color(color: &Color) -> Rgb<u8> {
    Rgb([
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8,
    ])
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    // (x − Cx)^2 + (y − Cy)^2 + (z − Cz)^2 = (P(t)−C)^2
    // (P(t) − C)^2 = r^2
    // (A + tb − C)^2 = r^2
    // (tb + A − C)^2 = r^2
    // (tb)^2 + 2tb⋅(A−C) + (A−C)^2 - r^2 = 0
    // Solve for tb
    // A- C is origin minus center, tb is direction

    let oc = r.origin() - *center;
    let a = r.direction().dot(&r.direction());
    let b = 2.0 * r.direction().dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // CAMERA
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    // VIEWPORT
    let origin = Point3 { e: [0.0, 0.0, 0.0] };
    let horizontal = Point3 {
        e: [VIEWPORT_WIDTH, 0.0, 0.0],
    };
    let vertical = Point3 {
        e: [0.0, VIEWPORT_HEIGHT, 0.0],
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Point3 {
            e: [0.0, 0.0, FOCAL_LENGTH],
        };

    // RENDER
    let mut imgbuf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let u: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
        let v: f64 = (IMAGE_HEIGHT - 1 - j) as f64 / (IMAGE_HEIGHT - 1) as f64;

        let r = Ray {
            origin,
            direction: lower_left_corner + u * horizontal + v * vertical - origin,
        };
        let color = ray_color(&r);

        *pixel = write_color(&color);
    }

    imgbuf.save("image/1.png").expect("Image cannot be saved.");
}
