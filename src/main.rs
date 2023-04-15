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

fn main() {
    let shader = PathTracerShader {
        camera: Camera::new(
            Vector3f::xyz(0.0, 0.0, -1.0),
            Vector3f::xyz(0.0, 1.0, 0.0),
            90_f32.to_radians(),
            Vector2i::xy(256, 256),
        ),
    };
    let mut img = Image::new(256, 256);
    shader.apply(&mut img);
    img.save("test.ppm");
}
