use crate::body::Body;
use crate::colour::Colour;
use crate::ray::Ray;

use std::cmp::Ordering::Equal;

pub struct Scene {
    pub bodies: Vec<Box<dyn Body + Sync>>,
    pub bounces: u32,
}

// if the attenuation is low, then the resulting pixel will be largely affected
// by the colour, for high attenuation, barely any effect. We should therefore
// cast more rays if the light is more effective to spend more computational
// power where it is required. this value is chosen arbitrarily to approximate
// even levels of detail.
const EFFICACY_CONSTANT: f64 = 30.0;

impl Scene {
    pub fn sampler(&self, ray: &Ray, bounce: u32) -> Colour {
        if bounce >= self.bounces {
            return Colour::BLACK;
        }

        self.bodies
            .iter()
            .filter_map(|body| {
                body.collision(ray.origin(), ray.direction())
                    .map(|collision| (body, collision))
            })
            .min_by(|(_b1, collision1), (_b2, collision2)| {
                collision1.t().partial_cmp(&collision2.t()).unwrap_or(Equal)
            })
            .map(|(body, collision)| {
                let att = ray.attenuation();
                // efficacy is how much a given ray is expected to affect the pixel.
                let efficacy = 1.0 + EFFICACY_CONSTANT * (att.red() + att.blue() + att.green());
                let rays = body.rays(ray, &collision, efficacy);

                let mut colour = body.emission().brighten_colour(&att);
                for ray in rays {
                    colour = &colour + &self.sampler(&ray, bounce + 1);
                }
                colour
            })
            .unwrap_or(Colour::BLACK)
    }
}
