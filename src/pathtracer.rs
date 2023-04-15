use geometry::Camera;
use image::{Color, Shader};

pub struct PathTracerShader {
    pub camera: Camera,
}

impl Shader for PathTracerShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let ray = self.camera.ray(x as f32, y as f32);
        let r = (ray.direction.x() + 1.0) * 0.5;
        let g = (ray.direction.y() + 1.0) * 0.5;
        let b = -ray.direction.z();
        Color { r, g, b }
    }
}
