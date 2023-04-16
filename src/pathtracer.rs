use geometry::{Camera, HitData, Hittable, Ray, Sphere};
use image::{Color, Shader};
use matrix::Vector3f;

pub struct PathTracerShader {
    pub camera: Camera,
    objects: Vec<Box<dyn Hittable>>,
}

impl PathTracerShader {
    pub fn new(camera: Camera) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Sphere::new(Vector3f::xyz(0.0, 0.0, -1.0), 0.5)));
        objects.push(Box::new(Sphere::new(Vector3f::xyz(1.0, 1.0, -2.0), 0.5)));
        Self { camera, objects }
    }
}

impl Shader for PathTracerShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let x = x as f32;
        let y = y as f32;
        let sample_points = [
            (x - 0.25, y - 0.25),
            (x + 0.25, y - 0.25),
            (x - 0.25, y + 0.25),
            (x + 0.25, y + 0.25),
            (x, y),
        ];
        let mut color = Color::rgb(0.0, 0.0, 0.0);
        for (x, y) in &sample_points {
            let ray = self.camera.back_project(*x, *y);
            color += self.compute_color_for_ray(&ray);
        }
        color / (sample_points.len() as f32)
    }
}

impl PathTracerShader {
    fn compute_color_for_ray(&self, ray: &Ray) -> Color {
        let compare = |a: &HitData, b: &HitData| {
            (ray.origin - a.intersection_point)
                .squared_length()
                .partial_cmp(&(ray.origin - b.intersection_point).squared_length())
                .unwrap()
        };

        if let Some(hit) = self
            .objects
            .iter()
            .filter_map(|object| object.intersect(ray))
            .min_by(compare)
        {
            Color::rgb(
                0.5 * (1.0 + hit.normal.x()),
                0.5 * (1.0 + hit.normal.y()),
                0.5 * (1.0 + hit.normal.z()),
            )
        } else {
            Color::rgb(0.0, 0.0, 0.0)
        }
    }
}
