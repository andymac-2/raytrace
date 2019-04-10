use crate::shape::{Direction, Position};
use rand::Rng;

/// A basic camera with rectillinear projection and a square apeture.
pub struct Camera {
    position: Position,
    down: Direction,
    right: Direction,
    top_left: Position,
    aperture_size: f64,
    pixel_width: f64,
    pixel_height: f64,
    x_resolution: u32,
    y_resolution: u32,
}

impl Camera {
    pub fn new(
        position: Position,
        focal_point: Position,
        width: f64,
        height: f64,
        aperture_size: f64,
        up: Direction,
        x_resolution: u32,
        y_resolution: u32,
    ) -> Camera {
        let direction = Direction::from_two_points(&position, &focal_point);
        let length = direction.len();

        let right = direction.cross(&up).normalise();
        let down = direction.cross(&right).normalise();
        let top_left = focal_point
            .move_along(&down, -(length * height / 2.0))
            .move_along(&right, -(length * width / 2.0));
        let pixel_width = length * width / (x_resolution as f64);
        let pixel_height = length * height / (y_resolution as f64);

        Camera {
            position,
            down,
            right,
            top_left,
            aperture_size,
            pixel_width,
            pixel_height,
            x_resolution,
            y_resolution,
        }
    }

    pub fn resolution(&self) -> (u32, u32) {
        (self.x_resolution, self.y_resolution)
    }

    pub fn generate_ray(&self, x: f64, y: f64) -> (Position, Direction) {
        let mut rng = rand::thread_rng();
        let start_dy: f64 = rng.gen();
        let start_dx: f64 = rng.gen();
        let end_dx: f64 = rng.gen();
        let end_dy: f64 = rng.gen();

        let start = self
            .position
            .move_along(&self.down, self.aperture_size * start_dy)
            .move_along(&self.right, self.aperture_size * start_dx);
        let end = self
            .top_left
            .move_along(&self.down, (y + end_dy) * self.pixel_height)
            .move_along(&self.right, (x + end_dx) * self.pixel_width);

        let direction = Direction::from_two_points(&start, &end).normalise();

        (start, direction)
    }
}
