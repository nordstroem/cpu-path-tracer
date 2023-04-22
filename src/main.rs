#[macro_use]
mod approx;
mod geometry;
mod image;
mod matrix;
mod renderer;
mod rng;

use geometry::{Camera, Sphere};
use matrix::{Vector2i, Vector3f};
use renderer::Renderer;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let image_size = Vector2i::xy(326, 256);
    let camera = Camera::new(
        Vector3f::xyz(0.0, 0.0, -1.0),
        Vector3f::xyz(0.0, 1.0, 0.0),
        100_f32.to_radians(),
        image_size,
    );
    let mut renderer = Renderer {
        camera: camera,
        objects: Vec::new(),
        ambient_light_color: Vector3f::rgb(1.0, 1.0, 1.0),
        max_depth: 25,
        samples_per_pixel: 25,
    };
    renderer.add_object(Box::new(Sphere::new(Vector3f::xyz(0.0, 0.0, -1.0), 0.5)));
    renderer.add_object(Box::new(Sphere::new(Vector3f::xyz(0.8, -0.4, -1.0), 0.3)));
    renderer.add_object(Box::new(Sphere::new(Vector3f::xyz(0.0, -20.5, 0.0), 20.0)));
    let img = renderer.render(42.0);
    img.save("test.ppm");
    let elapsed = now.elapsed();
    println!("Elapsed: {}ms", elapsed.as_millis());
}
