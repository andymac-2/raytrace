use crate::shape::{Direction, Position};
use crate::vec3::Vec3;

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
