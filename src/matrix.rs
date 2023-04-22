use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Numeric:
    From<u8>
    + Copy
    + Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + DivAssign
    + Div<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + PartialOrd
{
}
impl<
        T: From<u8>
            + Copy
            + Add<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Sub<Output = T>
            + AddAssign
            + MulAssign
            + DivAssign
            + SubAssign
            + PartialOrd,
    > Numeric for T
{
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<T: Numeric, const R: usize, const C: usize> {
    pub data: [[T; C]; R],
}

pub type Vector<T, const R: usize> = Matrix<T, R, 1>;
pub type Vector2i = Vector<i32, 2>;
pub type Vector2f = Vector<f32, 2>;
pub type Vector3f = Vector<f32, 3>;
pub type Vector4f = Vector<f32, 4>;

impl<T: Numeric> Vector<T, 3> {
    pub fn xyz(x: T, y: T, z: T) -> Self {
        Self::new([[x], [y], [z]])
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::xyz(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }
    pub fn x(&self) -> T {
        self.data[0][0]
    }
    pub fn y(&self) -> T {
        self.data[1][0]
    }
    pub fn z(&self) -> T {
        self.data[2][0]
    }
    pub fn homogeneous(&self) -> Vector<T, 4> {
        let mut result = Vector::zeros();
        for i in 0..3 {
            result.data[i][0] = self.data[i][0];
        }
        result.data[3][0] = T::from(1);
        result
    }
}

impl<T: Numeric> Vector<T, 2> {
    pub fn xy(x: T, y: T) -> Self {
        Self::new([[x], [y]])
    }
    pub fn x(&self) -> T {
        self.data[0][0]
    }
    pub fn y(&self) -> T {
        self.data[1][0]
    }
}

impl Vector4f {
    pub fn hnormalized(&self) -> Vector3f {
        let mut result = Matrix::zeros();
        for i in 0..3 {
            result.data[i][0] = self.data[i][0] / self.data[3][0]
        }
        result
    }
}

impl<T: Numeric, const R: usize> Matrix<T, R, 1> {
    pub fn dot(&self, rhs: &Matrix<T, R, 1>) -> T {
        let mut result = T::from(0);
        for i in 0..R {
            result += self.data[i][0] * rhs.data[i][0];
        }
        result
    }
}

fn fast_inverse_sqrt(x: f32) -> f32 {
    let xhalf = 0.5 * x;
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - xhalf * y * y)
}

impl<const R: usize> Matrix<f32, R, 1> {
    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn distance(&self, rhs: &Self) -> f32 {
        (*self - *rhs).length()
    }
    pub fn squared_distance(&self, rhs: &Self) -> f32 {
        (*self - *rhs).squared_length()
    }
    pub fn squared_length(&self) -> f32 {
        self.dot(self)
    }
    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
    pub fn fast_normalized(&self) -> Self {
        *self * fast_inverse_sqrt(self.squared_length())
    }
    pub fn cos_angle(&self, rhs: &Self) -> f32 {
        self.dot(rhs) / (self.length() * rhs.length())
    }
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        let mut result = Matrix::zeros();
        for i in 0..R {
            result.data[i][0] = self.data[i][0].max(min).min(max);
        }
        result
    }
}

impl<T: Numeric, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(data: [[T; C]; R]) -> Self {
        Self { data }
    }
    pub fn zeros() -> Self {
        Self::new([[T::from(0); C]; R])
    }
    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut result = Matrix::<T, C, R>::zeros();
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }
    pub fn at(&self, row: usize, col: usize) -> &T {
        &self.data[row][col]
    }
    pub fn at_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row][col]
    }

    pub fn cast<U: Numeric + From<T>>(&self) -> Matrix<U, R, C> {
        let mut result = Matrix::<U, R, C>::zeros();
        for i in 0..R {
            for j in 0..C {
                result.data[i][j] = U::from(self.data[i][j]);
            }
        }
        result
    }
}

impl<T: Numeric, const S: usize> Matrix<T, S, S> {
    pub fn identity() -> Self {
        let mut result = Self::zeros();
        for i in 0..S {
            result.data[i][i] = T::from(1);
        }
        result
    }
}

