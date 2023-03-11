use crate::utils::vec3::{Vec3, Point3};

#[derive(Debug, Copy, Clone)]
struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    fn origin (&self) -> Point3 {
        self.origin
    }

    fn direction (&self) -> Point3 {
        self.direction
    }

    fn at (&self, t:f64) -> Point3 {
        self.origin + t * self.direction
    }
}