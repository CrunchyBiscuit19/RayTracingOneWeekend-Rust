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
    pub fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hitpoint: &HitRecord) -> Option<(Ray, Color)> {
        let reflected_ray = Ray {
            origin: hitpoint.point,
            direction: ray_in.direction().unit_vector().reflect(hitpoint.normal) + self.fuzz * Vec3::random_in_unit_sphere(),
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

pub struct Dielectric {
    pub ir: f64, // refractive index
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hitpoint: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color { e: [1.0, 1.0, 1.0] };
        let refraction_ratio = if hitpoint.front_face {1.0 / self.ir} else { self.ir };        

        let unit_direction = ray_in.direction().unit_vector();

        let cos_theta = (-unit_direction).dot(&hitpoint.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let refracted = if refraction_ratio * sin_theta > 1.0 {
            unit_direction.reflect(hitpoint.normal)
        } else {
            unit_direction.refract(hitpoint.normal, refraction_ratio)
        };

        Some((
            Ray {
                origin: hitpoint.point,
                direction: refracted
            },
            attenuation
        ))
    }
}