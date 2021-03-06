use crate::shape::{Collision, Direction, Position, Shape};

// infinite plane
pub struct Plane(());

impl Plane {
    pub fn new() -> Plane {
        Plane(())
    }
}

impl Shape for Plane {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        if origin.z() <= 0.0 || direction.z() >= 0.0 {
            return None;
        }
        let t = -origin.z() / direction.z();
        assert!(t >= 0.0);
        let collision = origin.move_along(direction, t);

        Some(Collision::new(t, Direction::UP, collision))
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        if origin.z() >= 0.0 || direction.z() <= 0.0 {
            return None;
        }
        let t = -origin.z() / direction.z();
        assert!(t >= 0.0);
        let collision = origin.move_along(direction, t);

        Some(Collision::new(t, Direction::UP, collision))
    }
}
