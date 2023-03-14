mod utils;

use image::{ImageBuffer, Rgb, RgbImage};
use utils::hittable::{Hit, HitRecord, HittableList, Sphere};
use utils::ray::Ray;
use utils::rt_weekend::INFINITY;
use utils::vec3::{Color, Point3};

fn ray_color<T: Hit>(r: &Ray, world: &T) -> Color {
    let mut rec: HitRecord = Default::default();
    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color { e: [1.0, 1.0, 1.0] });
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

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 3840;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // WORLD
    let mut world = HittableList { objects: vec![] };
    world.add(Sphere {
        center: Point3 {
            e: [0.0, 0.0, -1.0],
        },
        radius: 0.5,
    });
    world.add(Sphere {
        center: Point3 {
            e: [0.0, -100.5, -1.0],
        },
        radius: 100.0,
    });

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
        let color = ray_color(&r, &world);

        *pixel = write_color(&color);
    }

    imgbuf.save("image/1.png").expect("Image cannot be saved.");
}
