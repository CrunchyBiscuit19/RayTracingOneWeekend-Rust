use crate::utils::hittable::HitRecord;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Color, Vec3};

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hitpoint: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color, // The amount of energy reflected by surface. Dark colors: 0, Light colors: 1
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, hitpoint: &HitRecord) -> Option<(Ray, Color)> {
        let mut scattered_direction = hitpoint.normal + Vec3::random_unit_vector();
        // random_unit_vector might generate vectors like or close to negative hitpoint.normal
        // scattered_direction will be near zero if that happens, which means nowhere for scattered ray to go
        if scattered_direction.near_zero() {
            scattered_direction = hitpoint.normal
        };
        Some((
            Ray {
                origin: hitpoint.point,
                direction: scattered_direction,
            },
            self.albedo,
        ))
    }
}
pub struct Metal {
    pub albedo: Color, // The amount of energy reflected by surface. Dark colors: 0, Light colors: 1
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hitpoint: &HitRecord) -> Option<(Ray, Color)> {
        let reflected_ray = Ray {
            origin: hitpoint.point,
            direction: Vec3::reflect(ray_in.direction().unit_vector(), hitpoint.normal),
        };
        if reflected_ray.direction.dot(&hitpoint.normal) > 0.0 {
            Some((
                reflected_ray,
                self.albedo,
            ))
        } else {
            None
        }
    }
}
