use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Scale<T> {
    scale: Vec3,
    shape: T,
}

impl<T> Scale<T> {
    pub fn new (scale: Vec3, shape: T) -> Scale<T> {
        Scale {scale: scale, shape: shape}
    }
}

impl<T: Shape> Shape for Scale<T> {
    
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let new_origin = origin.reduce_vec(self.scale);
        let new_direction = direction.reduce_vec(self.scale);
        self.shape.collision(new_origin, new_direction)
            .map(|Collision {t, normal, collision}| {
                Collision::new (
                    t, 
                    normal.reduce_vec(self.scale), 
                    collision.scale_vec(self.scale))
            })
    }

    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let new_origin = origin.reduce_vec(self.scale);
        let new_direction = direction.reduce_vec(self.scale);
        self.shape.collision_in(new_origin, new_direction)
            .map(|Collision {t, normal, collision}| {
                Collision::new (
                    t, 
                    normal.reduce_vec(self.scale), 
                    collision.scale_vec(self.scale))
            })
    }
}