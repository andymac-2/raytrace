mod body;
mod collision;
pub mod colour;
mod material;
mod ray;
mod scene;
pub mod shape;
pub mod vec3;
// mod monad;

use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::io;
use std::io::Write;

use body::BasicBody;
use colour::Colour;
use material::Physical;
use ray::Ray;
use scene::Scene;
use shape::{Direction, Plane, Position, Sphere, Translate, Union};

fn main() -> io::Result<()> {
    print!("P6 512 512 255 ");
    let mut stdout = io::stdout();

    let size: f64 = 0.5;
    let samples: u32 = 20;

    let cam_position = Position::new(-8.0, -30.0, 25.0);
    let focal_point = Position::new(0.0, 0.0, 10.0);
    let direction = Direction::from_two_points(&cam_position, &focal_point).normalise();

    let right = direction.cross(&Direction::UP).normalise();
    let down = direction.cross(&right).normalise();
    let top_left = focal_point
        .move_along(&down, -(256.0 * 0.1))
        .move_along(&right, -(256.0 * 0.1));

    let sun = Translate::new(Position::new(150.0, -150.0, 300.0), Sphere::new(100.0));
    let material_sun = Physical {
        refractive_index: Colour::new(0.0, 0.0, 0.0),
        refractive_absorption: None,
        emission: Some(Colour::new(80.0, 80.0, 80.0)),
        reflective_sharpness: None,
        refractive_sharpness: None,
    };

    let shapes1 = Union::new(vec![
        // ground layer
        Box::new(Translate::new(
            Position::new(10.0, 10.0, 4.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(10.0, -10.0, 4.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(-10.0, 10.0, 4.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(-10.0, -10.0, 4.0),
            Sphere::new(4.0),
        )),
        // layer 2
    ]);
    let material_shapes = Physical {
        refractive_index: Colour::new(6.0, 6.0, 6.0),
        refractive_absorption: None, // Some(Colour::new(0.97, 0.98, 0.97)),
        emission: None,
        reflective_sharpness: None,
        refractive_sharpness: None,
    };

    let shapes2 = Union::new(vec![
        Box::new(Translate::new(
            Position::new(10.0, 0.0, 15.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(-10.0, 0.0, 15.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(0.0, 10.0, 15.0),
            Sphere::new(4.0),
        )),
        //layer 3
        Box::new(Translate::new(
            Position::new(3.0, 2.0, 10.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(38.0, 18.0, 10.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(16.0, 30.0, 10.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(-20.0, 40.0, 4.0),
            Sphere::new(4.0),
        )),
        Box::new(Translate::new(
            Position::new(-21.0, 15.0, 10.0),
            Sphere::new(4.0),
        )),
    ]);
    let material_subsurface = Physical {
        refractive_index: Colour::new(1.1, 1.05, 1.001),
        refractive_absorption: None,
        emission: Some(Colour::new(10.0, 0.0, 0.0)),
        reflective_sharpness: None,
        refractive_sharpness: None,
    };

    let ground = Plane::new();
    let material_ground = Physical {
        refractive_index: Colour::new(3.0, 3.0, 4.0),
        refractive_absorption: None, //Some(Colour::new(0.8, 0.81, 0.8)),
        emission: None,
        reflective_sharpness: Some(1.0),
        refractive_sharpness: None,
    };

    let scene = Scene {
        bounces: 7,
        bodies: vec![
            Box::new(BasicBody {
                shape: sun,
                material: material_sun,
            }),
            Box::new(BasicBody {
                shape: shapes2,
                material: material_shapes,
            }),
            Box::new(BasicBody {
                shape: shapes1,
                material: material_subsurface,
            }),
            Box::new(BasicBody {
                shape: ground,
                material: material_ground,
            }),
        ],
    };

    let image: Vec<u8> = (0..512)
        .into_par_iter()
        .map(|y| {
            let mut rng = rand::thread_rng();
            (0..512)
                .map(|x| {
                    let mut colour = Colour::new(0.0, 0.0, 0.0);

                    (0..samples).for_each(|_| {
                        let start_dy: f64 = rng.gen();
                        let start_dx: f64 = rng.gen();
                        let end_dx: f64 = rng.gen();
                        let end_dy: f64 = rng.gen();

                        let start = cam_position
                            .move_along(&down, size * start_dy)
                            .move_along(&right, size * start_dx);
                        let end = top_left
                            .move_along(&down, ((y as f64) + end_dy) * 0.1)
                            .move_along(&right, ((x as f64) + end_dx) * 0.1);

                        let direction = Direction::from_two_points(&start, &end).normalise();
                        colour = &colour
                            + &scene.sampler(
                                &Ray::new(start, direction, Colour::new(1.0, 1.0, 1.0)),
                                0,
                            );
                    });

                    colour = colour.brighten(1.0 / (samples as f64));

                    colour.to_bytes()
                })
                .flatten()
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect();

    stdout.write(&image[..]).map(|_| ())
}
