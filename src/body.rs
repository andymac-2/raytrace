pub mod basicbody;
pub use basicbody::BasicBody;

use crate::vec3::Vec3;
use crate::shape::Collision;
use crate::colour::Colour;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub attenuation: Colour,
}

pub trait Body {
    fn get_collision (&self, origin: Vec3, direction: Vec3) 
        -> Option<Collision>;
    fn get_rays (&self, direction: Vec3, collision: Collision, 
        efficacy: f64) -> Vec<Ray>;
}

impl Ray {
    fn attenuate_num (&self, factor: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            attenuation: self.attenuation.brighten(1.0 / factor),
        }
    }
    pub fn attenuate (&self, colour: Colour) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            attenuation: self.attenuation.brighten_colour(&colour),
        }
    }
}