use crate::shape::{Collision, Direction, Position, Shape};

pub struct Difference {
    base: Box<dyn Shape + Sync>,
    negative: Box<dyn Shape + Sync>,
}

impl Difference {
    pub fn new(base: Box<dyn Shape + Sync>, negative: Box<dyn Shape + Sync>) -> Difference {
        Difference {
            base: base,
            negative: negative,
        }
    }
}

impl Shape for Difference {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        // base collision does not exist
        let base_col = self.base.collision(origin, direction)?;
        // base collision exists but negative collision doesn't
        match self.negative.collision(origin, direction) {
            None => Some(base_col),
            Some(neg_col) => {
                // base collision is in front of negative collision
                if base_col.t() < neg_col.t() {
                    return Some(base_col);
                }
                // we never exit the negative object, so no collision.
                let neg_col_out = self.negative.collision_in(origin, direction)?;
                // we exit the negative object before we intersect the base object
                if neg_col_out.t() < base_col.t() {
                    // recursive call in case the shape is convex.
                    let new_origin = origin.move_along(direction, neg_col_out.t());
                    return self.collision(&new_origin, direction);
                }
                // we never exit the base object, so the collision is the negative out.
                self.base.collision_in(origin, direction).map_or(
                    Some(neg_col_out.flip_normal()),
                    |base_col_out| {
                        // we exit the negative object before we exit the base
                        if neg_col_out.t() < base_col_out.t() {
                            Some(neg_col_out.flip_normal())
                        // we exit the negative object after we exit the base.
                        } else {
                            // call recursively in case base or negative is convex.
                            let new_origin = origin.move_along(direction, neg_col_out.t());
                            return self.collision(&new_origin, direction);
                        }
                    },
                )
            }
        }
    }

    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        // no base collision means that the ray is outside the shape. TODO: fix
        // edge case where ray begins inside base and negative, therefore
        // outside shape
        self.base.collision_in(origin, direction).map(|base_col| {
            match self.negative.collision(origin, direction) {
                Some(collision) => {
                    if collision.t() < base_col.t() {
                        collision.flip_normal()
                    } else {
                        base_col
                    }
                }
                _ => base_col,
            }
        })
    }
}
