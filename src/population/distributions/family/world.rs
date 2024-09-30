use rand_distr::Normal;

use super::super::couple::{CoupleMembersDistribution, CoupleParterDistribution};
use super::ChildrenDistribution;
use crate::population::groups::{couple::CoupleDistribution, family::FamilyDistribution};

pub fn create_world_family_distribution() -> Box<FamilyDistribution> {
    Box::new(FamilyDistribution::new(
        vec![10, 20, 20, 30, 10, 5, 1, 0],
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
        )),
        Box::new(ChildrenDistribution {
            rng: rand::thread_rng(),
            child_age_distance_from_parent_distribution: Normal::new(28., 5.).unwrap(),
        }),
    ))
}
