use crate::vec3::Vec3;
use crate::body::{Body, Ray};
use crate::colour::Colour;
use crate::shape::Shape;

pub struct Scene<T> {
    pub body: Body<T>,
}

impl<T: Shape> Scene<T> {
    pub fn sampler (&self, Ray{origin, direction, attenuation}: &Ray) -> Colour {
        let direction = direction.normalise();
        let rays = self.body.get_rays(*origin, direction);

        // possibly dangerous. Dark material which does not reflect or refract may
        // appear tranparent
        if rays.len() == 0 {
            let intensity = direction
                .dot(&Vec3::new(5.0, 0.0, 10.0).normalise())
                .max(0.0)
                .powi(50);
            let ambient = Colour::new(50.0, 200.0, 300.0);
            let sun = Colour::new(500.0, 500.0, 400.0);
            ambient + sun.brighten(100.0 * intensity)

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
        else {
            let mut colour = Colour::BLACK;
            for ray in rays.iter() {
                colour = (colour + self.sampler(ray)).brighten_colour(*attenuation);
            }
            colour
        }
    }
}