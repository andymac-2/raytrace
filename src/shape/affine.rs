use crate::shape::{Collision, Direction, Position, Shape};
use nalgebra::base::Matrix4;

pub struct Affine<S> {
    shape: S,
    transform: Matrix4<f64>,
}

impl<S: Shape> Affine<S> {
    pub fn new(shape: S, transform: Matrix4<f64>) -> Affine<S> {
        assert!(transform.is_invertible());
        Affine {
            shape: shape,
            transform: transform,
        }
    }
}

impl<S: Shape> Shape for Affine<S> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin.affine_trans(self.transform);
        let new_direction = direction.affine_trans(self.transform);
        self.shape
            .collision(&new_origin, &new_direction)
            .map(|collision| {
                Collision::new(
                    collision.t(),
                    collision
                        .normal()
                        .affine_normal_inv(self.transform)
                        .normalise(),
                    collision.collision().affine_inverse(self.transform),
                    direction.clone(),
                )
            })
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let new_origin = origin.affine_trans(self.transform);
        let new_direction = direction.affine_trans(self.transform);
        self.shape
            .collision_in(&new_origin, &new_direction)
            .map(|collision| {
                Collision::new(
                    collision.t(),
                    collision
                        .normal()
                        .affine_normal_inv(self.transform)
                        .normalise(),
                    collision.collision().affine_inverse(self.transform),
                    direction.clone(),
                )
            })
    }
}
