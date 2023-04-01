use crate::utils::material::Scatter;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3};
use crate::utils::hittable::{HitRecord, Hit};

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: &'a dyn Scatter,
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // (x − Cx)^2 + (y − Cy)^2 + (z − Cz)^2 = (P(t)−C)^2
        // (P(t) − C)^2 = r^2
        // (A + tb − C)^2 = r^2
        // (tb + A − C)^2 = r^2
        // (tb)^2 + 2tb⋅(A−C) + (A−C)^2 - r^2 = 0
        // Solve for t
        // A- C is origin minus center, tb is direction

        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a; // Minus first to get the closest point which ray intersects with sphere
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            point: r.at(root),
            normal: Default::default(),
            t: root,
            front_face: Default::default(),
            mat_ptr: self.mat_ptr,
        };
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}