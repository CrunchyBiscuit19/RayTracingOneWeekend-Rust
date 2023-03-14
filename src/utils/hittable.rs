use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a; // Minus first to get the closest point which ray intersects with sphere
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
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

pub struct HittableList<T: Hit> {
    pub objects: Vec<T>,
}

impl<T> HittableList<T>
where
    T: Hit,
{
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }
}

impl<T> Hit for HittableList<T>
where
    T: Hit,
{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t; // Update closest so far to render only the closest object  
                *rec = HitRecord { ..temp_rec };
            }
        }

        hit_anything
    }
}

