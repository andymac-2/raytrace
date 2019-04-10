use crate::collision::Collision;
use crate::shape::{Direction, Position, Shape};

pub mod cuboid;
pub mod sphere;

pub trait Marcher {
    const EPSILON: f64 = 0.00001;
    const NORMAL_EPSILON: f64 = 0.001;
    const DRAW_DIST: f64 = 100.0;

    fn inside(&self, point: &Position) -> bool {
        self.distance_estimator(point) < 0.0
    }

    /// returns
    fn distance_estimator(&self, point: &Position) -> f64;

    /// Returns the normal of a point at the surface of a shape defined by a
    /// distance estimator. If you are implementing your own method, ensure that
    /// the normal is normalised.
    fn get_normal(&self, point: &Position) -> Direction {
        let dx = Position::ORIGIN.move_along(&Direction::RIGHT, Self::NORMAL_EPSILON);
        let dy = Position::ORIGIN.move_along(&Direction::FORWARDS, Self::NORMAL_EPSILON);
        let dz = Position::ORIGIN.move_along(&Direction::UP, Self::NORMAL_EPSILON);
        Direction::new(
            self.distance_estimator(&(point + &dx)) - self.distance_estimator(&(point - &dx)),
            self.distance_estimator(&(point + &dy)) - self.distance_estimator(&(point - &dy)),
            self.distance_estimator(&(point + &dz)) - self.distance_estimator(&(point - &dz)),
        )
        .normalise()
    }
}

pub struct MarchShape<T>(pub T);

impl<T: Marcher> Marcher for MarchShape<T> {
    const EPSILON: f64 = T::EPSILON;
    const NORMAL_EPSILON: f64 = T::NORMAL_EPSILON;
    const DRAW_DIST: f64 = T::DRAW_DIST;
    fn distance_estimator(&self, point: &Position) -> f64 {
        self.0.distance_estimator(point)
    }
    fn get_normal(&self, point: &Position) -> Direction {
        self.0.get_normal(point)
    }
}

impl<T: Marcher> Shape for MarchShape<T> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        if self.inside(origin) {
            return None;
        }

        let mut t = 0.0;
        let normalised = direction.normalise();

        loop {
            let position = origin.move_along(&normalised, t);
            let safe_dist = self.distance_estimator(&position);
            t += safe_dist;

            if t > Self::DRAW_DIST {
                return None;
            }
            if safe_dist < Self::EPSILON {
                return Some(Collision::new(t, self.get_normal(&position), position));
            }
        }
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        if !self.inside(origin) {
            return None;
        }

        let mut t = 0.0;
        let normalised = direction.normalise();

        loop {
            let position = origin.move_along(&normalised, t);
            let safe_dist = -self.distance_estimator(&position);
            t += safe_dist;

            if t > Self::DRAW_DIST {
                return None;
            }
            if safe_dist < Self::EPSILON {
                return Some(Collision::new(t, self.get_normal(&position), position));
            }
        }
    }
}
