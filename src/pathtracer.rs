use matrix::Vector3f;

struct Camera {
    origin: Vector3f,
    forward: Vector3f,
    up: Vector3f,
    right: Vector3f,
    focal_length: f32,
}

pub struct PathTracerShader {
    camera: Camera,
}

impl Camera {
    fn new(origin: Vector3f, forward: Vector3f, up: Vector3f, focal_length: f32) -> Self {
        let forward = forward.normalized();
        let right = forward.cross(&up).normalized();
        let up = right.cross(&forward).normalized();
        Self {
            origin,
            forward,
            up,
            right,
            focal_length,
        }
    }
    fn ray(&self, x: f32, y: f32) -> Vector3f {
        self.forward * self.focal_length + self.right * x + self.up * y
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_ray() {
        let camera = Camera::new(
            Vector3f::xyz(0.0, 0.0, 0.0),
            Vector3f::xyz(0.0, 0.0, 1.0),
            Vector3f::xyz(0.0, 1.0, 0.0),
            1.0,
        );
        let ray = camera.ray(0.0, 0.0);
        assert_eq!(ray.x(), 0.0);
        assert_eq!(ray.y(), 0.0);
        assert_eq!(ray.z(), 1.0);
    }
}
