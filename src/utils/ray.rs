use crate::utils::vec3::{Vec3, Point3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn origin (&self) -> Point3 {
        self.origin
    }

    pub fn direction (&self) -> Point3 {
        self.direction
    }

    pub fn at (&self, t:f64) -> Point3 {
        self.origin + t * self.direction
    }
}