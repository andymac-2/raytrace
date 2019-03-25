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
}
