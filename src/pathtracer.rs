use geometry::{Camera, Sphere};
use image::{Color, Shader};
use matrix::Vector3f;

pub struct PathTracerShader {
    pub camera: Camera,
    sphere: Sphere,
}

impl PathTracerShader {
    pub fn new(camera: Camera) -> Self {
        let sphere = Sphere::new(Vector3f::xyz(0.0, 0.0, -1.0), 0.5);
        Self { camera, sphere }
    }
}

impl Shader for PathTracerShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let ray = self.camera.ray(x as f32, y as f32);
        let intersection_point = self.sphere.intersect(&ray);
        match intersection_point {
            Some(_) => Color::rgb(1.0, 0.0, 0.0),
            None => Color::rgb(0.0, 0.0, 0.0),
        }
    }
}
