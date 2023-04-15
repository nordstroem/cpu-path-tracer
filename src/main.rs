#[macro_use]
mod approx;
mod image;
mod matrix;
mod pathtracer;

use image::{Color, Image, Shader};

struct RainbowShader;

impl Shader for RainbowShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let r = (x % 256) as f32 / 255.0;
        let g = (y % 256) as f32 / 255.0;
        let b = 0.0;
        Color { r, g, b }
    }
}

fn main() {
    let shader = RainbowShader;
    let mut img = Image::new(256, 256);
    shader.apply(&mut img);
    img.save("test.ppm");
    println!("{}", 1);
}
