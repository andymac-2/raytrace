use crate::shape::{Collision, Direction, Position, Shape};
use nalgebra::base::Matrix4;

pub struct Affine<S> {
    shape: S,
    transform: Matrix4<f64>,
    inv_transform: Matrix4<f64>,
}

impl<S: Shape> Affine<S> {
    pub fn new(shape: S, transform: Matrix4<f64>) -> Affine<S> {
        assert!(transform.is_invertible());
        Affine {
            shape: shape,
            transform: transform,
            inv_transform: transform.try_inverse().unwrap(),
        }
    }
}

impl<S: Shape> Shape for Affine<S> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin.affine_trans(&self.inv_transform);
        let new_direction = direction.affine_trans(&self.inv_transform);
        self.shape
            .collision(&new_origin, &new_direction)
            .map(|collision| collision.affine_trans(&self.transform))
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin.affine_trans(&self.inv_transform);
        let new_direction = direction.affine_trans(&self.inv_transform);
        self.shape
            .collision_in(&new_origin, &new_direction)
            .map(|collision| collision.affine_trans(&self.transform))
    }
}
