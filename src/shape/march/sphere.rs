use super::Marcher;
use crate::shape::{Direction, Position};

pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere { radius: radius }
    }
}

impl Marcher for Sphere {
    fn distance_estimator(&self, point: &Position) -> f64 {
        point.len() - self.radius
    }

    fn get_normal(&self, point: &Position) -> Direction {
        point.to_direction().normalise()
    }
}
