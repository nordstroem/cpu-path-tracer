use matrix::{Vector2f, Vector2i, Vector3f};

pub struct Camera {
    forward: Vector3f,
    up: Vector3f,
    right: Vector3f,
    focal_length: f32,
    principal_point: Vector2f,
    pub sensor_size_px: Vector2i,
}

impl Camera {
    pub fn new(forward: Vector3f, up: Vector3f, fov_rad: f32, sensor_size_px: Vector2i) -> Self {
        let forward = forward.normalized();
        let right = forward.cross(&up).normalized();
        let up = right.cross(&forward).normalized();
        let sensor_size = sensor_size_px.x().max(sensor_size_px.y()) as f32;
        let focal_length = (0.5 * (sensor_size - 1.0) / (fov_rad * 0.5).tan()).abs();
        let principal_point = Vector2f::xy(
            sensor_size_px.x() as f32 / 2.0 - 0.5,
            sensor_size_px.y() as f32 / 2.0 - 0.5,
        );
        Self {
            forward,
            up,
            right,
            focal_length,
            principal_point,
            sensor_size_px,
        }
    }

    pub fn back_project(&self, x: f32, y: f32) -> Ray {
        let x = x - self.principal_point.x();
        let y = -(y - self.principal_point.y());
        Ray {
            origin: Vector3f::zeros(),
            direction: (self.forward * self.focal_length + self.right * x + self.up * y)
                .normalized(),
        }
    }
}

pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3f {
        self.origin + self.direction * t
    }
}

pub enum Material {
    Lambertian { albedo: Vector3f },
}
pub struct HitData {
    pub intersection_point: Vector3f,
    pub normal: Vector3f,
}

pub trait Hittable: Sync {
    fn intersect(&self, ray: &Ray, min_distance: f32) -> Option<HitData>;
}

pub struct Sphere {
    pub center: Vector3f,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, min_distance: f32) -> Option<HitData> {
        let oc = ray.origin - self.center;
        let half_p = ray.direction.dot(&oc);
        let q = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_p * half_p - q;

        if discriminant < 0.0 {
            return None;
        }
        let t1 = -half_p - discriminant.sqrt();
        let t2 = -half_p + discriminant.sqrt();

        let intersection_point = match (t1 > min_distance, t2 > min_distance) {
            (true, _) => ray.at(t1),
            (false, true) => ray.at(t2),
            (false, false) => return None,
        };

        let mut normal = (intersection_point - self.center) / self.radius;
        if normal.dot(&ray.direction) > 0.0 {
            normal = normal * -1.0;
        }
        Some(HitData {
            intersection_point,
            normal,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_ray_forward() {
        let camera = Camera::new(
            Vector3f::xyz(0.0, 0.0, -1.0),
            Vector3f::xyz(0.0, 1.0, 0.0),
            90_f32.to_radians(),
            Vector2i::xy(256, 256),
        );

        let ray = camera.back_project(127.5, 127.5);
        let tol = 1e-6;
        assert_approx!(ray.direction.x(), 0.0, tol);
        assert_approx!(ray.direction.y(), 0.0, tol);
        assert_approx!(ray.direction.z(), -1.0, tol);
        assert_approx!(ray.origin.x(), 0.0, tol);
        assert_approx!(ray.origin.y(), 0.0, tol);
        assert_approx!(ray.origin.z(), 0.0, tol);
    }

    #[test]
    fn test_camera_ray_left_side() {
        let forward = Vector3f::xyz(0.0, 0.0, -1.0);
        let up = Vector3f::xyz(0.0, 1.0, 0.0);
        let camera = Camera::new(forward, up, 90_f32.to_radians(), Vector2i::xy(256, 256));

        let ray = camera.back_project(0.0, 127.5);
        let tol = 1e-6;
        assert!(ray.direction.x() < 0.0);
        assert_approx!(
            ray.direction.cos_angle(&forward),
            45_f32.to_radians().cos(),
            tol
        );
    }
    #[test]
    fn test_camera_ray_up_side() {
        let forward = Vector3f::xyz(0.0, 0.0, -1.0);
        let up = Vector3f::xyz(0.0, 1.0, 0.0);
        let camera = Camera::new(forward, up, 90_f32.to_radians(), Vector2i::xy(256, 512));

        let ray = camera.back_project(127.5, 0.0);
        let tol = 1e-6;
        assert!(ray.direction.y() > 0.0);
        assert_approx!(
            ray.direction.cos_angle(&forward),
            45_f32.to_radians().cos(),
            tol
        );
    }
}
