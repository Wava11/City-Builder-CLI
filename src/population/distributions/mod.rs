use super::PopulationDistribution;

mod couple;
mod family;
use couple::world::{
    create_world_elderly_couple_distribution, create_world_young_couple_distribution,
};
use family::world::create_world_family_distribution;

pub fn create_world_population() -> PopulationDistribution {
    PopulationDistribution {
        distributions: vec![
            (create_world_young_couple_distribution(), 0.2),
            (create_world_elderly_couple_distribution(), 0.2),
            (create_world_family_distribution(), 0.6)
        ],
    }
}
