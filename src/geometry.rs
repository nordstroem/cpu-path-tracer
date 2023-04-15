use matrix::{Vector2f, Vector2i, Vector3f};

pub struct Camera {
    forward: Vector3f,
    up: Vector3f,
    right: Vector3f,
    focal_length: f32,
    principal_point: Vector2f,
}

pub struct Ray {
    pub origin: Vector3f,
    pub direction: Vector3f,
}

pub struct Sphere {
    pub center: Vector3f,
    pub radius: f32,
}

impl Camera {
    pub fn new(forward: Vector3f, up: Vector3f, fov_rad: f32, sensor_size_px: Vector2i) -> Self {
        let forward = forward.normalized();
        let right = forward.cross(&up).normalized();
        let up = right.cross(&forward).normalized();
        let sensor_size = sensor_size_px.x().max(sensor_size_px.y()) as f32;
        let focal_length = (0.5 * sensor_size / (fov_rad * 0.5).tan()).abs();
        let principal_point = Vector2f::xy(
            sensor_size_px.x() as f32 / 2.0 + 0.5,
            sensor_size_px.y() as f32 / 2.0 + 0.5,
        );
        Self {
            forward,
            up,
            right,
            focal_length,
            principal_point,
        }
    }

    pub fn ray(&self, x: f32, y: f32) -> Ray {
        let x = x - self.principal_point.x();
        let y = self.principal_point.y() - y;
        Ray {
            origin: Vector3f::zeros(),
            direction: (self.forward * self.focal_length + self.right * x + self.up * y)
                .normalized(),
        }
    }
}

impl Ray {
    pub fn new(origin: Vector3f, direction: Vector3f) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vector3f {
        self.origin + self.direction * t
    }
}

impl Sphere {
    pub fn new(center: Vector3f, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        if t1 > 0.0 {
            Some(t1)
        } else if t2 > 0.0 {
            Some(t2)
        } else {
            None
        }
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

        let ray = camera.ray(128.5, 128.5);
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

        let ray = camera.ray(0.5, 128.5);
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
        let camera = Camera::new(forward, up, 90_f32.to_radians(), Vector2i::xy(256, 256));

        let ray = camera.ray(128.5, 0.5);
        let tol = 1e-6;
        assert!(ray.direction.y() > 0.0);
        assert_approx!(
            ray.direction.cos_angle(&forward),
            45_f32.to_radians().cos(),
            tol
        );
    }
}
