use matrix::{Vector2f, Vector2i, Vector3f};

struct Camera {
    forward: Vector3f,
    up: Vector3f,
    right: Vector3f,
    focal_length: f32,
    principal_point: Vector2f,
}

pub struct PathTracerShader {
    camera: Camera,
}

impl Camera {
    fn new(forward: Vector3f, up: Vector3f, fov_rad: f32, sensor_size_px: Vector2i) -> Self {
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
    fn ray(&self, x: u32, y: u32) -> Vector3f {
        let x = x as f32 - self.principal_point.x();
        let y = y as f32 - self.principal_point.y();
        (self.forward * self.focal_length + self.right * x + self.up * y).normalized()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_ray() {
        let camera = Camera::new(
            Vector3f::xyz(0.0, 0.0, -1.0),
            Vector3f::xyz(0.0, 1.0, 0.0),
            90_f32.to_radians(),
            Vector2i::xy(256, 256),
        );

        let ray = camera.ray(128, 128);
        assert_eq!(ray.x(), 0.0);
        assert_eq!(ray.y(), 0.0);
        assert_eq!(ray.z(), 1.0);
    }
}
