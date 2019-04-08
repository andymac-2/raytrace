use crate::shape::{Direction, Position};
use crate::vec3::Vec3;
use nalgebra::base::Matrix4;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Collision {
    t: f64,
    normal: Direction,
    collision: Position,
}

impl Collision {
    pub fn new(t: f64, normal: Direction, collision: Position) -> Collision {
        assert!(normal.normalised());
        Collision {
            t: t,
            normal: normal,
            collision: collision,
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

    pub fn affine_trans(&self, transform: &Matrix4<f64>) -> Collision {
        Collision::new(
            self.t,
            self.normal.affine_normal(transform).normalise(),
            self.collision.affine_trans(transform),
        )
    }
    pub fn affine_inv(&self, transform: &Matrix4<f64>) -> Collision {
        Collision::new(
            self.t,
            self.normal.affine_normal_inv(transform).normalise(),
            self.collision.affine_inverse(transform),
        )
    }
    pub fn translate(&self, translation: &Position) -> Collision {
        Collision::new(self.t, self.normal.clone(), &self.collision + translation)
    }
    pub fn scale(&self, scale: &Vec3) -> Collision {
        Collision::new(
            self.t,
            self.normal.reduce_vec(scale).normalise(),
            self.collision.scale_vec(scale),
        )
    }

    pub fn flip_normal(&self) -> Collision {
        Collision::new(self.t, self.normal.negate(), self.collision.clone())
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Collision) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}
impl PartialEq for Collision {
    fn eq(&self, other: &Collision) -> bool {
        other.t() == self.t()
    }
}
impl Eq for Collision {}
impl Ord for Collision {
    fn cmp(&self, other: &Collision) -> Ordering {
        other.t().partial_cmp(&self.t()).unwrap_or(Ordering::Equal)
    }
}
