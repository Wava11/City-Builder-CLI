use rand_distr::Normal;

use crate::population::groups::couple::CoupleDistribution;

use super::{CoupleMembersDistribution, CoupleParterDistribution};

pub fn create_world_young_couple_distribution() -> Box<CoupleDistribution> {
    Box::new(CoupleDistribution::new(
        Box::new(CoupleMembersDistribution {
            rng: rand::thread_rng(),
            min_age: 18.,
            max_age: 120.,
            age_distribution: Normal::new(28., 5.).unwrap(),
        }),
        Box::new(CoupleParterDistribution {
            rng: rand::thread_rng(),
            min_age: 18.,
            max_age: 120.,
            age_factor_distribution: Normal::new(1., 0.1).unwrap(),
        }),
    ))
}
pub fn create_world_elderly_couple_distribution() -> Box<CoupleDistribution> {
    Box::new(CoupleDistribution::new(
        Box::new(CoupleMembersDistribution {
            rng: rand::thread_rng(),
            min_age: 45.,
            max_age: 120.,
            age_distribution: Normal::new(65., 10.).unwrap(),
        }),
        Box::new(CoupleParterDistribution {
            rng: rand::thread_rng(),
            min_age: 40.,
            max_age: 120.,
            age_factor_distribution: Normal::new(1., 0.05).unwrap(),
        }),
    ))
}
