use crate::utils::vec3::{Vec3, Point3};
use crate::utils::ray::Ray;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r:&Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = r.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return false }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a; // Minus first to get the closest point which ray intersects with sphere
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max  {
                return false;
            }
        }

        rec.point = r.at(root);
        rec.t = root;
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}


