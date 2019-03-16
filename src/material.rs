use crate::colour::Colour;
use crate::vec3::Vec3;

use rand::Rng;

pub struct Material {
    reflection: Colour,
    refraction: Option<Colour>,
    // 0.0 for lambertian, high value for specularity. None for perfect
    // specularity
    specularity: Option<f64>,
    optical_density: f64,
    diffuse: Colour,
}

impl Material {
    pub fn new (reflection: Colour, refraction: Option<Colour>, 
        specularity: Option<f64>, optical_density: f64, diffuse: Colour) 
        -> Material 
    {
        Material {
            reflection: reflection,
            refraction: refraction,
            specularity: specularity,
            optical_density: optical_density,
            diffuse: diffuse,
        }
    }
    pub fn diffuse_colour (&self, light_direction: Vec3, normal: Vec3, 
        colour: Colour) -> Colour 
    {
        let incidence = light_direction.normalise().dot(&normal.normalise());
        colour.brighten_colour(self.diffuse.brighten(incidence))
    }
    pub fn reflect_direction (&self, direction: Vec3, normal: Vec3) -> Vec3 {
        let perfect_reflection = 
            (direction - normal.scale(2.0 * normal.dot(&direction)))
                .normalise();

        self.specularity.map_or(perfect_reflection, |specularity| {
            let mut rng = rand::thread_rng();
            let mut random_vec = loop {
                let x = rng.gen::<f64>() * 2.0 - 1.0;
                let y = rng.gen::<f64>() * 2.0 - 1.0;
                let z = rng.gen::<f64>() * 2.0 - 1.0;
                if x*x + y*y + z*z < 1.0 { 
                    break Vec3::new(x, y, z).normalise();
                }
            };

            let modified_vec = random_vec + (perfect_reflection - random_vec).scale(specularity);

            match modified_vec.dot(&normal) < 0.0 {
                true => modified_vec.scale(-1.0),
                false => modified_vec,
            }
        })
    }
    pub fn reflect_colour (&self, colour: Colour) -> Colour {
        colour.brighten_colour(self.reflection)
    }
    pub fn refract (&self, _direction: Vec3, _normal: Vec3) -> Vec3 {
        unimplemented!()
    }
}