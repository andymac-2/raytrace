use crate::vec3::Vec3;
use crate::shape::{Shape, Collision};
use crate::colour::Colour;
use crate::material::Material;
use crate::body::{Ray, Body};

pub struct BasicBody<T> {
    pub shape: T,
    pub material: Material,
}

const SLIGHTLY_OFF_SURFACE: f64 = 0.0001;

impl<T: Shape> BasicBody<T> {
    // origin and direction from the world
    fn get_reflection_ray (&self, direction: Vec3, normal: Vec3, 
        collision: Vec3) -> Option<Ray> 
    {
        let reflection_dir = self.material
            .reflect_direction(direction, normal)?;
        let attenuation = self.material.reflective_absorption
            .map_or(Colour::BLACK, |attenuation| {attenuation});

        Some(Ray {
            origin: collision + reflection_dir.scale(SLIGHTLY_OFF_SURFACE),
            direction: reflection_dir,
            attenuation: attenuation,
        })
    }

    // origin and direction are from *inside* the shape.
    fn get_refraction_ray (&self, direction: Vec3, normal: Vec3,
        origin: Vec3) -> Option<Ray> 
    {
        let mut direction = self.material.refract_direction(direction, normal)?;
        let mut origin = origin + direction.scale(SLIGHTLY_OFF_SURFACE);
        let mut length: f64 = 0.0;
        let bounces: i32 = 7;

        for _x in 0..bounces {
            let Collision {normal, collision, t} = 
                self.shape.collision_in(origin, direction)?;
            let normal = normal.normalise();
            let (vec_out, is_inside) = 
                self.material.refract_direction_out(direction, normal)?;

            // direction must be normalised otherwise t will not be length
            assert!(direction.normalised());
            length += t;
            origin = collision + direction.scale(SLIGHTLY_OFF_SURFACE);
            direction = vec_out;

            if !is_inside {
                return Some(Ray {
                    origin: origin,
                    direction: direction,
                    attenuation: self.material.refract_attenuation(length),
                });
            }
        }

        None
    }
}

impl<T: Shape> Body for BasicBody<T> {
    fn get_collision (&self, origin: Vec3, direction: Vec3) 
        -> Option<Collision> 
    {
        self.shape.collision(origin, direction)
    }

    fn get_rays (&self, direction: Vec3, 
        Collision {normal, collision, ..}: Collision, efficacy: f64) 
        -> Vec<Ray> 
    {
        let normal = normal.normalise();
        let reflective_ray_count = self.material
            .reflective_ray_count(efficacy);
        let refractive_ray_count = self.material
            .refractive_ray_count(efficacy);
        let reflection_rays = (0..reflective_ray_count as u32)
            .filter_map(|_| {
                self.get_reflection_ray(direction, normal, collision)
            })
            .map(|ray| ray.attenuate_num(reflective_ray_count));
        let refraction_rays = (0..refractive_ray_count as u32)
            .filter_map(|_| {
                self.get_refraction_ray(direction, normal, collision)
            })
            .map(|ray| ray.attenuate_num(refractive_ray_count));

        reflection_rays.chain(refraction_rays).collect()
    }
}