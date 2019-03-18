pub mod vec3;
pub mod colour;
mod shape;
mod material;
mod body;
mod scene;
// mod monad;

use std::io;
use std::io::Write;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use vec3::Vec3;
use shape::{Sphere, Translate, Scale, Union, Plane, Difference};
use material::Material;
use body::{Ray, Body};
use colour::Colour;
use scene::Scene;

fn main() -> io::Result<()> {
    print!("P6 512 512 255 ");
    let mut stdout = io::stdout();

    let size: f64 = 5.0;
    let samples: u32 = 500;

    let cam_position = Vec3::new(-3.0, -30.0, 16.0);
    let focal_point = Vec3::new(4.0, 0.0, 5.0);
    let direction = (focal_point - cam_position).normalise();
    
    let right = direction.cross(&Vec3::Z).normalise().scale(0.05);
    let down = direction.cross(&right).normalise().scale(0.05);
    let top_left = focal_point - (right + down).scale(256.0);

    let shape = Union::new (vec![
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
        Box::new(Difference::new(
            Box::new(
                Translate::new(Vec3::new(-4.0, 0.0, 10.0), 
                Sphere::new(3.0))),
            Box::new(
                Translate::new(Vec3::new(-4.0, 0.0, 12.0), 
                Sphere::new(3.0))))),
        Box::new(
            Translate::new(Vec3::new(4.0, -7.0, 8.0), 
            Sphere::new(3.0))),
        Box::new(
            Plane::new()),
    ]);
    let material = Material {
        reflective_absorption: Some(Colour::new(0.5, 0.5, 0.5)),
        refractive_absorption: Some(Colour::new(0.4, 0.6, 0.4)),
        diffuse_absorption: Some(Colour::new(0.9, 0.4, 0.4)),
        // 0.0 for lambertian, high value for specular, None for perfectly sharp
        reflective_sharpness: Some(5.0),
        // 0.0 for diffuse, high value for specular, None for perfectly sharp
        refractive_sharpness: None,
        refractive_index: 1.2,
    };
    let body = Body {shape: shape, material: material};
    let scene = Scene {body: body};

    let image: Vec<u8> = (0..512).into_par_iter().map(|y| {
        let mut rng = rand::thread_rng();
        (0..512).map(|x| {
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
                colour = colour + scene.sampler (&Ray {
                    origin: start,
                    direction: direction,
                    attenuation: Colour::new(1.0, 1.0, 1.0),
                });
            });

            colour = colour.brighten(1.0/(samples as f64));

            colour.to_bytes()
        }).flatten().collect::<Vec<u8>>()
    }).flatten().collect();

    stdout.write(&image[..]).map(|_| ())
}