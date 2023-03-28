use image::{ImageBuffer, Rgb, RgbImage};

mod utils;

use utils::camera::Camera;
use utils::rt_weekend::random_double;
use utils::hittable::{Hit, HittableList, Sphere};
use utils::ray::Ray;
use utils::rt_weekend::{clamp, INFINITY};
use utils::vec3::{Color, Point3};
use utils::material::{Lambertian, Metal};

fn ray_color<T: Hit>(r: &Ray, world: &T, depth: u32) -> Color {
    if depth <= 0 {
        return Color { e: [0.0, 0.0, 0.0] };
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(closest_rec) => {
            match closest_rec.mat_ptr.scatter(r, &closest_rec) {
                Some((scattered_ray, attenuation)) => { // Attenuation is a percentage of the original light that is used to color a pixel.
                    attenuation * ray_color(&scattered_ray, world, depth - 1)
                }
                None => {
                    Color { e: [0.0, 0.0, 0.0] }
                }
            }
        }
        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] }
        }
    }
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
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_REFLECTIONS_DEPTH: u32 = 50;

    // WORLD
    let mut world = HittableList { objects: vec![] };
    
    world.add(Sphere {
        center: Point3 {
            e: [0.0, -100.5, -1.0],
        },
        radius: 100.0,
        mat_ptr: &Lambertian { albedo: Color { e: [0.8, 0.8, 0.0] } }
    });
    world.add(Sphere {
        center: Point3 {
            e: [0.0, 0.0, -1.0],
        },
        radius: 0.5,
        mat_ptr: &Lambertian { albedo: Color { e: [0.7, 0.3, 0.3] } }
    });
    world.add(Sphere {
        center: Point3 {
            e: [-1.0, 0.0, -1.0],
        },
        radius: 0.5,
        mat_ptr: &Metal { albedo: Color { e: [0.8, 0.8, 0.8] } }
    });
    world.add(Sphere {
        center: Point3 {
            e: [1.0, 0.0, -1.0],
        },
        radius: 0.5,
        mat_ptr: &Metal { albedo: Color { e: [0.8, 0.6, 0.2] } }
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

    imgbuf
        .save(format!("image/{}x{}.png", IMAGE_WIDTH, IMAGE_HEIGHT))
        .expect("Image cannot be saved.");
}
