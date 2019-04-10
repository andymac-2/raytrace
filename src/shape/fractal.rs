use crate::shape::{Collision, Direction, Position, Shape};
use nalgebra::base::Matrix4;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Fractal<P, B> {
    primitive: P,
    bounds: B,
    transforms: Vec<Matrix4<f64>>,
    dwell: u32,
}

/// Collisions are in global space, not local space.
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

fn push_opt<T: Ord>(heap: &mut BinaryHeap<T>, item: Option<T>) {
    match item {
        None => {}
        Some(element) => heap.push(element),
    }
}

impl<P: Shape, B: Shape> Fractal<P, B> {
    pub fn new(
        primitive: P,
        bounds: B,
        transforms: Vec<Matrix4<f64>>,
        dwell: u32,
    ) -> Fractal<P, B> {
        assert!(transforms.iter().all(|matrix| matrix.is_invertible()));
        Fractal {
            primitive: primitive,
            bounds: bounds,
            transforms: transforms,
            dwell: dwell,
        }
    }

    fn get_inner_events(
        &self,
        origin: &Position,
        direction: &Direction,
        transform: &Matrix4<f64>,
    ) -> Vec<Event> {
        assert!(transform.is_invertible());
        let mut result: Vec<Event> = self
            .transforms
            .iter()
            .filter_map(|inner_transform| {
                let global_transform = transform * inner_transform;
                let inv_transform = global_transform.try_inverse().unwrap();

                let new_origin = origin.affine_trans(&inv_transform);
                let new_direction = direction.affine_trans(&inv_transform);

                self.bounds
                    .collision(&new_origin, &new_direction)
                    .map(|collision| {
                        Event::Bound(collision.affine_trans(&global_transform), global_transform)
                    })
            })
            .collect();

        let inv_transform = transform.try_inverse().unwrap();
        let new_origin = origin.affine_trans(&inv_transform);
        let new_direction = direction.affine_trans(&inv_transform);

        if let Some(collision) = self.primitive.collision(&new_origin, &new_direction) {
            let collision = collision.affine_trans(transform);
            result.push(Event::Real(collision));
        }
        result
    }

    fn collision_dwell(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        let collision = self.bounds.collision(origin, direction)?;

        let mut heap = BinaryHeap::new();
        heap.push(Event::Bound(collision, Matrix4::identity()));

        for _i in 0..self.dwell {
            match heap.pop() {
                None => return None,
                Some(Event::Real(collision)) => return Some(collision),
                Some(Event::Bound(_, transform)) => {
                    self.get_inner_events(origin, direction, &transform)
                        .into_iter()
                        .for_each(|event| {
                            heap.push(event);
                        });
                }
            }
        }
        heap.pop().map(|event| match event {
            Event::Real(collision) => collision,
            Event::Bound(collision, _) => collision,
        })
    }
    pub fn collision_in_dwell(
        &self,
        _origin: &Position,
        _direction: &Direction,
        _dwell: u32,
    ) -> Option<(Collision)> {
        unimplemented!()
    }
}

impl<P: Shape, B: Shape> Shape for Fractal<P, B> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        self.collision_dwell(origin, direction)
    }
    fn collision_in(&self, origin: &Position, direction: &Direction) -> Option<(Collision)> {
        unimplemented!()
    }
}
