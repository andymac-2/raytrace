use crate::shape::{Collision, Direction, Position, Shape};

#[derive(Debug)]
pub struct Translate<T> {
    translation: Position,
    shape: T,
}

impl<T> Translate<T> {
    pub fn new(translate: Position, shape: T) -> Translate<T> {
        Translate {
            translation: translate,
            shape: shape,
        }
    }
}

impl<T: Shape> Shape for Translate<T> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin - &self.translation;
        self.shape
            .collision(&new_origin, direction)
            .map(|collision| collision.translate(&self.translation))
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin - &self.translation;
        self.shape
            .collision_in(&new_origin, direction)
            .map(|collision| collision.translate(&self.translation))
    }
}
