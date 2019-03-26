use crate::body::Body;
use crate::collision::Collision;
use crate::colour::Colour;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::{Direction, Position, Shape};

const SLIGHTLY_OFF_SURFACE: f64 = 0.0001;

pub struct BasicBody<S, M> {
    pub shape: S,
    pub material: M,
}

struct InternalRay<'a, S, M> {
    pub body: &'a BasicBody<S, M>,
    pub ray: Option<Ray>,
}

impl<'a, S: Shape, M: Material> Iterator for InternalRay<'a, S, M> {
    type Item = Option<Ray>;
    /// bounce a single ray. Stop if no more internal reflections. May return
    /// None if it is a total internal reflection
    fn next(&mut self) -> Option<Option<Ray>> {
        self.ray.clone().and_then(|ray| {
            let direction = ray.direction();
            let origin = ray.origin().move_along(direction, SLIGHTLY_OFF_SURFACE);
            self.body
                .shape
                .collision_in(&origin, direction)
                .map(|collision: Collision| {
                    let distance = origin.dist(collision.collision());
                    let attenuation = self.body.material.refract_attenuation(distance);
                    let ray_proper = ray.attenuate(&attenuation);

                    let (opt_reflection, opt_refraction) =
                        self.body.material.rays(&collision, &ray_proper);
                    self.ray = opt_reflection;
                    opt_refraction
                })
        })
    }
}

impl<S: Shape, M: Material> BasicBody<S, M> {
    fn internal_ray(&self, refracted_ray: Ray) -> Vec<Ray> {
        let ray_iter = InternalRay {
            body: self,
            ray: Some(refracted_ray),
        };
        ray_iter.take(5).filter_map(|x| x).collect()
    }
}

impl<S: Shape, M: Material> Body for BasicBody<S, M> {
    fn collision(&self, origin: &Position, direction: &Direction) -> Option<Collision> {
        self.shape.collision(origin, direction)
    }

    fn emission(&self) -> Colour {
        self.material.emission()
    }

    fn rays(&self, ray_in: &Ray, collision: &Collision, efficacy: f64) -> Vec<Ray> {
        let normal = collision.normal();
        assert!(normal.normalised());
        let ray_count = self.material.ray_count(efficacy);

        let mut rays = Vec::new();
        (0..ray_count as u32).for_each(|_| {
            let (opt_reflection, opt_refraction) = self.material.rays(collision, ray_in);
            opt_reflection.map(|reflection| rays.push(reflection));
            opt_refraction.map(|refraction| {
                rays.append(&mut self.internal_ray(refraction));
            });
        });

        rays.iter()
            .map(|ray| ray.attenuate_num(ray_count))
            .collect()
    }
}
