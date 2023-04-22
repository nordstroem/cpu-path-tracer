use matrix::Vector3f;

pub struct Rng {
    pub seed: f32,
}

impl Rng {
    pub fn new(seed: f32) -> Self {
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
