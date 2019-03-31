mod direction_position;

mod affine;
mod cuboid;
mod difference;
mod fractal;
mod intersection;
mod plane;
mod scale;
mod sphere;
mod translate;
mod union;

pub use affine::Affine;
pub use cuboid::Cuboid;
pub use difference::Difference;
pub use fractal::Fractal;
pub use intersection::Intersection;
pub use plane::Plane;
pub use scale::Scale;
pub use sphere::Sphere;
pub use translate::Translate;
pub use union::Union;

use crate::collision::Collision;
pub use direction_position::{Direction, Position};

/// A basic Shape trait. A Shape should be able to, given a line representing an
/// origin and a direction, deterine a collision with that line and the shape.
/// In addition to this, the Shape must be able to return a surface normal.
pub trait Shape {
    /// collision to the outside of a shape. normal points to ouside the shape.
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision>;
    /// collision to the inside of a shape. normal points to outside shape.
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<Collision>;
}

impl<'a, T: Shape> Shape for &'a T {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        (*self).collision(origin, direction)
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        (*self).collision_in(origin, direction)
    }
}

impl<T: Shape> Shape for Box<T> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        self.as_ref().collision(origin, direction)
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        self.as_ref().collision_in(origin, direction)
    }
}
