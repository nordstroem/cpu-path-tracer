use std::ops::Mul;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix4f {
    pub data: [f32; 16],
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Camera {
    pub position: Vector3f,
    pub forward: Vector3f,
    pub up: Vector3f,
    pub right: Vector3f,
    pub fov: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f { x, y, z }
    }
    pub fn zeros() -> Vector3f {
        Vector3f::new(0.0, 0.0, 0.0)
    }
    pub fn dot(&self, rhs: &Vector3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
    pub fn normalized(&self) -> Vector3f {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len == 0.0 {
            return Vector3f::zeros();
        }
        Vector3f::new(self.x / len, self.y / len, self.z / len)
    }
}

impl Matrix4f {
    pub fn new(data: [f32; 16]) -> Matrix4f {
        Matrix4f { data }
    }
    pub fn zeros() -> Matrix4f {
        Matrix4f::new([0.0; 16])
    }
    pub fn identity() -> Matrix4f {
        Matrix4f::new([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }
    pub fn transpose(&self) -> Matrix4f {
        let mut result = Matrix4f::zeros();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i + j * 4] = self.data[j + i * 4];
            }
        }
        result
    }
}

impl Mul<&Matrix4f> for &Matrix4f {
    type Output = Matrix4f;

    fn mul(self, rhs: &Matrix4f) -> Matrix4f {
        let mut result = Matrix4f::zeros();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i + j * 4] += self.data[k + j * 4] * rhs.data[i + k * 4];
                }
            }
        }
        result
    }
}

impl Mul<f32> for &Matrix4f {
    type Output = Matrix4f;

    fn mul(self, rhs: f32) -> Matrix4f {
        let mut result = Matrix4f::zeros();
        for i in 0..16 {
            result.data[i] = self.data[i] * rhs;
        }
        result
    }
}

impl Mul<&Vector3f> for &Matrix4f {
    type Output = Vector3f;

    fn mul(self, rhs: &Vector3f) -> Vector3f {
        let result = Vector3f::new(
            self.data[0] * rhs.x + self.data[1] * rhs.y + self.data[2] * rhs.z + self.data[3],
            self.data[4] * rhs.x + self.data[5] * rhs.y + self.data[6] * rhs.z + self.data[7],
            self.data[8] * rhs.x + self.data[9] * rhs.y + self.data[10] * rhs.z + self.data[11],
        );
        let w =
            self.data[12] * rhs.x + self.data[13] * rhs.y + self.data[14] * rhs.z + self.data[15];
        Vector3f {
            x: result.x / w,
            y: result.y / w,
            z: result.z / w,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply_by_scalar() {
        let m = Matrix4f::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        let expected = Matrix4f::new([
            2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0, 26.0, 28.0, 30.0,
            32.0,
        ]);
        assert_eq!(m.mul(2.0), expected);
    }

    #[test]
    fn test_multiply_two_identity_matrices() {
        let a = Matrix4f::identity();
        let b = Matrix4f::identity();
        assert_eq!(a.mul(&b), Matrix4f::identity());
    }

    #[test]
    fn test_transpose() {
        let m = Matrix4f::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        let expected = Matrix4f::new([
            1.0, 5.0, 9.0, 13.0, 2.0, 6.0, 10.0, 14.0, 3.0, 7.0, 11.0, 15.0, 4.0, 8.0, 12.0, 16.0,
        ]);
        assert_eq!(m.transpose(), expected);
    }

    #[test]
    fn test_multiply_two_matrices() {
        let a = Matrix4f::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        let b = Matrix4f::new([
            17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0,
            31.0, 32.0,
        ]);
        let expected = Matrix4f::new([
            250.0, 260.0, 270.0, 280.0, 618.0, 644.0, 670.0, 696.0, 986.0, 1028.0, 1070.0, 1112.0,
            1354.0, 1412.0, 1470.0, 1528.0,
        ]);
        assert_eq!(a.mul(&b), expected);
    }

    #[test]
    fn test_multiply_with_vector() {
        let m = Matrix4f::new([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        let v = Vector3f::new(1.0, 2.0, 3.0);
        let w = 102.0;
        let expected = Vector3f::new(18.0 / w, 46.0 / w, 74.0 / w);
        assert_eq!(m.mul(&v), expected);
    }
}
