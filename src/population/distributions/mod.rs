use super::{groups::{self, couple::CoupleDistribution}, PopulationDistribution};

static mut WORLD_POPULATION: PopulationDistribution = PopulationDistribution {
    distributions: vec![
        CoupleDistribution::new(
            Box::new(
                
            )
        )
    ],
};
