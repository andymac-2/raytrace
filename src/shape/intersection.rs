use crate::shape::{Collision, Direction, Position, Shape};

pub struct Intersection<T, U> {
    shape1: T,
    shape2: U,
}

impl<T: Shape, U: Shape> Intersection<T, U> {
    pub fn new(shape1: T, shape2: U) -> Intersection<T, U> {
        Intersection {
            shape1: shape1,
            shape2: shape2,
        }
    }
}

impl<T: Shape, U: Shape> Shape for Intersection<T, U> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let collision1 = self.shape1.collision(origin, direction);
        // maybe do a check here to see if we can short circuit?
        let collision2 = self.shape2.collision(origin, direction);

        match (collision1, collision2) {
            (Some(collision1), Some(collision2)) => {
                let t1 = collision1.t();
                let t2 = collision2.t();
                assert!(t1 >= 0.0 && t2 >= 0.0);

                if t1 > t2 {
                    Some(collision1)
                } else {
                    Some(collision2)
                }
            }
            _ => None,
        }
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let collision1 = self.shape1.collision_in(origin, direction);
        // maybe do a check here to see if we can short circuit?
        let collision2 = self.shape2.collision_in(origin, direction);

        match (collision1, collision2) {
            (Some(collision1), Some(collision2)) => {
                let t1 = collision1.t();
                let t2 = collision2.t();
                assert!(t1 >= 0.0 && t2 >= 0.0);

                if t1 < t2 {
                    Some(collision1)
                } else {
                    Some(collision2)
                }
            }
            _ => None,
        }
    }
}
