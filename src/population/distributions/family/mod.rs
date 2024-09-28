use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};

use crate::{population::{Age, CitizenBundle}, statistics::{Sample, SampleBasedOn}};

pub mod world;

struct ParentsDistribution {
    rng: ThreadRng,
    min_age: f32,
    max_age: f32,
    age_distribution: Normal<f32>,
}
impl Sample<CitizenBundle> for ParentsDistribution {
    fn sample(&mut self, amount: u64) -> Vec<CitizenBundle> {
        (0..amount)
            .map(|_| {
                let age = self
                    .age_distribution
                    .sample(&mut self.rng)
                    .clamp(self.min_age, self.max_age);
                CitizenBundle { age: Age(age) }
            })
            .collect()
    }
}

struct ParentParterDistribution {
    rng: ThreadRng,
    min_age: f32,
    max_age: f32,
    age_factor_distribution: Normal<f32>,
}
impl SampleBasedOn<CitizenBundle> for ParentParterDistribution {
    fn sample_based_on(&mut self, t: &CitizenBundle) -> CitizenBundle {
        let age_factor = self.age_factor_distribution.sample(&mut self.rng);
        let unclamped_age = age_factor * t.age.0;
        CitizenBundle {
            age: Age(unclamped_age.clamp(self.min_age, self.max_age)),
        }
    }
}