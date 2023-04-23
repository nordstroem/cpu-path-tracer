use matrix::Vector3f;
use std::fs::File;
use std::io::Write;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        let black = Color::rgb(0.0, 0.0, 0.0);
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
            .map(|c| [c.r(), c.g(), c.b()])
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

pub type Color = Vector3f;

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color::xyz(r, g, b)
    }
    pub fn r(&self) -> f32 {
        self.x()
    }
    pub fn g(&self) -> f32 {
        self.y()
    }
    pub fn b(&self) -> f32 {
        self.z()
    }
}

pub fn gamma_correct(color: Color) -> Color {
    Color::rgb(color.r().sqrt(), color.g().sqrt(), color.b().sqrt())
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
}
