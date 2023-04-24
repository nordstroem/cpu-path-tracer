use geometry::{Camera, Hittable, Material, Ray};
use image::{gamma_correct, Color, Image};
use matrix::Vector3f;
use rng::Rng;

pub struct Renderer {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Hittable>>,
    pub ambient_light_color: Color,
    pub max_depth: u32,
    pub samples_per_pixel: u32,
}

impl Renderer {
    pub fn render(&self, seed: u32) -> Image {
        let image_size = self.camera.sensor_size_px;
        let mut rng = Rng::new(seed);
        let mut image = Image::new(image_size.x() as u32, image_size.y() as u32);
        let width = image.width;
        image.data.iter_mut().enumerate().for_each(|(i, color)| {
            let x = i as u32 % width;
            let y = i as u32 / width;
            *color = self.compute_color_for_pixel(x, y, &mut rng);
        });
        image
    }

    pub fn average_render(&self, seeds: &[u32]) -> Image {
        let image_size = self.camera.sensor_size_px;
        let weight = 1.0 / seeds.len() as f32;

        std::thread::scope(|s| {
            let mut threads = Vec::new();
            for seed in seeds {
                threads.push(s.spawn(move || self.render(*seed)));
            }
            let mut average_image = Image::new(image_size.x() as u32, image_size.y() as u32);
            for thread in threads {
                let image = thread.join().unwrap();
                for i in 0..image.data.len() {
                    average_image.data[i] += image.data[i] * weight;
                }
            }
            average_image
        })
    }

    fn compute_color_for_pixel(&self, x: u32, y: u32, rng: &mut Rng) -> Color {
        let x = x as f32;
        let y = y as f32;

        let mut color = Color::rgb(0.0, 0.0, 0.0);
        for _ in 0..self.samples_per_pixel {
            let x = x + (rng.uniform() - 0.5);
            let y = y + (rng.uniform() - 0.5);
            let ray = self.camera.back_project(x, y);
            color += self.compute_color_for_ray(&ray, rng, self.max_depth);
        }
        gamma_correct(color / (self.samples_per_pixel as f32)).clamp(0.0, 1.0)
    }

    fn compute_color_for_ray(&self, ray: &Ray, rng: &mut Rng, max_depth: u32) -> Color {
        if max_depth == 0 {
            return Color::rgb(0.0, 0.0, 0.0);
        }

        let compare = |a: &Vector3f, b: &Vector3f| {
            (ray.origin.squared_distance(&a))
                .partial_cmp(&ray.origin.squared_distance(&b))
                .unwrap_or(std::cmp::Ordering::Equal)
        };
        const MIN_DISTANCE: f32 = 1e-3;
        if let Some((intersection_point, object)) = self
            .objects
            .iter()
            .filter_map(|object| {
                object
                    .intersection(ray, MIN_DISTANCE)
                    .and_then(|hit| Some((hit, object)))
            })
            .min_by(|(a, _), (b, _)| compare(a, b))
        {
            let normal = object.normal(&ray, intersection_point);
            let target = intersection_point + normal + rng.unit_vector();
            let ray = Ray {
                origin: intersection_point,
                direction: (target - intersection_point).normalized(),
            };
            let color = match object.material() {
                Material::Lambertian { albedo } => albedo,
            };
            return self.compute_color_for_ray(&ray, rng, max_depth - 1) * color;
        } else {
            self.ambient_light_color
        }
    }
}
