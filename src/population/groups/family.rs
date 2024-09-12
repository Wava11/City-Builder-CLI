use rand::rngs::ThreadRng;
use rand::distributions::{WeightedIndex, Distribution};

use crate::housing::{get_housing_capacity, get_housing_type, HousingType};
use crate::population::ScoreHousing;
use crate::{population::{CitizenBundle, ToCitizensBundles}, statistics::Sample};

struct Family {
    parents: Vec<CitizenBundle>,
    children: Vec<CitizenBundle>,
}
impl ToCitizensBundles for Family {
    fn to_citizens_bundles(&self) -> Vec<CitizenBundle> {
        [self.parents.clone(), self.children.clone()].concat()
    }
}

impl ScoreHousing for Family {
    fn score_housing<'a>(&self, housing: &crate::housing::Housing<'a>) -> u16 {
        if (get_housing_capacity(housing) as usize) < self.parents.len() + self.children.len() {
            return 0;
        }
        match get_housing_type(housing) {
            HousingType::SingleFamilyHome => u16::MAX,
            HousingType::Apartment => u16::MAX/2,
        }
    }
}

pub struct FamilyDistribution {
    rng: ThreadRng,
    num_of_parents_distribution: WeightedIndex<u8>,
    num_of_children_distribution: WeightedIndex<u8>,
    parents_distribution: Box<dyn Sample<CitizenBundle>>,
    children_distribution: Box<dyn Sample<CitizenBundle>>,
}
impl FamilyDistribution {
    // TODO: make parents similar to each other and also children
    fn new(
        num_of_parents_weights: Vec<u8>,
        num_of_children_weights: Vec<u8>,
        parents_distribution: Box<dyn Sample<CitizenBundle>>,
        children_distribution: Box<dyn Sample<CitizenBundle>>,
    ) -> Self {
        Self {
            rng: rand::thread_rng(),
            num_of_parents_distribution: WeightedIndex::new(num_of_parents_weights).unwrap(),
            num_of_children_distribution: WeightedIndex::new(num_of_children_weights).unwrap(),
            parents_distribution,
            children_distribution,
        }
    }
}
impl Sample<Family> for FamilyDistribution {
    fn sample(&mut self, amount: u64) -> Vec<Family> {
        (0..amount)
            .map(|_| {
                let num_of_parents = self.num_of_parents_distribution.sample(&mut self.rng) as u64;
                let num_of_children =
                    self.num_of_children_distribution.sample(&mut self.rng) as u64;
                Family {
                    parents: self.parents_distribution.sample(num_of_parents),
                    children: self.children_distribution.sample(num_of_children),
                }
            })
            .collect()
    }
}