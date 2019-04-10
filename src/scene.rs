use crate::body::Body;
use crate::camera::Camera;
use crate::colour::Colour;
use crate::ray::Ray;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use std::cmp::Ordering::Equal;

pub struct Scene<'a> {
    samples: u32,
    bounces: u32,
    camera: Camera,
    bodies: Vec<Box<dyn Body + Sync + 'a>>,
}

// if the attenuation is low, then the resulting pixel will be largely affected
// by the colour, for high attenuation, barely any effect. We should therefore
// cast more rays if the light is more effective to spend more computational
// power where it is required. this value is chosen arbitrarily to approximate
// even levels of detail.
const EFFICACY_CONSTANT: f64 = 40.0;

impl<'a> Scene<'a> {
    pub fn new(
        samples: u32,
        bounces: u32,
        camera: Camera,
        bodies: Vec<Box<dyn Body + Sync + 'a>>,
    ) -> Scene<'a> {
        Scene {
            samples: samples,
            bounces: bounces,
            camera: camera,
            bodies: bodies,
        }
    }

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

    pub fn render_ppm(&self) -> Vec<u8> {
        let (x_res, y_res) = self.camera.resolution();

        let mut header = format!("P6 {} {} 255 ", x_res, y_res).as_bytes().to_vec();
        let mut pixels = (0..y_res)
            .into_par_iter()
            .map(|y| {
                (0..x_res)
                    .map(|x| {
                        let mut colour = Colour::new(0.0, 0.0, 0.0);

                        (0..self.samples).for_each(|_| {
                            let (start, direction) = self.camera.generate_ray(x as f64, y as f64);
                            colour = &colour
                                + &self.sampler(
                                    &Ray::new(start, direction, Colour::new(1.0, 1.0, 1.0)),
                                    0,
                                );
                        });

                        colour = colour.brighten(1.0 / (self.samples as f64));

                        colour.to_bytes()
                    })
                    .flatten()
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect();

        header.append(&mut pixels);
        header
    }
}
