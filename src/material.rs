pub mod physical;

use crate::collision::Collision;
use crate::colour::Colour;
use crate::ray::Ray;

pub use physical::Physical;

pub trait Material {
    /// return the emissive value of a material.
    fn emission(&self) -> Colour;
    /// return optionally a reflective ray if applicable, and a refractive ray,
    /// if applicable.
    fn rays(&self, collision: &Collision, ray: &Ray) -> (Option<Ray>, Option<Ray>);
    /// given then length a ray has travelled through a material, ruturn the
    /// attenuation.
    fn refract_attenuation(&self, length: f64) -> Colour;
    /// return the number of rays considered to be "effective". Perfectly
    /// reflective surfaces only need to cast one ray to check every
    /// possibility, but diffuse areas will require multiple rays.
    fn ray_count(&self, efficacy: f64) -> f64;
    // returns true if the object is a light source.
    fn is_light(&self) -> bool;
}
