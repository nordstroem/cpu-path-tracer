use geometry::{Camera, HitData, Hittable, Ray, Sphere};
use image::{Color, Shader};
use matrix::Vector3f;

pub struct PathTracerShader {
    pub camera: Camera,
    objects: Vec<Box<dyn Hittable>>,
    seed: f32,
}

impl Shader for PathTracerShader {
    fn compute_color(&self, x: u32, y: u32) -> Color {
        let x = x as f32;
        let y = y as f32;

        let mut rng = Rng::new(self.seed + x + y * 100.0);
        let mut color = Color::rgb(0.0, 0.0, 0.0);
        let number_of_samples = 25;
        let max_depth = 25;
        for _ in 0..number_of_samples {
            let x = x + (rng.uniform() - 0.5);
            let y = y + (rng.uniform() - 0.5);
            let ray = self.camera.back_project(x, y);
            color += self.compute_color_for_ray(&ray, &mut rng, max_depth);
        }
        gamma_correct(color / (number_of_samples as f32)).clamp(0.0, 1.0)
    }
}

fn gamma_correct(color: Color) -> Color {
    Color::rgb(color.r().sqrt(), color.g().sqrt(), color.b().sqrt())
}

impl PathTracerShader {
    pub fn new(camera: Camera, seed: f32) -> Self {
        let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
        objects.push(Box::new(Sphere::new(Vector3f::xyz(0.0, 0.0, -1.0), 0.5)));
        objects.push(Box::new(Sphere::new(Vector3f::xyz(0.8, -0.4, -1.0), 0.3)));
        objects.push(Box::new(Sphere::new(Vector3f::xyz(0.0, -20.5, 0.0), 20.0)));
        Self {
            camera,
            objects,
            seed,
        }
    }

    fn compute_color_for_ray(&self, ray: &Ray, rng: &mut Rng, max_depth: i32) -> Color {
        let compare = |a: &HitData, b: &HitData| {
            (ray.origin.squared_distance(&a.intersection_point))
                .partial_cmp(&ray.origin.squared_distance(&b.intersection_point))
                .unwrap()
        };

        if max_depth <= 0 {
            return Color::rgb(0.0, 0.0, 0.0);
        }

        const MIN_DISTANCE: f32 = 1e-3;

        if let Some(hit) = self
            .objects
            .iter()
            .filter_map(|object| object.intersect(ray, MIN_DISTANCE))
            .min_by(compare)
        {
            let target = hit.intersection_point + hit.normal + rng.unit_sphere();
            let ray = Ray {
                origin: hit.intersection_point,
                direction: (target - hit.intersection_point).normalized(),
            };
            return self.compute_color_for_ray(&ray, rng, max_depth - 1) * 0.5;
        } else {
            Color::rgb(1.0, 1.0, 1.0) // Light color
        }
    }
}

struct Rng {
    seed: f32,
}

impl Rng {
    fn new(seed: f32) -> Self {
        Self { seed }
    }
    fn unit_sphere(&mut self) -> Vector3f {
        let mut p = Vector3f::xyz(1.0, 1.0, 1.0);
        while p.squared_length() >= 1.0 {
            p = Vector3f::xyz(
                2.0 * self.uniform() - 1.0,
                2.0 * self.uniform() - 1.0,
                2.0 * self.uniform() - 1.0,
            )
        }
        p
    }
    fn uniform(&mut self) -> f32 {
        let mut new_seed = self.seed.sin() * 43758.5453123;
        if new_seed == self.seed {
            new_seed += 0.01;
        }
        self.seed = new_seed;
        self.seed - self.seed.floor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_sphere() {
        let mut rng = Rng::new(0.0);
        let mut points = Vec::new();
        for _ in 0..100 {
            let point = rng.unit_sphere();
            assert!(point.squared_length() < 1.0);
            assert!(!points.contains(&point));
            points.push(point);
        }
    }

    #[test]
    fn test_uniform() {
        let mut rng = Rng::new(0.0);
        let mut numbers = Vec::new();
        for _ in 0..100 {
            numbers.push(rng.uniform());
        }
        let mut rng = Rng::new(0.0);
        for number in numbers {
            assert_eq!(number, rng.uniform());
        }
    }
}
