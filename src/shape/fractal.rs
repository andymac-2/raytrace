use crate::shape::{Collision, Direction, Position, Shape};
use nalgebra::base::Matrix4;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Fractal<P, B> {
    primitive: P,
    bounds: B,
    branches: Vec<Matrix4<f64>>,
    dwell: u32,
}

enum Event {
    Real(Collision),
    Bound(Collision, Matrix4<f64>),
}

impl Event {
    fn get_collision(&self) -> &Collision {
        match self {
            Event::Real(collision) => &collision,
            Event::Bound(collision, _) => &collision,
        }
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.get_collision() == other.get_collision()
    }
}
impl Eq for Event {}
impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        other.get_collision().cmp(self.get_collision())
    }
}
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl<P: Shape, B: Shape> Fractal<P, B> {
    pub fn new(primitive: P, bounds: B, branches: Vec<Matrix4<f64>>, dwell: u32) -> Fractal<P, B> {
        assert!(branches.iter().all(|matrix| matrix.is_invertible()));
        Fractal {
            primitive: primitive,
            bounds: bounds,
            branches: branches,
            dwell: dwell,
        }
    }

    fn push_opt<T: Ord>(heap: &mut BinaryHeap<T>, item: Option<T>) {
        match item {
            None => {}
            Some(element) => heap.push(element),
        }
    }

    pub fn collision_dwell(
        &self,
        origin: &Position,
        direction: &Direction,
        dwell: u32,
    ) -> Option<(Collision)> {
        let collision = self.bounds.collision(origin, direction)?;

        let mut heap = BinaryHeap::new();
        heap.push(Event::Bound(collision, Matrix4::identity()));

        for _i in 0..self.dwell {
            match heap.pop() {
                None => return None,
                Some(Event::Real(collision)) => return Some(collision),
                Some(Event::Bound(collision, transform)) => {
                    //push_opt(heap, self.bounds.)
                    unimplemented!()
                }
            }
        }
        None
        // while the heap is not empty
        // pop the first element of the heap
        // if it's a real collision, return the collision.
        // if we don't collide with the bounds of the item, continue
        // otherwise, add the primitive to the heap alongide the bounds
        // of the child fractal

        // if the heap is empty, return None.
    }
    pub fn collision_in_dwell(
        &self,
        origin: &Position,
        direction: &Direction,
        dwell: u32,
    ) -> Option<(Collision)> {
        unimplemented!()
    }
}

impl<P: Shape, B: Shape> Shape for Fractal<P, B> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        unimplemented!()
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        unimplemented!()
    }
}
