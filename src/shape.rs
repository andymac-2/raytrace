pub mod sphere;
pub mod plane;
pub mod translate;
pub mod scale;
pub mod union;
pub mod intersection;
pub mod difference;

use crate::vec3::Vec3;

pub use sphere::Sphere;
pub use plane::Plane;
pub use translate::Translate;
pub use scale::Scale;
pub use union::Union;
pub use intersection::Intersection;
pub use difference::Difference;

#[derive(Debug, Clone)]
pub struct Collision {
    pub t: f64,
    pub normal: Vec3,
    pub collision: Vec3,
}

impl Collision {
    pub fn new (t: f64, normal: Vec3, collision: Vec3) -> Collision {
        Collision {
            t: t,
            normal: normal,
            collision: collision,
        }
    }

    pub fn flip_normal (&self) -> Collision {
        Collision::new(self.t, self.normal.negate(), self.collision)
    }
}

pub trait Shape {
    // collision to the outside of a shape
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<Collision>;
    // collision to the inside of a shape. normal points to outside shape.
    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<Collision>;
}