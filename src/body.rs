use crate::vec3::Vec3;
use crate::shape::{Shape, Collision};
use crate::material::Material;
use crate::colour::Colour;

pub struct Body<T> {
    pub shape: T,
    pub material: Material,
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub attenuation: Colour,
}

const SLIGHTLY_OFF_SURFACE: f64 = 0.0001;

impl<T: Shape> Body<T> {
    pub fn get_rays (&self, origin: Vec3, direction: Vec3) -> Vec<Ray> {
        let mut rays = Vec::new();
        self.shape
            .collision(origin, direction)
            .map(|Collision {normal, collision, ..}| {
                let normal = normal.normalise();
                self.get_reflection_ray(direction, normal, collision).map(|ray|{
                    rays.push(ray);
                });
                self.get_refraction_ray(direction, normal, collision).map(|ray|{
                    rays.push(ray);
                }); 
            });
        rays
    }

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
        let bounces: i32 = 4;

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