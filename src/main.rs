#[macro_use]
mod approx;
mod geometry;
mod image;
mod matrix;
mod renderer;
mod rng;

use geometry::{Camera, Hittable, Sphere};
use matrix::{Vector2i, Vector3f};
use renderer::Renderer;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let camera = Camera::new(
        Vector3f::xyz(0.0, 0.0, -1.0),
        Vector3f::xyz(0.0, 1.0, 0.0),
        100_f32.to_radians(),
        Vector2i::xy(326, 256),
    );
    let objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vector3f::xyz(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3f::xyz(0.8, -0.4, -1.0), 0.3)),
        Box::new(Sphere::new(Vector3f::xyz(0.0, -20.5, 0.0), 20.0)),
    ];
    let renderer = Renderer {
        camera: camera,
        objects: objects,
        ambient_light_color: Vector3f::rgb(1.0, 1.0, 1.0),
        max_depth: 25,
        samples_per_pixel: 25,
    };
    let seeds = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let img = renderer.average_render(&seeds);
    img.save("test.ppm");
    let elapsed = now.elapsed();
    println!("Elapsed: {}ms", elapsed.as_millis());
}
