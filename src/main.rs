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
    let image_size = Vector2i::xy(256, 256);
    let shader = PathTracerShader::new(Camera::new(
        Vector3f::xyz(0.0, 0.0, -1.0),
        Vector3f::xyz(0.0, 1.0, 0.0),
        90_f32.to_radians(),
        image_size,
    ));
    let mut img = Image::new(image_size.x() as u32, image_size.y() as u32);
    shader.apply(&mut img);
    img.save("test.ppm");
}
