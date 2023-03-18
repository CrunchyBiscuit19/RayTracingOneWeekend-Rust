mod utils;

use crate::utils::camera::Camera;
use crate::utils::rt_weekend::random_double;
use image::{ImageBuffer, Rgb, RgbImage};
use utils::hittable::{Hit, HitRecord, HittableList, Sphere};
use utils::ray::Ray;
use utils::rt_weekend::{clamp, INFINITY};
use utils::vec3::{Color, Point3, Vec3};

fn ray_color<T: Hit>(r: &Ray, world: &T, depth: u32) -> Color {
    if depth <= 0 {
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let mut rec: HitRecord = Default::default();
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.point + rec.normal + Vec3::random_unit_vector();
        return 0.5 * ray_color(&Ray {origin: rec.point, direction: target - rec.point}, world, depth - 1);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] }
}

fn write_color(color: &Color, samples_per_pixel: u32) -> Rgb<u8> {
    let mut r = color[0];
    let mut g = color[1];
    let mut b = color[2];

    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    Rgb([
        (clamp(r, 0.0, 0.999) * 256.0) as u8,
        (clamp(g, 0.0, 0.999) * 256.0) as u8,
        (clamp(b, 0.0, 0.999) * 256.0) as u8,
    ])
}

fn main() {
    // IMAGE
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_REFLECTIONS_DEPTH: u32 = 50;

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
    let camera: Camera = Default::default();

    // RENDER
    let mut imgbuf: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (i, j, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut color: Color = Default::default();

        for _ in 0..SAMPLES_PER_PIXEL {
            let u: f64 = (i as f64 + random_double(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64;
            let v: f64 = ((IMAGE_HEIGHT - 1 - j) as f64 + random_double(0.0, 1.0))
                / (IMAGE_HEIGHT - 1) as f64;

            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, &world, MAX_REFLECTIONS_DEPTH);
        }

        *pixel = write_color(&color, SAMPLES_PER_PIXEL);
    }

    imgbuf.save("image/hd_lambert.png").expect("Image cannot be saved.");
}
