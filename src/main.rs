mod body;
mod camera;
mod collision;
pub mod colour;
mod material;
mod ray;
mod scene;
pub mod shape;
pub mod vec3;
// mod monad;

use std::io;
use std::io::Write;

use body::{BasicBody, Body};
use camera::Camera;
use colour::Colour;
use material::Physical;
use scene::Scene;
use shape::march;
use shape::{Cuboid, Difference, Direction, Plane, Position, Shape, Sphere, Translate, Union};

fn main() -> io::Result<()> {
    let camera = Camera::new(
        Position::new(-20.0, -60.0, 40.0),
        Position::new(0.0, 0.0, 20.0),
        1.2, // size, x, y
        1.2,
        1.0, // aperture size
        Direction::UP,
        256, // resolution
        256,
    );

    let sun = Translate::new(Position::new(150.0, -150.0, 300.0), Sphere::new(100.0));
    let material_sun = Physical {
        refractive_index: Colour::new(0.0, 0.0, 0.0),
        refractive_absorption: None,
        emission: Some(Colour::new(150.0, 150.0, 150.0)),
        reflective_sharpness: None,
        refractive_sharpness: None,
    };

    let shape1 = Difference::new(Sphere::new(6.0), Cuboid::new(9.0, 9.0, 9.0));
    let shape1_vec = vec![
        Translate::new(Position::new(15.0, 15.0, 6.0), &shape1),
        Translate::new(Position::new(15.0, -15.0, 6.0), &shape1),
        Translate::new(Position::new(-15.0, 15.0, 6.0), &shape1),
        Translate::new(Position::new(-15.0, -15.0, 6.0), &shape1),
    ];
    let mut shape_1_dyn_vec: Vec<&(dyn Shape + Sync)> = Vec::new();
    shape1_vec.iter().for_each(|x| {
        shape_1_dyn_vec.push(x);
    });
    let shapes1 = Union::new(shape_1_dyn_vec);
    let material_shapes1 = Physical {
        refractive_index: Colour::new(1.5, 1.5, 1.5),
        refractive_absorption: Some(Colour::new(0.99, 0.97, 0.97)),
        emission: None, // Some(Colour::new(1.0, 0.0, 0.0)),
        reflective_sharpness: None,
        refractive_sharpness: None,
    };

    let material_shapes2 = Physical {
        refractive_index: Colour::new(1.5, 1.5, 1.5),
        refractive_absorption: Some(Colour::new(0.97, 0.97, 0.99)),
        emission: None, // Some(Colour::new(0.0, 1.0, 0.0)),
        reflective_sharpness: None,
        refractive_sharpness: None,
    };

    let _shapes2 = Translate::new(
        Position::new(0.0, 0.0, 15.0),
        march::MarchShape(march::sphere::Sphere::new(10.0)),
    );
    let _shapes2 = Translate::new(Position::new(0.0, 0.0, 15.0), Sphere::new(10.0));

    let shapes2 = Translate::new(
        Position::new(0.0, 0.0, 15.0),
        march::MarchShape(march::cuboid::Cuboid::new(20.0, 10.0, 14.0)),
    );
    let _shapes2 = Translate::new(Position::new(0.0, 0.0, 15.0), Cuboid::new(20.0, 10.0, 14.0));

    let _shapes2 = Translate::new(
        Position::new(0.0, 0.0, 15.0),
        Difference::new(Cuboid::new(20.0, 20.0, 20.0), Sphere::new(13.0)),
    );

    let ground = Plane::new();
    let material_ground = Physical {
        refractive_index: Colour::new(3.0, 3.0, 4.0),
        refractive_absorption: None, //Some(Colour::new(0.8, 0.81, 0.8)),
        emission: Some(Colour::new(0.1, 0.1, 0.1)),
        reflective_sharpness: Some(1.0),
        refractive_sharpness: None,
    };

    let bodies: Vec<Box<dyn Body + Sync>> = vec![
        Box::new(BasicBody {
            shape: &sun,
            material: material_sun,
        }),
        Box::new(BasicBody {
            shape: &shapes2,
            material: material_shapes2,
        }),
        Box::new(BasicBody {
            shape: &shapes1,
            material: material_shapes1,
        }),
        Box::new(BasicBody {
            shape: &ground,
            material: material_ground,
        }),
    ];

    let scene = Scene::new(5, 7, camera, bodies);
    let image = scene.render_ppm();

    let mut stdout = io::stdout();
    stdout.write(&image[..]).map(|_| ())
}
