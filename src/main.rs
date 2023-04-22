#[macro_use]
mod approx;
mod geometry;
mod image;
mod matrix;
mod pathtracer;

use geometry::Camera;
use image::{Image, Shader};
use matrix::{Vector2i, Vector3f};
use pathtracer::PathTracerShader;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let image_size = Vector2i::xy(326, 256);

    let mut images = Vec::new();
    let num_threads = 8;
    for i in 0..num_threads {
        images.push(std::thread::spawn(move || {
            let shader = PathTracerShader::new(
                Camera::new(
                    Vector3f::xyz(0.0, 0.0, -1.0),
                    Vector3f::xyz(0.0, 1.0, 0.0),
                    100_f32.to_radians(),
                    image_size,
                ),
                (i * 10) as f32,
            );
            let mut img = Image::new(image_size.x() as u32, image_size.y() as u32);
            shader.apply(&mut img);
            img
        }));
    }
    let mut img = Image::new(image_size.x() as u32, image_size.y() as u32);
    let weight = 1.0 / images.len() as f32;
    for image in images {
        let image = image.join().unwrap().data;
        for i in 0..image.len() {
            img.data[i] += image[i] * weight;
        }
    }
    img.save("test.ppm");
    let elapsed = now.elapsed();
    println!("Elapsed: {}ms", elapsed.as_millis());
}