impl<T: Numeric, const R1: usize, const C1: usize, const C2: usize> Mul<&Matrix<T, C1, C2>>
    for &Matrix<T, R1, C1>
{
    type Output = Matrix<T, R1, C2>;

    fn mul(self, rhs: &Matrix<T, C1, C2>) -> Matrix<T, R1, C2> {
        let mut result = Matrix::<T, R1, C2>::zeros();
        for i in 0..R1 {
            for j in 0..C2 {
                for k in 0..C1 {
                    result.data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        }
        result
    }
}

impl<T: Numeric, const R: usize, const C: usize> Add<&Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(mut self, rhs: &Matrix<T, R, C>) -> Matrix<T, R, C> {
        self.data
            .iter_mut()
            .flatten()
            .zip(rhs.data.iter().flatten())
            .for_each(|(lhs, rhs)| *lhs += *rhs);
        self
    }
}

impl<T: Numeric, const R: usize, const C: usize> AddAssign<Matrix<T, R, C>> for Matrix<T, R, C> {
    fn add_assign(&mut self, rhs: Matrix<T, R, C>) {
        self.data
            .iter_mut()
            .flatten()
            .zip(rhs.data.iter().flatten())
            .for_each(|(lhs, rhs)| *lhs += *rhs);
    }
}

impl<T: Numeric, const R: usize, const C: usize> AddAssign<T> for Matrix<T, R, C> {
    fn add_assign(&mut self, rhs: T) {
        self.data.iter_mut().flatten().for_each(|lhs| *lhs += rhs);
    }
}

impl<T: Numeric, const R: usize, const C: usize> DivAssign<T> for Matrix<T, R, C> {
    fn div_assign(&mut self, rhs: T) {
        self.data.iter_mut().flatten().for_each(|lhs| *lhs /= rhs);
    }
}

impl<T: Numeric, const R: usize, const C: usize> Add<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(mut self, rhs: Matrix<T, R, C>) -> Matrix<T, R, C> {
        self.data
            .iter_mut()
            .flatten()
            .zip(rhs.data.iter().flatten())
            .for_each(|(lhs, rhs)| *lhs += *rhs);
        self
    }
}

impl<T: Numeric, const R: usize, const C: usize> Sub<Matrix<T, R, C>> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(mut self, rhs: Matrix<T, R, C>) -> Matrix<T, R, C> {
        self.data
            .iter_mut()
            .flatten()
            .zip(rhs.data.iter().flatten())
            .for_each(|(lhs, rhs)| *lhs -= *rhs);
        self
    }
}

impl<T: Numeric, const R: usize, const C: usize> Add<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn add(mut self, rhs: T) -> Matrix<T, R, C> {
        self.data.iter_mut().flatten().for_each(|x| *x += rhs);
        self
    }
}

impl<T: Numeric, const R: usize, const C: usize> Sub<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn sub(mut self, rhs: T) -> Matrix<T, R, C> {
        self.data.iter_mut().flatten().for_each(|x| *x += rhs);
        self
    }
}

impl<T: Numeric, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn mul(mut self, rhs: T) -> Matrix<T, R, C> {
        self.data.iter_mut().flatten().for_each(|x| *x *= rhs);
        self
    }
}

impl<T: Numeric, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
    type Output = Matrix<T, R, C>;

    fn div(mut self, rhs: T) -> Matrix<T, R, C> {
        self.data.iter_mut().flatten().for_each(|x| *x /= rhs);
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    pub type Matrix4f = Matrix<f32, 4, 4>;

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
        assert_eq!(m * 2.0, expected);
    }

    #[test]
    fn test_multiply_two_identity_matrices() {
        let a = Matrix4f::identity();
        let b = Matrix4f::identity();
        assert_eq!(&a * &b, Matrix4f::identity());
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
        assert_eq!(&a * &b, expected);
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
    fn test_multiply4x4_with_vector3() {
        let m = Matrix4f::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let v = Vector3f::xyz(1.0, 2.0, 3.0);
        let w = 102.0;
        let expected = Vector3f::xyz(18.0 / w, 46.0 / w, 74.0 / w);
        let result = (&m * &v.homogeneous()).hnormalized();
        let tol = 1e-6;
        assert_approx!(result.x(), expected.x(), tol);
        assert_approx!(result.y(), expected.y(), tol);
        assert_approx!(result.z(), expected.z(), tol);
    }

    #[test]
    fn test_add_matrices() {
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
            [18.0, 20.0, 22.0, 24.0],
            [26.0, 28.0, 30.0, 32.0],
            [34.0, 36.0, 38.0, 40.0],
            [42.0, 44.0, 46.0, 48.0],
        ]);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_add_two_vectors() {
        let a = Vector3f::xyz(1.0, 2.0, 3.0);
        let b = Vector3f::xyz(4.0, 5.0, 6.0);
        let expected = Vector3f::xyz(5.0, 7.0, 9.0);
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_subtract_two_vectors() {
        let a = Vector3f::xyz(1.0, 2.0, 3.0);
        let b = Vector3f::xyz(4.0, 5.0, 6.0);
        let expected = Vector3f::xyz(-3.0, -3.0, -3.0);
        assert_eq!(a - b, expected);
    }
}
