use crate::colour::Colour;
use crate::vec3::Vec3;

use rand::Rng;

pub struct Material {
    // if None, then the object does not reflect.
    pub reflective_absorption: Option<Colour>,
    // if None, then the object does not refract.
    pub refractive_absorption: Option<Colour>,
    // if None, then the object does not implement diffuse light.
    pub diffuse_absorption: Option<Colour>,
    // 0.0 for lambertian, high value for specular, None for perfectly smooth
    pub reflective_sharpness: Option<f64>,
    // 0.0 for diffuse, high value for specular, None for perfectly smooth
    pub refractive_sharpness: Option<f64>,
    pub refractive_index: f64,
}

impl Material {
    pub fn is_reflective (&self) -> bool {
        self.reflective_absorption.is_some()
    }
    pub fn reflective_ray_count (&self, efficacy: f64) -> f64 {
        self.reflective_sharpness.map_or(1.0, |sharpness| {
            f64::ceil(efficacy / (1.0 + sharpness))
        })
    }
    pub fn reflect_direction (&self, direction: Vec3, normal: Vec3) -> Option<Vec3> {
        if !self.is_reflective() {
            return None;
        }

        let perfect_reflection = 
            (direction - normal.scale(2.0 * normal.dot(&direction)))
                .normalise();

        Some(self.reflective_sharpness.map_or(perfect_reflection, |sharpness| {
            Material::wobble_vector (perfect_reflection, normal.normalise(), sharpness)
        }))
    }

    pub fn is_refractive (&self) -> bool {
        self.refractive_absorption.is_some()
    }
    pub fn refractive_ray_count (&self, efficacy: f64) -> f64 {
        self.refractive_sharpness.map_or(1.0, |sharpness| {
            f64::ceil(efficacy / (1.0 + sharpness))
        })
    }
    pub fn refract_direction (&self, direction: Vec3, normal: Vec3) 
        -> Option<Vec3> 
    {
        if !self.is_refractive() {
            return None;
        }
        assert!(direction.normalised() && normal.normalised());

        // taken from wikipedia page on snell's law.
        let r = 1.0 / self.refractive_index;
        let c = - (normal.dot(&direction));
        assert!(c >= 0.0);

        // if determinant negative, then total internal reflection
        let determinant = 1.0 - r * r * (1.0 - c * c);

        if determinant < 0.0 { 
            // total internal refraction. object has lower refractivity index
            // than air
            return None;
        } 
        let perfect_refraction = 
            direction.scale(r) + normal.scale(r * c - f64::sqrt(determinant));

        Some(self.refractive_sharpness.map_or(perfect_refraction, |refractivity| {
            Material::wobble_vector (perfect_refraction, normal.scale(-1.0), refractivity)
        }))
    }

    // normal points to outside of shape. true for inside, false for outside.
    pub fn refract_direction_out (&self, direction: Vec3, normal: Vec3) 
        -> Option<(Vec3, bool)> 
    {
        if !self.is_refractive() {
            return None;
        }
        assert!(direction.normalised() && normal.normalised());

        // taken from wikipedia page on snell's law.
        let r = self.refractive_index;
        let c = normal.dot(&direction);
        assert!(c >= 0.0);

        // if determinant negative, then total internal reflection
        let determinant = 1.0 - r * r * (1.0 - c * c);

        if determinant < 0.0 { 
            // Total internal reflection
            let perfect_reflection = 
                direction - normal.scale(2.0 * normal.dot(&direction));

            let vec_out = self.refractive_sharpness
                .map_or(perfect_reflection, |refractivity| {
                    Material::wobble_vector (
                        perfect_reflection, normal.scale(-1.0), refractivity)
                });
            Some((vec_out, true))
        } 
        else { 
            let perfect_refraction = 
                (direction.scale(r) + normal.scale(r * c - f64::sqrt(determinant)))
                    .normalise();

            let vec_out = self.refractive_sharpness
                .map_or(perfect_refraction, |refractivity| {
                    Material::wobble_vector (
                        perfect_refraction, normal, refractivity)
                });
            Some((vec_out, false))
        }
    }
    pub fn refract_attenuation (&self, length: f64) -> Colour {
        self.refractive_absorption.map_or(Colour::BLACK, |absorption| {
            absorption.powf(length)
        })
    }

    fn wobble_vector (vector: Vec3, normal: Vec3, factor: f64) -> Vec3 {
        assert!(vector.dot(&normal) > 0.0);
        assert!(normal.normalised() && vector.normalised());

        let mut rng = rand::thread_rng();
        let random_vec = loop {
            // Wolfram math world sphere point picking (Marsaglia 1972)
            let a = rng.gen::<f64>() * 2.0 - 1.0;
            let b = rng.gen::<f64>() * 2.0 - 1.0;
            let squares = a * a + b * b;

            if squares >= 1.0 {
                continue;
            }

            // a^2 + b^2 + c^2 = 1
            let c = f64::sqrt(1.0 - squares);
            let x = 2.0 * a * c;
            let y = 2.0 * b * c;
            let z = 1.0 - 2.0 * squares;

            break Vec3::new(x, y, z);
        };

        let random_hemisphere = match random_vec.dot(&normal) < 0.0 {
            true => random_vec.scale(-1.0),
            false => random_vec,
        };
        assert!(random_hemisphere.normalised());

        let ratio = random_hemisphere.dist(normal) / 2.0;

        (vector + (random_hemisphere - vector).scale(ratio.powf(factor)))
            .normalise()
    }
}