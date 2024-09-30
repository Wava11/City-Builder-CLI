use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Normal};

use crate::{
    population::{groups::couple::Couple, Age, CitizenBundle},
    statistics::SampleBasedOn,
};

pub mod world;

struct ChildrenDistribution {
    rng: ThreadRng,
    child_age_distance_from_parent_distribution: Normal<f32>,
}
impl SampleBasedOn<Couple, CitizenBundle> for ChildrenDistribution {
    fn sample_based_on(&mut self, parents: &Couple, amount: u64) -> Vec<CitizenBundle> {
        let avg_parents_age =
            parents.members.iter().map(|x| x.age.0).sum::<f32>() / parents.members.len() as f32;

        // TODO: make children have reasonable gaps in age
        (0..amount)
            .map(|_| {
                let child_age_distance_from_parents_avg = self
                    .child_age_distance_from_parent_distribution
                    .sample(&mut self.rng);
                let child_age = avg_parents_age
                    - child_age_distance_from_parents_avg.clamp(0., avg_parents_age - 18.);
                CitizenBundle {
                    age: Age(child_age),
                }
            })
            .collect()
    }
}
