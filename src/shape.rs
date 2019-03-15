use super::vec3::Vec3;

pub mod sphere;
pub use sphere::Sphere;

pub mod translate;
pub use translate::Translate;

pub mod scale;
pub use scale::Scale;

pub mod union;
pub use union::Union;

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
}

pub trait Shape {
    // collision to the outside of a shape
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<Collision>;
    // collision to the inside of a shape
    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<Collision>;
}