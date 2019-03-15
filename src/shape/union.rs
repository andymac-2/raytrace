use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

use std::cmp::Ordering::Equal;

#[derive(Debug)]
pub struct Union<T> {
    shapes: Vec<T>
}

impl<T> Union<T> {
    pub fn new (shapes: Vec<T>) -> Union<T> {
        Union {shapes: shapes}
    }
}

impl<T: Shape> Shape for Union<T> {   
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let collisions = self.shapes.iter().filter_map(|shape| {
            shape.collision(origin, direction)
        });
        
        collisions.min_by(|Collision{t: t1, ..}, Collision {t: t2, ..}| {
            t1.partial_cmp(t2).unwrap_or(Equal)
        })
    }

    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let collisions = self.shapes.iter().filter_map(|shape| {
            shape.collision_in(origin, direction)
        });
        
        collisions.min_by(|Collision{t: t1, ..}, Collision {t: t2, ..}| {
            t1.partial_cmp(t2).unwrap_or(Equal)
        })
    }
}