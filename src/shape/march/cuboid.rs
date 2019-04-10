use super::Marcher;
use crate::shape::{Direction, Position};

pub struct Cuboid {
    half_x: f64,
    half_y: f64,
    half_z: f64,
}

impl Cuboid {
    pub fn new(x_length: f64, y_length: f64, z_length: f64) -> Cuboid {
        Cuboid {
            half_x: x_length / 2.0,
            half_y: y_length / 2.0,
            half_z: z_length / 2.0,
        }
    }
}

impl Marcher for Cuboid {
    fn distance_estimator(&self, point: &Position) -> f64 {
        (point.x().abs() - self.half_x)
            .max(point.y().abs() - self.half_y)
            .max(point.z().abs() - self.half_z)
    }

    fn get_normal(&self, point: &Position) -> Direction {
        let x = point.x();
        let y = point.y();
        let z = point.z();

        let x_dist = (x.abs() - self.half_x).abs();
        let y_dist = (y.abs() - self.half_y).abs();
        let z_dist = (z.abs() - self.half_z).abs();

        let direction = if x_dist < y_dist {
            if x_dist < z_dist {
                Direction::new(x.signum(), 0.0, 0.0)
            } else {
                Direction::new(0.0, 0.0, z.signum())
            }
        } else {
            // x_dist eliminated
            if y_dist < z_dist {
                Direction::new(0.0, y.signum(), 0.0)
            } else {
                Direction::new(0.0, 0.0, z.signum())
            }
        };

        assert!(direction.normalised());
        direction
    }
}
