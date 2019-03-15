use crate::shape::{Shape, Collision};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Translate<T> {
    translation: Vec3,
    shape: T,
}

impl<T> Translate<T> {
    pub fn new (translate: Vec3, shape: T) -> Translate<T> {
        Translate {translation: translate, shape: shape}
    }
}

impl<T: Shape> Shape for Translate<T> {
    fn collision (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let new_origin = origin - self.translation;
        self.shape.collision(new_origin, direction)
            .map(|Collision {t, normal, collision}| {
                Collision::new (t, normal, collision + self.translation)
            })
    }

    fn collision_in (&self, origin: Vec3, direction: Vec3) -> Option<(Collision)> {
        let new_origin = origin - self.translation;
        self.shape.collision_in(new_origin, direction)
            .map(|Collision {t, normal, collision}| {
                Collision::new (t, normal, collision + self.translation)
            })
    }
}