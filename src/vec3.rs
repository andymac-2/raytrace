use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new (x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn cross (&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn dot (&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn scale_vec (&self, factor: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * factor.x,
            y: self.y * factor.y,
            z: self.z * factor.z,
        }
    }
    pub fn scale (&self, factor: f64) -> Vec3 {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn reduce_vec (&self, factor: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / factor.x,
            y: self.y / factor.y,
            z: self.z / factor.z,
        }
    }

    pub fn normalise (&self) -> Vec3 {
        self.scale(1.0 / self.len())
    }
    pub fn normalised (&self) -> bool {
        (self.len_sq() - 1.0).abs() < 0.0001
    }

    pub fn negate (&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn dist_sq (&self, other: Vec3) -> f64 {
        (self.x - other.x) * (self.x - other.x) +
        (self.y - other.y) * (self.y - other.y) +
        (self.z - other.z) * (self.z - other.z)
    }
    pub fn dist (&self, other: Vec3) -> f64 {
        f64::sqrt(self.dist_sq(other))
    }

    pub fn len_sq (&self) -> f64 {
        self.dot(self)
    }

    pub fn len (&self) -> f64{
        f64::sqrt(self.len_sq())
    }

    pub const X: Vec3 = Vec3 {x: 1.0, y: 0.0, z: 0.0};
    pub const Y: Vec3 = Vec3 {x: 0.0, y: 1.0, z: 0.0};
    pub const Z: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 1.0};
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add (self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub (self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

