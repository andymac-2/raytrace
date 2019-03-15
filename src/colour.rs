use std::ops::{Add, Sub};

use super::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Colour {
    colour: Vec3,
}

impl Colour {
    pub fn new (r: f64, g: f64, b:f64) -> Colour{
        Colour {
            colour: Vec3::new(r, g, b),
        }
    }

    fn red (&self) -> f64{
        self.colour.x
    }
    fn green (&self) -> f64 {
        self.colour.y
    }
    fn blue (&self) -> f64 {
        self.colour.z
    }

    // absorb a colour. the absorb argument is the colour of an object after
    // white light (255, 255, 255) has been shone on it
    pub fn absorb (&self, absorb: &Colour) -> Colour {
        let red = self.red() * absorb.red() / 255.0;
        let green = self.green() * absorb.green() / 255.0;
        let blue = self.blue() * absorb.blue() / 255.0;
        Colour::new(red, green, blue)
    }

    pub fn brighten (&self, factor: f64) -> Colour {
        Colour {colour: self.colour.scale(factor)}
    }

    pub fn to_bytes (&self) -> [u8; 3] {
        [self.red() as u8, self.green() as u8, self.blue() as u8]
    }

    pub const PINK: Colour = Colour {
        colour: Vec3 {x: 255.0, y: 105.0, z: 180.0},
    };
    pub const GREEN: Colour = Colour {
        colour: Vec3 {x: 100.0, y: 255.0, z: 100.0},
    };
    pub const GOLD: Colour = Colour {
        colour: Vec3 {x: 255.0, y: 215.0, z: 0.0},
    };
    pub const WHITE: Colour = Colour {
        colour: Vec3 {x: 255.0, y: 255.0, z: 255.0},
    };
    pub const GREY: Colour = Colour {
        colour: Vec3 {x: 180.0, y: 180.0, z: 180.0},
    };
}

impl Add for Colour {
    type Output = Colour;
    fn add (self, other: Colour) -> Colour {
        Colour {
            colour: self.colour + other.colour,
        }
    }
}

impl Sub for Colour {
    type Output = Colour;
    fn sub (self, other: Colour) -> Colour {
        Colour {
            colour: self.colour - other.colour,
        }
    }
}