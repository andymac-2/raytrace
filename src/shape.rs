mod direction_position;

mod difference;
mod intersection;
mod plane;
mod scale;
mod sphere;
mod translate;
mod union;

pub use difference::Difference;
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
