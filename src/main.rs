mod image;
use image::{Color, Image, Shader};

struct RainbowShader;

impl Shader for RainbowShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let r = (x % 256) as u8;
        let g = (y % 256) as u8;
        let b = 0;
        Color { r, g, b }
    }
}

fn main() {
    let shader = RainbowShader;
    let mut img = Image::new(256, 256);
    shader.apply(&mut img);
    img.save("test.ppm");
}
