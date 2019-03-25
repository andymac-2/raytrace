use std::ops::{Add, Sub};

use super::vec3::Vec3;

/// A colour type. It has f64 values for each channel. how this works for large values of r, g,
/// b, is determined by the user
#[derive(Debug, Clone)]
pub struct Colour(Vec3);

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour(Vec3::new(r, g, b))
    }

    pub fn red(&self) -> f64 {
        self.0.x
    }
    pub fn green(&self) -> f64 {
        self.0.y
    }
    pub fn blue(&self) -> f64 {
        self.0.z
    }

    // absorb a colour. the absorb argument is the colour of an object after
    // white light (255, 255, 255) has been shone on it
    pub fn absorb(&self, absorb: &Colour) -> Colour {
        let red = self.red() / absorb.red();
        let green = self.green() / absorb.green();
        let blue = self.blue() / absorb.blue();
        Colour::new(red, green, blue)
    }

    pub fn brighten_colour(&self, factor: &Colour) -> Colour {
        Colour::new(
            self.red() * factor.red(),
            self.green() * factor.green(),
            self.blue() * factor.blue(),
        )
    }
    pub fn powf(&self, factor: f64) -> Colour {
        Colour::new(
            self.red().powf(factor),
            self.green().powf(factor),
            self.blue().powf(factor),
        )
    }
    pub fn brighten(&self, factor: f64) -> Colour {
        Colour(self.0.scale(factor))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let attenuated = (self + &Colour::new(0.0, 0.0, 0.0)).brighten(1.0 / 200.0);
        let red = (attenuated.red() * 255.0) / (attenuated.red() + 1.0);
        let green = (attenuated.green() * 255.0) / (attenuated.green() + 1.0);
        let blue = (attenuated.blue() * 255.0) / (attenuated.blue() + 1.0);
        vec![red as u8, green as u8, blue as u8]
    }

    pub const PINK: Colour = Colour(Vec3 {
        x: 255.0,
        y: 105.0,
        z: 180.0,
    });
    pub const GREEN: Colour = Colour(Vec3 {
        x: 100.0,
        y: 255.0,
        z: 100.0,
    });
    pub const GOLD: Colour = Colour(Vec3 {
        x: 255.0,
        y: 215.0,
        z: 0.0,
    });
    pub const WHITE: Colour = Colour(Vec3 {
        x: 255.0,
        y: 255.0,
        z: 255.0,
    });
    pub const GREY: Colour = Colour(Vec3 {
        x: 180.0,
        y: 180.0,
        z: 180.0,
    });
    pub const BLACK: Colour = Colour(Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

impl Add for &Colour {
    type Output = Colour;
    fn add(self, other: &Colour) -> Colour {
        Colour(&self.0 + &other.0)
    }
}

impl Sub for &Colour {
    type Output = Colour;
    fn sub(self, other: &Colour) -> Colour {
        Colour(&self.0 - &other.0)
    }
}
