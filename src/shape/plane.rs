use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

// infinite plane
pub struct Plane(());

impl Plane {
    pub fn new () -> Plane {
        Plane(())
    }
}

impl Shape for Plane {
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<Collision> {
        if origin.z <= 0.0 || direction.z >= 0.0 {
            return None;
        }
        let t = -origin.z / direction.z;
        assert!(t >= 0.0);
        let collision = origin + direction.scale(t);

        Some(Collision::new(t, Vec3::Z, collision))
    }

    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<Collision> {
        if origin.z >= 0.0 || direction.z <= 0.0 {
            return None;
        }
        let t = -origin.z / direction.z;
        assert!(t >= 0.0);
        let collision = origin + direction.scale(t);

        Some(Collision::new(t, Vec3::Z, collision))
    }
}