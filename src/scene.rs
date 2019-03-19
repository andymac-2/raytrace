use crate::vec3::Vec3;
use crate::shape::Collision;
use crate::body::{Body, Ray};
use crate::colour::Colour;

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
const EFFICACY_CONSTANT: f64 = 20.0;

impl Scene {
    pub fn sampler (&self, Ray{origin, direction, attenuation}: &Ray, 
        bounce: u32) -> Colour 
    {
        let direction = direction.normalise();

        if bounce >= self.bounces {
            return Scene::ambient(&direction, attenuation);
        }

        self.bodies.iter()
            .filter_map(|body| {
                body.get_collision(*origin, direction).map(|collision|{
                    (body, collision)
                })
            })
            .min_by(|(_b1, Collision {t: t1, ..}), (_b2, Collision {t: t2, ..})| {
                t1.partial_cmp(t2).unwrap_or(Equal)
            })
            .map (|(body, collision)| {
                // efficacy is how much a given ray is expected to affect the pixel.
                let efficacy = 1.0 + EFFICACY_CONSTANT * (attenuation.red() + 
                    attenuation.blue() + attenuation.green());
                let rays = body.get_rays(direction, collision, efficacy);
    
                let mut colour = Colour::BLACK;
                for ray in rays {
                colour = colour + self.sampler(
                    &ray.attenuate(*attenuation), bounce + 1);
                }
                colour
            })
            .unwrap_or_else (|| Scene::ambient(&direction, attenuation))

            // if Vec3::Z.dot(&direction) > 0.0 {
            //     return Colour::new(300.0, 600.0, 700.0)
            //         .brighten(direction.z / 2.0 + 0.5)
            //         .brighten_colour(*attenuation);
            // }

            // let t = (-origin.z) / direction.z;
            // let collision = *origin + direction.scale(t);
            // if (f64::ceil(collision.x / 3.0) + 
            //     f64::ceil(collision.y / 3.0)) as i32 % 2 == 0
            // {
            //     Colour::new(20.0, 20.0, 20.0).brighten_colour(*attenuation)
            // }
            // else {
            //     Colour::new(300.0, 300.0, 300.0).brighten_colour(*attenuation)
            // }
    }

    fn ambient (direction: &Vec3, attenuation: &Colour) -> Colour {
        let intensity = direction
            .dot(&Vec3::new(5.0, 0.0, 10.0).normalise())
            .max(0.0)
            .powi(20);
        let sun = Colour::new(500.0, 500.0, 500.0);
        sun.brighten(100.0 * intensity).brighten_colour(attenuation)
    }
}