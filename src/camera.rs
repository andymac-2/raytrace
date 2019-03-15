mod vec3;

use vec3::Vec3

struct Camera {
    position: Vec3,
    size: f64,
    focal_point: Vec3,
    up: Vec3,
    left: Vec3,
}

impl Camera {
    fn generate_ray (&self, x: f64, y: f64) {
        let mut rng = thread_rng();

        start = position + 
            self.up.scale(rng.gen() * self.size) + 
            self.left.scale(rng.gen() * self.size);
        end = focal_point +
            self.up.scale(y) + 
            self.left.scale(x);

        end - start;
    }
}