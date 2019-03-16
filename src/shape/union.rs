use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

use std::cmp::Ordering::Equal;

pub struct Union {
    shapes: Vec<Box<dyn Shape>>
}

impl Union {
    pub fn new (shapes: Vec<Box<dyn Shape>>) -> Union {
        Union {shapes: shapes}
    }
}

impl Shape for Union {   
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