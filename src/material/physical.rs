use crate::collision::Collision;
use crate::colour::Colour;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Direction;

pub struct Physical {
    /// refractive index: real part. Corresponds to the reflective absorption.
    /// High values mean more light reflected.
    pub refractive_index: Colour,
    /// Three channels for the complex part of the refractive index for red
    /// green and blue respectively. low values absorb light. High values allow
    /// light to transmit. each value should lie between 0.0 and 1.0.
    pub refractive_absorption: Option<Colour>,
    /// if None, then the object does not implement diffuse light, otherwise it
    /// will emit light of it's own accord.
    pub emission: Option<Colour>,
    /// Albedo. 0.0 for lambertian, high value for specular, None for infinitely
    /// smooth
    pub reflective_sharpness: Option<f64>,
    /// 0.0 for diffuse (marble), high value for glassy/crystalline. None for
    /// perfectly glassy (infinite sharpness)
    pub refractive_sharpness: Option<f64>,
}

impl Material for Physical {
    fn emission(&self) -> Colour {
        self.emission
            .as_ref()
            .map_or(Colour::BLACK, |colour| colour.brighten(200.0))
    }
    fn rays(&self, collision: &Collision, ray_in: &Ray) -> (Option<Ray>, Option<Ray>) {
        assert!(collision.direction().normalised());
        assert!(collision.normal().normalised());

        let flipped = collision.flip_normal();
        let (collision, refractive_index) = if collision.direction().dot(&collision.normal()) < 0.0
        {
            (collision, self.avg_refractive_index())
        } else {
            (&flipped, 1.0 / self.avg_refractive_index())
        };

        let opt_refraction_dir = self.refract_direction(collision, refractive_index);
        if let Some(refraction_dir) = opt_refraction_dir {
            (
                self.reflection(collision)
                    .map(|new_ray| new_ray.attenuate(&ray_in.attenuation())),
                self.refraction(collision, refraction_dir)
                    .map(|new_ray| new_ray.attenuate(&ray_in.attenuation())),
            )
        } else {
            (
                Some(
                    self.total_internal_reflection(collision)
                        .attenuate(&ray_in.attenuation()),
                ),
                None,
            )
        }
    }
    fn refract_attenuation(&self, length: f64) -> Colour {
        self.refractive_absorption
            .as_ref()
            .map_or(Colour::BLACK, |absorption| absorption.powf(length))
    }
    fn ray_count(&self, efficacy: f64) -> f64 {
        self.reflective_sharpness
            .map_or(1.0, |sharpness| f64::ceil(efficacy / (1.0 + sharpness)))
    }
}

impl Physical {
    fn refraction(&self, collision: &Collision, refract_direction: Direction) -> Option<Ray> {
        if self.is_refractive() {
            let cos_incidence = -(collision.normal().dot(&collision.direction()));

            let attenuation = self.initial_refract_attenuation(cos_incidence);
            let origin = collision.collision();
            Some(Ray::new(origin.clone(), refract_direction, attenuation))
        } else {
            None
        }
    }
    fn reflection(&self, collision: &Collision) -> Option<Ray> {
        if self.is_reflective() {
            let cos_incidence = -(collision.normal().dot(&collision.direction()));

            let reflection_direction = self.reflect_direction(collision);
            let attenuation = self.reflection_attenuation(cos_incidence);
            let origin = collision.collision();
            Some(Ray::new(origin.clone(), reflection_direction, attenuation))
        } else {
            None
        }
    }
    fn total_internal_reflection(&self, collision: &Collision) -> Ray {
        let reflection_direction = self.reflect_direction(collision);
        let attenuation = Colour::new(1.0, 1.0, 1.0);
        let origin = collision.collision();
        Ray::new(origin.clone(), reflection_direction, attenuation)
    }

    fn refract_direction(&self, collision: &Collision, refractive_index: f64) -> Option<Direction> {
        assert!(collision.direction().dot(&collision.normal()) <= 0.0);
        collision
            .direction()
            .refraction(&collision.normal(), refractive_index)
            .map(|perfect_refraction| match self.reflective_sharpness {
                Some(sharpness) => {
                    perfect_refraction.wobble(&collision.normal().negate(), sharpness)
                }
                None => perfect_refraction,
            })
    }
    fn reflect_direction(&self, collision: &Collision) -> Direction {
        let reflection = collision.direction().reflection(&collision.normal());
        match self.reflective_sharpness {
            Some(sharpness) => reflection.wobble(&collision.normal(), sharpness),
            None => reflection,
        }
    }
    fn is_reflective(&self) -> bool {
        let index = &self.refractive_index;
        index.red() + index.green() + index.blue() > 0.0
    }
    fn is_refractive(&self) -> bool {
        self.refractive_absorption.is_some()
    }
    fn avg_refractive_index(&self) -> f64 {
        (self.refractive_index.red() + self.refractive_index.green() + self.refractive_index.blue())
            / 3.0
    }
    fn initial_refract_attenuation(&self, cos_incidence: f64) -> Colour {
        // conservation of energy
        let reflective_attenuation = self.reflection_attenuation(cos_incidence);
        Colour::new(
            1.0 - reflective_attenuation.red(),
            1.0 - reflective_attenuation.green(),
            1.0 - reflective_attenuation.blue(),
        )
    }
    fn reflection_attenuation(&self, cos_incidence: f64) -> Colour {
        Colour::new(
            Physical::reflection_coeficient(self.refractive_index.red(), cos_incidence),
            Physical::reflection_coeficient(self.refractive_index.green(), cos_incidence),
            Physical::reflection_coeficient(self.refractive_index.blue(), cos_incidence),
        )
    }
    fn reflection_coeficient(refractive_index: f64, cos_incidence: f64) -> f64 {
        // Shlick's approximation.
        // TODO: replace with fresnel equations later.
        let coef_normal = (1.0 - refractive_index) / (1.0 + refractive_index);
        let coef_normal = coef_normal * coef_normal;

        coef_normal + (1.0 - coef_normal) * (1.0 - cos_incidence).powi(5)
    }
}
