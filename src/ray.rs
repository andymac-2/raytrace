use crate::colour::Colour;
use crate::shape::{Direction, Position};
use nalgebra::base::Matrix4;

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Position,
    direction: Direction,
    attenuation: Colour,
}

impl Ray {
    pub fn new(origin: Position, direction: Direction, attenuation: Colour) -> Ray {
        assert!(direction.normalised());
        Ray {
            origin: origin,
            direction: direction,
            attenuation: attenuation,
        }
    }
    pub fn origin(&self) -> &Position {
        &self.origin
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }
    pub fn attenuation(&self) -> &Colour {
        &self.attenuation
    }

    pub fn attenuate(&self, colour: &Colour) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.direction.clone(),
            attenuation: self.attenuation.brighten_colour(colour),
        }
    }
    pub fn attenuate_num(&self, factor: f64) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.direction.clone(),
            attenuation: self.attenuation.brighten(1.0 / factor),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeometricRay {
    origin: Position,
    direction: Direction,
}
impl GeometricRay {
    pub fn new(origin: Position, direction: Direction) -> GeometricRay {
        assert!(direction.normalised());
        GeometricRay {
            origin: origin,
            direction: direction,
        }
    }
    pub fn origin(&self) -> &Position {
        &self.origin
    }
    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn affine_trans(&self, transform: Matrix4<f64>) -> GeometricRay {
        GeometricRay {
            origin: self.origin.affine_trans(transform),
            direction: self.direction.affine_trans(transform),
        }
    }
}
