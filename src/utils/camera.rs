use crate::utils::ray::Ray;
use crate::utils::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin = Point3 { e: [0.0, 0.0, 0.0] };
        let horizontal = Vec3 {
            e: [VIEWPORT_WIDTH, 0.0, 0.0],
        };
        let vertical = Vec3 {
            e: [0.0, VIEWPORT_HEIGHT, 0.0],
        };

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3 {
                    e: [0.0, 0.0, FOCAL_LENGTH],
                },
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
