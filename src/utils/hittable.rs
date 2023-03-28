use crate::utils::material::Scatter;
use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: &'a dyn Scatter,
}

impl<'a> HitRecord<'a> {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0; // Dot product of ray and normal vectors determine if they are facing same direction, and therefore also  
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

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

pub struct HittableList {
    pub objects: Vec<Box<dyn Hit>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: impl Hit + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hit for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            match object.hit(r, t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t; // Update closest so far to render only the closest object
                    closest_rec = Some(temp_rec);
                }
                None => continue,
            }
        }

        closest_rec
    }
}
