

use std::io;
use std::io::Write;
use rand::Rng;

pub mod vec3;
use vec3::Vec3;

mod shape;
use shape::{Shape, Collision, Sphere, Translate, Scale, Union};

mod body;
use body::{Ray, Body};

mod material;
use material::Material;

pub mod colour;
use colour::Colour;

fn main() -> io::Result<()> {
    print!("P6 512 512 255 ");
    let mut stdout = io::stdout();
    let mut rng = rand::thread_rng();

    let size: f64 = 2.0;
    let samples: u32 = 100;

    let cam_position = Vec3::new(-3.0, -30.0, 16.0);
    let focal_point = Vec3::new(4.0, 0.0, 5.0);
    let direction = (focal_point - cam_position).normalise();
    
    let right = direction.cross(&Vec3::Z).normalise().scale(0.05);
    let down = direction.cross(&right).normalise().scale(0.05);
    let top_left = focal_point - (right + down).scale(256.0);

    let _res : io::Result<()> = (0..512).try_for_each(|y| 
        (0..512).try_for_each(|x| {
            let mut colour = Colour::new(0.0, 0.0, 0.0);

            (0..samples).for_each(|_| {
                let start_dy: f64 = rng.gen();
                let start_dx: f64 = rng.gen();
                let end_dx: f64 = rng.gen();
                let end_dy:f64 = rng.gen();

                let start = cam_position + right.scale(size * start_dx) +
                    down.scale(size * start_dy);
                let end = top_left + right.scale((x as f64) + end_dx) + 
                    down.scale((y as f64) + end_dy);

                let direction = (end - start).normalise();
                colour = colour + sampler (&Ray {
                    origin: start,
                    direction: direction,
                    attenuation: Colour::new(1.0, 1.0, 1.0),
                });
            });

            colour = colour.brighten(1.0/(samples as f64));

            stdout.write(&colour.to_bytes())?;
            Ok(())
    }));

    Ok(())
}

fn sampler (Ray{origin, direction, attenuation}: &Ray) -> Colour {
    let direction = direction.normalise();
    let scene = Union::new (vec![
        Box::new(
            Scale::new(Vec3::new(2.0, 1.0, 1.0),
            Translate::new(Vec3::new(4.0, 0.0, 13.0), 
            Sphere::new(5.0)))),
        Box::new(
            Translate::new(Vec3::new(-4.0, 0.0, 5.0), 
            Sphere::new(4.0))),
        Box::new(
            Translate::new(Vec3::new(4.0, 0.0, 5.0), 
            Sphere::new(3.0))),
        Box::new(
            Translate::new(Vec3::new(4.0, -7.0, 8.0), 
            Sphere::new(3.0))),
    ]);
    let material = Material {
        reflective_absorption: Some(Colour::new(0.2, 0.2, 0.2)),
        refractive_absorption: Some(Colour::new(0.8, 0.95, 0.8)),
        diffuse_absorption: Some(Colour::new(0.9, 0.4, 0.4)),
        // 0.0 for lambertian, high value for specular, None for perfectly sharp
        reflective_sharpness: Some(2.0),
        // 0.0 for diffuse, high value for specular, None for perfectly sharp
        refractive_sharpness: None,
        refractive_index: 1.2,
    };
    let body = Body {shape: scene, material: material};
    let rays = body.get_rays(*origin, direction);

    // possibly dangerous. Dark material which does not reflect or refract may
    // appear tranparent
    if rays.len() == 0 {
        if Vec3::Z.dot(&direction) > 0.0 {
            return Colour::new(300.0, 600.0, 700.0)
                .brighten(direction.z / 2.0 + 0.5)
                .brighten_colour(*attenuation);
        }

        let t = (-origin.z) / direction.z;
        let collision = *origin + direction.scale(t);
        if (f64::ceil(collision.x / 3.0) + 
            f64::ceil(collision.y / 3.0)) as i32 % 2 == 0
        {
            Colour::new(20.0, 20.0, 20.0).brighten_colour(*attenuation)
        }
        else {
            Colour::new(300.0, 300.0, 300.0).brighten_colour(*attenuation)
        }
    }
    else {
        let mut colour = Colour::BLACK;
        for ray in rays.iter() {
            colour = (colour + sampler(ray)).brighten_colour(*attenuation);
        }
        colour
    }
}