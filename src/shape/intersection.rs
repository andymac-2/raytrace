use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

pub struct Intersection {
    shape1: Box<dyn Shape + Sync>,
    shape2: Box<dyn Shape + Sync>,
}

impl Intersection {
    pub fn new (shape1: Box<dyn Shape + Sync>, 
        shape2: Box<dyn Shape + Sync>) -> Intersection
    {
        Intersection {
            shape1: shape1,
            shape2: shape2
        }
    }
}

impl Shape for Intersection {   
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let collision1 = self.shape1.collision(origin, direction);
        // maybe do a check here to see if we can short circuit?
        let collision2 = self.shape2.collision(origin, direction);

        match (collision1, collision2) {
            (Some(collision1), Some(collision2)) => {
                let Collision {t: t1, ..} = collision1;
                let Collision {t: t2, ..} = collision2;
                assert!(t1 >= 0.0 && t2 >= 0.0);
                if t1 > t2 { Some(collision1) } else { Some(collision2) }
            },
            _ => None,
        }
    }

    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let collision1 = self.shape1.collision_in(origin, direction);
        // maybe do a check here to see if we can short circuit?
        let collision2 = self.shape2.collision_in(origin, direction);

        match (collision1, collision2) {
            (Some(collision1), Some(collision2)) => {
                let Collision {t: t1, ..} = collision1;
                let Collision {t: t2, ..} = collision2;
                assert!(t1 >= 0.0 && t2 >= 0.0);
                if t1 < t2 { Some(collision1) } else { Some(collision2) }
            }
            _ => None,
        }
    }
}