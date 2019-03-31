pub mod basicbody;
pub use basicbody::BasicBody;

use crate::collision::Collision;
use crate::colour::Colour;
use crate::ray::Ray;
use crate::shape::{Direction, Position};

pub trait Body {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision>;
    fn rays(&self, ray_in: &Ray, collision: &Collision, efficacy: f64) -> Vec<Ray>;
    fn emission(&self) -> Colour;
    fn is_light(&self) -> bool;
}

impl<T: Body> Body for Box<T> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        (**self).collision(origin, direction)
    }
    fn rays(&self, ray_in: &Ray, collision: &Collision, efficacy: f64) -> Vec<Ray> {
        (**self).rays(ray_in, collision, efficacy)
    }
    fn emission(&self) -> Colour {
        (**self).emission()
    }
    fn is_light(&self) -> bool {
        (**self).is_light()
    }
}

impl<T: Body> Body for &T {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        (*self).collision(origin, direction)
    }
    fn rays(&self, ray_in: &Ray, collision: &Collision, efficacy: f64) -> Vec<Ray> {
        (*self).rays(ray_in, collision, efficacy)
    }
    fn emission(&self) -> Colour {
        (*self).emission()
    }
    fn is_light(&self) -> bool {
        (*self).is_light()
    }
}
