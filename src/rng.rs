use matrix::Vector3f;

pub struct Rng {
    pub seed: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
    pub fn unit_sphere(&mut self) -> Vector3f {
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

    pub fn uniform(&mut self) -> f32 {
        let mut x = self.seed + 42;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.seed = x;
        self.seed as f32 / std::u32::MAX as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_sphere() {
        let mut rng = Rng::new(0);
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
        let mut rng = Rng::new(0);
        let mut numbers = Vec::new();
        for _ in 0..100 {
            numbers.push(rng.uniform());
        }
        let mut rng = Rng::new(0);
        for number in numbers {
            assert_eq!(number, rng.uniform());
        }
    }
}
