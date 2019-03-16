use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new (radius: f64) -> Sphere {
        Sphere {radius: radius}
    }
}

impl Shape for Sphere {
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let b: f64 = 2.0 * (direction.dot(&origin));
        let a: f64 = direction.dot(&direction);
        let c: f64 = origin.dot(&origin) - self.radius - self.radius;

        let determinant_sq = b * b - 4.0 * a * c;
        if determinant_sq < 0.0 {
            return None;
        }

        let t: f64 = (-b - f64::sqrt(determinant_sq)) / (2.0 * a);
        // intersection behind camera
        if t < 0.0 {
            return None;
        }

        let collision = origin + direction.scale(t);
        Some(Collision::new(t, collision, collision))
    }

    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let b: f64 = 2.0 * (direction.dot(&origin));
        let a: f64 = direction.dot(&direction);
        let c: f64 = origin.dot(&origin) - self.radius - self.radius;

        let determinant_sq = b * b - 4.0 * a * c;
        if determinant_sq < 0.0 {
            return None;
        }

        let t: f64 = (-b + f64::sqrt(determinant_sq)) / (2.0 * a);
        // intersection behind camera
        if t < 0.0 {
            return None;
        }

        let collision = origin + direction.scale(t);
        Some(Collision::new(t, collision, collision))
    }
}