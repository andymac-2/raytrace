use crate::vec3::Vec3;
use nalgebra::base::{Matrix4, RowVector4, Vector4};
use rand::Rng;
use std::ops::{Add, Sub};

/// a direction vector. Some methods are disabled, for example, it doesn't make
/// any sense to add two directions.
#[derive(Debug, Clone)]
pub struct Direction(Vec3);

impl Direction {
    pub fn new(x: f64, y: f64, z: f64) -> Direction {
        Direction(Vec3::new(x, y, z))
    }
    pub fn from_two_points(from: &Position, to: &Position) -> Direction {
        Direction(&to.0 - &from.0)
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    pub fn z(&self) -> f64 {
        self.0.z
    }

    pub fn to_position(&self) -> Position {
        Position(self.0.clone())
    }
    pub fn dot(&self, other: &Direction) -> f64 {
        self.0.dot(&other.0)
    }
    pub fn dot_position(&self, other: &Position) -> f64 {
        self.0.dot(&other.0)
    }
    pub fn cross(&self, other: &Direction) -> Direction {
        Direction(self.0.cross(&other.0))
    }

    pub fn len_sq(&self) -> f64 {
        self.0.len_sq()
    }
    pub fn len(&self) -> f64 {
        self.0.len()
    }
    pub fn negate(&self) -> Direction {
        Direction(Vec3 {
            x: -self.0.x,
            y: -self.0.y,
            z: -self.0.z,
        })
    }
    pub fn normalise(&self) -> Direction {
        Direction(self.0.normalise())
    }
    pub fn normalised(&self) -> bool {
        self.0.normalised()
    }
    pub fn reduce_vec(&self, vector: &Vec3) -> Direction {
        Direction(self.0.reduce_vec(vector))
    }

    /// Perform an affine transformation of a direction. Do not normalise the
    /// result, as it may be used in distance calculations later
    pub fn affine_trans(&self, matrix: &Matrix4<f64>) -> Direction {
        let original = Vector4::new(self.0.x, self.0.y, self.0.z, 0.0);
        let transformed = matrix * original;
        Direction(Vec3 {
            x: transformed.x,
            y: transformed.y,
            z: transformed.z,
        })
    }

    pub fn affine_inverse(&self, matrix: &Matrix4<f64>) -> Direction {
        let original = Vector4::new(self.0.x, self.0.y, self.0.z, 0.0);
        let transformed = matrix.try_inverse().unwrap() * original;
        Direction(Vec3 {
            x: transformed.x,
            y: transformed.y,
            z: transformed.z,
        })
    }

    /// Given a plane defined by a normal in an affine space, return the normal
    /// to a plane before the transform
    pub fn affine_normal_inv(&self, matrix: &Matrix4<f64>) -> Direction {
        let original = RowVector4::new(self.0.x, self.0.y, self.0.z, 0.0);
        let transformed = original * matrix;
        Direction(Vec3 {
            x: transformed.x,
            y: transformed.y,
            z: transformed.z,
        })
    }

    /// Given a plane defined by a normal in an affine space, return the normal
    /// to a plane before the transform
    pub fn affine_normal(&self, matrix: &Matrix4<f64>) -> Direction {
        let original = RowVector4::new(self.0.x, self.0.y, self.0.z, 0.0);
        let transformed = original * matrix.try_inverse().unwrap();
        Direction(Vec3 {
            x: transformed.x,
            y: transformed.y,
            z: transformed.z,
        })
    }

    /// creates a random unit vector. The distribution of random points on the
    /// unit sphere whould be unbiased.
    pub fn random() -> Direction {
        let mut rng = rand::thread_rng();
        loop {
            // Wolfram math world sphere point picking (Marsaglia 1972)
            let a = rng.gen::<f64>() * 2.0 - 1.0;
            let b = rng.gen::<f64>() * 2.0 - 1.0;
            let squares = a * a + b * b;

            if squares >= 1.0 {
                continue;
            }

            // a^2 + b^2 + c^2 = 1
            let c = f64::sqrt(1.0 - squares);
            let x = 2.0 * a * c;
            let y = 2.0 * b * c;
            let z = 1.0 - 2.0 * squares;

            break Direction(Vec3::new(x, y, z));
        }
    }

    /// caluculate a perfect reflection between an incident ray (self) and a
    /// normal. The normal must be normalised.
    pub fn reflection(&self, normal: &Direction) -> Direction {
        assert!(normal.normalised());
        let vec3 = (&self.0 - &normal.0.scale(2.0 * normal.0.dot(&self.0))).normalise();

        Direction(vec3)
    }

    /// calculate a perfect refraction given an incident ray (self). Both the
    /// incident ray and the normal should be normalised. The incident ray
    /// should be pointing towards the surface, and the normal away from the
    /// surface
    pub fn refraction(&self, normal: &Direction, refractive_index: f64) -> Option<Direction> {
        assert!(self.normalised() && normal.normalised());
        // taken from wikipedia page on snell's law.
        let r = 1.0 / refractive_index;
        let c = -(self.dot(normal));
        assert!(c >= 0.0);

        // if determinant negative, then total internal reflection
        let determinant = 1.0 - r * r * (1.0 - c * c);
        if determinant < 0.0 {
            // total internal refraction. object has lower refractivity index
            // than air
            return None;
        }
        let perfect_refraction = &self.0.scale(r) + &normal.0.scale(r * c - f64::sqrt(determinant));

        Some(Direction(perfect_refraction))
    }

    pub fn wobble(&self, normal: &Direction, factor: f64) -> Direction {
        assert!(self.dot(normal) > 0.0);
        assert!(normal.normalised() && self.normalised());

        let random_vec = Direction::random();

        let random_hemisphere = match random_vec.dot(normal) < 0.0 {
            true => random_vec.0.scale(-1.0),
            false => random_vec.0,
        };
        assert!(random_hemisphere.normalised());

        let ratio = random_hemisphere.dist(&normal.0) / 2.0;
        let wobbled_vec = &self.0 + &(&random_hemisphere - &self.0).scale(ratio.powf(factor));

        Direction(wobbled_vec.normalise())
    }

    pub const UP: Direction = Direction(Vec3::Z);
    pub const DOWN: Direction = Direction(crate::vec3::Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    });
    pub const RIGHT: Direction = Direction(Vec3::X);
    pub const LEFT: Direction = Direction(crate::vec3::Vec3 {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    });
    pub const FORWARDS: Direction = Direction(Vec3::Y);
    pub const BACKWARDS: Direction = Direction(crate::vec3::Vec3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    });
}

