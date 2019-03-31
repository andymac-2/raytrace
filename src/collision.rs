use crate::shape::{Direction, Position};
use crate::vec3::Vec3;
use nalgebra::base::Matrix4;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Collision {
    t: f64,
    normal: Direction,
    collision: Position,
    direction: Direction,
}

impl Collision {
    pub fn new(t: f64, normal: Direction, collision: Position, direction: Direction) -> Collision {
        assert!(normal.normalised());
        assert!(direction.normalised());
        Collision {
            t: t,
            normal: normal,
            collision: collision,
            direction: direction,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn normal(&self) -> &Direction {
        &self.normal
    }
    pub fn collision(&self) -> &Position {
        &self.collision
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn affine_inv(&self, transform: Matrix4<f64>) -> Collision {
        Collision::new(
            self.t,
            self.normal.affine_normal_inv(transform),
            self.collision.affine_inverse(transform),
            self.direction.affine_inverse(transform),
        )
    }
    pub fn translate(&self, translation: &Position) -> Collision {
        Collision::new(
            self.t,
            self.normal.clone(),
            &self.collision + translation,
            self.direction.clone(),
        )
    }
    pub fn scale(&self, scale: &Vec3) -> Collision {
        Collision::new(
            self.t,
            self.normal.reduce_vec(scale).normalise(),
            self.collision.scale_vec(scale),
            self.direction.clone(),
        )
    }

    pub fn flip_normal(&self) -> Collision {
        Collision::new(
            self.t,
            self.normal.negate(),
            self.collision.clone(),
            self.direction.clone(),
        )
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Collision) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Collision {
    fn eq(&self, other: &Collision) -> bool {
        self.t() == other.t()
    }
}
impl Eq for Collision {}
impl Ord for Collision {
    fn cmp(&self, other: &Collision) -> Ordering {
        self.t().partial_cmp(&other.t()).unwrap_or(Ordering::Equal)
    }
}
