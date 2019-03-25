use crate::shape::{Collision, Direction, Position, Shape};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Scale<T> {
    scale: Vec3,
    shape: T,
}

impl<T> Scale<T> {
    pub fn new(scale: Vec3, shape: T) -> Scale<T> {
        Scale {
            scale: scale,
            shape: shape,
        }
    }
}

impl<T: Shape> Shape for Scale<T> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin.reduce_vec(&self.scale);
        let new_direction = direction.reduce_vec(&self.scale);
        self.shape
            .collision_in(&new_origin, &new_direction)
            .map(|collision| collision.scale(&self.scale))
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin.reduce_vec(&self.scale);
        let new_direction = direction.reduce_vec(&self.scale);
        self.shape
            .collision_in(&new_origin, &new_direction)
            .map(|collision| collision.scale(&self.scale))
    }
}
