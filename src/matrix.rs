use std::ops::{Div, Mul};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrixf<const R: usize, const C: usize> {
    pub data: [[f32; C]; R],
}

pub type Matrix4f = Matrixf<4, 4>;
pub type Vector3f = Matrixf<3, 1>;
pub type Vector4f = Matrixf<4, 1>;

impl Vector3f {
    pub fn xyz(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f::new([[x], [y], [z]])
    }
    pub fn cross(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f::xyz(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
    pub fn x(&self) -> f32 {
        self.data[0][0]
    }
    pub fn y(&self) -> f32 {
        self.data[1][0]
    }
    pub fn z(&self) -> f32 {
        self.data[2][0]
    }
    pub fn homogeneous(&self) -> Vector4f {
        let mut result = Matrixf::zeros();
        for i in 0..3 {
            *result.at_mut(i, 0) = *self.at(i, 0);
        }
        result.data[3][0] = 1.0;
        result
    }
}

impl Vector4f {
    pub fn hnormalized(&self) -> Vector3f {
        let mut result = Matrixf::zeros();
        for i in 0..3 {
            result.data[i][0] = self.at(i, 0) / self.at(3, 0);
        }
        result
    }
}

impl<const R: usize> Matrixf<R, 1> {
    pub fn dot(&self, rhs: &Matrixf<R, 1>) -> f32 {
        let mut result = 0.0;
        for i in 0..R {
            result += self.at(i, 0) * rhs.data[i][0];
        }
        result
    }
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn normalized(&self) -> Matrixf<R, 1> {
        self / self.length()
    }
}

impl<const R: usize, const C: usize> Matrixf<R, C> {
    pub fn new(data: [[f32; C]; R]) -> Self {
        Self { data }
    }
    pub fn zeros() -> Self {
        Self::new([[0.0; C]; R])
    }
    pub fn transpose(&self) -> Matrixf<C, R> {
        let mut result = Matrixf::<C, R>::zeros();
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }
    pub fn at(&self, row: usize, col: usize) -> &f32 {
        &self.data[row][col]
    }
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut f32 {
        &mut self.data[row][col]
    }
}

impl<const S: usize> Matrixf<S, S> {
    pub fn identity() -> Self {
        let mut result = Self::zeros();
        for i in 0..S {
            result.data[i][i] = 1.0;
        }
        result
    }
}

impl<const R1: usize, const C1: usize, const C2: usize> Mul<&Matrixf<C1, C2>> for &Matrixf<R1, C1> {
    type Output = Matrixf<R1, C2>;

    fn mul(self, rhs: &Matrixf<C1, C2>) -> Matrixf<R1, C2> {
        let mut result = Matrixf::<R1, C2>::zeros();
        for i in 0..R1 {
            for j in 0..C2 {
                for k in 0..C1 {
                    result.data[i][j] += self.at(i, k) * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl<const R: usize, const C: usize> Mul<f32> for &Matrixf<R, C> {
    type Output = Matrixf<R, C>;

    fn mul(self, rhs: f32) -> Matrixf<R, C> {
        let mut result = *self;
        result.data.iter_mut().flatten().for_each(|x| *x *= rhs);
        result
    }
}

impl<const R: usize, const C: usize> Div<f32> for &Matrixf<R, C> {
    type Output = Matrixf<R, C>;

    fn div(self, rhs: f32) -> Matrixf<R, C> {
        let mut result = *self;
        result.data.iter_mut().flatten().for_each(|x| *x /= rhs);
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multiply_by_scalar() {
        let m = Matrix4f::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let expected = Matrix4f::new([
            [2.0, 4.0, 6.0, 8.0],
            [10.0, 12.0, 14.0, 16.0],
            [18.0, 20.0, 22.0, 24.0],
            [26.0, 28.0, 30.0, 32.0],
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
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let expected = Matrix4f::new([
            [1.0, 5.0, 9.0, 13.0],
            [2.0, 6.0, 10.0, 14.0],
            [3.0, 7.0, 11.0, 15.0],
            [4.0, 8.0, 12.0, 16.0],
        ]);
        assert_eq!(m.transpose(), expected);
    }

    #[test]
    fn test_multiply_two_matrices() {
        let a = Matrix4f::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let b = Matrix4f::new([
            [17.0, 18.0, 19.0, 20.0],
            [21.0, 22.0, 23.0, 24.0],
            [25.0, 26.0, 27.0, 28.0],
            [29.0, 30.0, 31.0, 32.0],
        ]);
        let expected = Matrix4f::new([
            [250.0, 260.0, 270.0, 280.0],
            [618.0, 644.0, 670.0, 696.0],
            [986.0, 1028.0, 1070.0, 1112.0],
            [1354.0, 1412.0, 1470.0, 1528.0],
        ]);
        assert_eq!(a.mul(&b), expected);
    }

    #[test]
    fn test_identity() {
        let m = Matrix4f::identity();
        let expected = Matrix4f::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert_eq!(m, expected);
    }

    #[test]
    fn test_dot() {
        let a = Vector3f::xyz(1.0, 2.0, 3.0);
        let b = Vector3f::xyz(4.0, 5.0, 6.0);
        assert_eq!(a.dot(&b), 32.0);
    }
    #[test]
    fn test_normalized() {
        let a = Vector3f::xyz(1.0, 2.0, 3.0);
        let expected = Vector3f::xyz(0.26726124, 0.5345225, 0.8017837);
        assert_eq!(a.normalized(), expected);
    }
    #[test]
    fn test_multiply_with_vector() {
        let m = Matrix4f::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let v = Vector3f::xyz(1.0, 2.0, 3.0);
        let w = 102.0;
        let expected = Vector3f::xyz(18.0 / w, 46.0 / w, 74.0 / w);
        assert_eq!(m.mul(&v.homogeneous()).hnormalized(), expected);
    }
}
