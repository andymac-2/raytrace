use crate::shape::{Collision, Direction, Position, Shape};

#[derive(Debug)]
pub struct Cuboid {
    half_x: f64,
    half_y: f64,
    half_z: f64,
}

struct Intersect {
    t: f64,
    normal: Direction,
}

impl Intersect {
    fn min_max(self, other: Intersect) -> (Intersect, Intersect) {
        if self.t < other.t {
            (self, other)
        } else {
            (other, self)
        }
    }

    fn max(self, other: Intersect) -> Intersect {
        if self.t > other.t {
            self
        } else {
            other
        }
    }

    fn min(self, other: Intersect) -> Intersect {
        if self.t < other.t {
            self
        } else {
            other
        }
    }
}

impl Cuboid {
    pub fn new(x_length: f64, y_length: f64, z_length: f64) -> Cuboid {
        Cuboid {
            half_x: x_length / 2.0,
            half_y: y_length / 2.0,
            half_z: z_length / 2.0,
        }
    }

    fn possible_collisions(
        &self,
        origin: &Position,
        direction: &Direction,
    ) -> (Intersect, Intersect) {
        let i1 = Intersect {
            t: (self.half_x - origin.x()) / direction.x(),
            normal: Direction::RIGHT,
        };
        let i2 = Intersect {
            t: (-self.half_x - origin.x()) / direction.x(),
            normal: Direction::LEFT,
        };
        let (min_tx, max_tx) = i1.min_max(i2);

        let i1 = Intersect {
            t: (self.half_y - origin.y()) / direction.y(),
            normal: Direction::FORWARDS,
        };
        let i2 = Intersect {
            t: (-self.half_y - origin.y()) / direction.y(),
            normal: Direction::BACKWARDS,
        };
        let (min_ty, max_ty) = i1.min_max(i2);

        let i1 = Intersect {
            t: (self.half_z - origin.z()) / direction.z(),
            normal: Direction::UP,
        };
        let i2 = Intersect {
            t: (-self.half_z - origin.z()) / direction.z(),
            normal: Direction::DOWN,
        };
        let (min_tz, max_tz) = i1.min_max(i2);

        let collision_in = min_tx.max(min_ty).max(min_tz);
        let collision_out = max_tx.min(max_ty).min(max_tz);

        (collision_in, collision_out)
    }
}

impl Shape for Cuboid {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let (collision_in, collision_out) = self.possible_collisions(origin, direction);

        if collision_in.t > 0.0 && collision_in.t < collision_out.t {
            let position = origin.move_along(direction, collision_in.t);
            Some(Collision::new(
                collision_in.t,
                collision_in.normal,
                position,
            ))
        } else {
            None
        }
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let (collision_in, collision_out) = self.possible_collisions(origin, direction);

        if collision_out.t > 0.0 && collision_in.t < collision_out.t {
            let position = origin.move_along(direction, collision_out.t);
            Some(Collision::new(
                collision_out.t,
                collision_out.normal,
                position,
            ))
        } else {
            None
        }
    }
}
