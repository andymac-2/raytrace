use crate::shape::{Collision, Direction, Position, Shape};

use std::cmp::Ordering::Equal;

pub struct Union<'a> {
    shapes: Vec<&'a (dyn Shape + Sync)>,
}

impl<'a> Union<'a> {
    pub fn new(shapes: Vec<&'a (dyn Shape + Sync)>) -> Union<'a> {
        Union { shapes: shapes }
    }
}

impl<'a> Shape for Union<'a> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let collisions = self
            .shapes
            .iter()
            .filter_map(|shape| shape.collision(origin, direction));

        collisions.min_by(|collision1, collision2| {
            collision1.t().partial_cmp(&collision2.t()).unwrap_or(Equal)
        })
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let collisions = self
            .shapes
            .iter()
            .filter_map(|shape| shape.collision_in(origin, direction));

        collisions.min_by(|collision1, collision2| {
            collision1.t().partial_cmp(&collision2.t()).unwrap_or(Equal)
        })
    }
}