/// a position vector. Some methods are disabled, for example, it doesn't make
/// any sense to normalise a position.
#[derive(Debug, Clone)]
pub struct Position(Vec3);

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Position {
        Position(Vec3::new(x, y, z))
    }
    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    pub fn z(&self) -> f64 {
        self.0.z
    }

    pub fn to_direction(&self) -> Direction {
        Direction(self.0.clone())
    }

    pub fn dot(&self, other: &Position) -> f64 {
        self.0.dot(&other.0)
    }
    pub fn dot_direction(&self, other: &Direction) -> f64 {
        self.0.dot(&other.0)
    }
    pub fn cross(&self, other: &Direction) -> Position {
        Position(self.0.cross(&other.0))
    }

    pub fn len_sq(&self) -> f64 {
        self.0.len_sq()
    }
    pub fn len(&self) -> f64 {
        self.0.len()
    }
    pub fn dist(&self, other: &Position) -> f64 {
        self.0.dist(&other.0)
    }
    pub fn dist_sq(&self, other: &Position) -> f64 {
        self.0.dist_sq(&other.0)
    }

    pub fn move_along(&self, direction: &Direction, t: f64) -> Position {
        Position(&self.0 + &direction.0.scale(t))
    }
    pub fn get_t(&self, end: &Position, direction: &Direction) -> f64 {
        (self - end).len() / direction.len()
    }
    pub fn scale(&self, t: f64) -> Position {
        Position(self.0.scale(t))
    }
    pub fn scale_vec(&self, vector: &Vec3) -> Position {
        Position(self.0.scale_vec(vector))
    }
    pub fn reduce_vec(&self, vector: &Vec3) -> Position {
        Position(self.0.reduce_vec(vector))
    }

    pub fn affine_trans(&self, matrix: &Matrix4<f64>) -> Position {
        let original = Vector4::new(self.0.x, self.0.y, self.0.z, 1.0);
        let transformed = matrix * original;
        Position(Vec3 {
            x: transformed.x,
            y: transformed.y,
            z: transformed.z,
        })
    }

    pub fn affine_inverse(&self, matrix: &Matrix4<f64>) -> Position {
        let original = Vector4::new(self.0.x, self.0.y, self.0.z, 1.0);
        let transformed = matrix.try_inverse().unwrap() * original;
        Position(Vec3 {
            x: transformed.x,
            y: transformed.y,
            z: transformed.z,
        })
    }

    pub const ORIGIN: Position = Position(crate::vec3::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

impl Add for &Position {
    type Output = Position;
    fn add(self, other: &Position) -> Position {
        Position(&self.0 + &other.0)
    }
}

impl Sub for &Position {
    type Output = Position;
    fn sub(self, other: &Position) -> Position {
        Position(&self.0 - &other.0)
    }
}
