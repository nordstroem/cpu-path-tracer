use std::fs::File;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let black = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        };
        Image {
            width,
            height,
            data: vec![black; (width * height) as usize],
        }
    }

    pub fn save(&self, filename: &str) {
        assert!(filename.ends_with(".ppm"));
        let mut file = File::create(filename).unwrap();
        file.write_all(&self.to_ppm()).unwrap();
    }

    fn to_ppm(&self) -> Vec<u8> {
        let float_to_byte = |f: f32| ((f.max(0.0).min(1.0) * 255.0).round()) as u8;
        let header = format!("P6 {} {} 255 ", self.width, self.height);
        let data_bytes: Vec<u8> = self
            .data
            .iter()
            .map(|c| [c.r, c.g, c.b])
            .flatten()
            .map(float_to_byte)
            .collect();
        header
            .as_bytes()
            .iter()
            .chain(data_bytes.iter())
            .cloned()
            .collect()
    }
}

pub trait Shader {
    fn compute_color(&self, x: u32, y: u32) -> Color;

    fn apply(&self, image: &mut Image) {
        let width = image.width;
        image.data.iter_mut().enumerate().for_each(|(i, color)| {
            let x = i as u32 % width;
            let y = i as u32 / width;
            *color = self.compute_color(x, y);
        })
    }
}

// This is a wrapper around a closure that implements the Shader trait.
// This allows us to pass a closure to the apply_shader function.
pub struct ClosureShader<F: Fn(u32, u32) -> Color>(F);

impl<F: Fn(u32, u32) -> Color> Shader for ClosureShader<F> {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        (self.0)(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image() {
        let img = Image::new(100, 100);
        assert_eq!(img.width, 100);
        assert_eq!(img.height, 100);
        assert_eq!(img.data.len(), 10000);
    }

    #[test]
    fn test_to_ppm() {
        let img = Image::new(100, 100);
        let ppm = img.to_ppm();
        let expected_header = "P6 100 100 255 ";
        let expected_num_bytes = img.width * img.height * 3 + expected_header.len() as u32;
        assert_eq!(ppm.len(), expected_num_bytes as usize);
        assert!(ppm.starts_with(expected_header.as_bytes()));
        assert!(ppm.ends_with(&[0, 0, 0][..]));
    }

    #[test]
    fn test_apply_shader() {
        let shader = ClosureShader(|x, y| Color {
            r: x as f32,
            g: y as f32,
            b: 0.0,
        });
        let mut image = Image::new(2, 2);
        shader.apply(&mut image);
        assert_eq!(image.data[0], Color::rgb(0.0, 0.0, 0.0));
        assert_eq!(image.data[1], Color::rgb(1.0, 0.0, 0.0));
        assert_eq!(image.data[2], Color::rgb(0.0, 1.0, 0.0));
        assert_eq!(image.data[3], Color::rgb(1.0, 1.0, 0.0));
    }
}
