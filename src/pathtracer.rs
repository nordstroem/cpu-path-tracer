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
        objects.push(Box::new(Sphere::new(Vector3f::xyz(0.0, -20.5, 0.0), 20.0)));
        Self { camera, objects }
    }
}

struct Rng {
    seed: f32,
}

impl Rng {
    fn new(seed: f32) -> Self {
        Self { seed }
    }

    fn next(&mut self) -> f32 {
        self.seed = self.seed * 16807.0 % 2147483647.0;
        self.seed / 2147483647.0
    }
}

fn noise(seed: f32) -> f32 {
    let x = seed.sin() * 43758.5453123;
    x - x.floor()
}

impl Shader for PathTracerShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let x = x as f32;
        let y = y as f32;

        let mut rng = Rng::new(x + y * 1000.0);
        let mut color = Color::rgb(0.0, 0.0, 0.0);
        let number_of_samples = 5;
        for _ in 0..number_of_samples {
            let x = x + 0.5 * (rng.next() - 0.5);
            let y = y + 0.5 * (rng.next() - 0.5);
            let ray = self.camera.back_project(x, y);
            color += self.compute_color_for_ray(&ray);
        }
        color / (number_of_samples as f32)
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
