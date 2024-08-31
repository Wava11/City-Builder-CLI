use bevy::prelude::*;
use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::ThreadRng, Rng};
use rand_distr::Normal;

use crate::{
    housing::{HousingBundle, HousingCapacity, HousingType, HousingVacancy},
    population,
};

#[derive(Component)]
pub struct Population(pub u64);

pub enum AgeGroup {
    toddler,
    child,
    teen,
    young_adult,
    adult,
    elderly,
}

#[derive(Component)]
pub struct Age(u8);

#[derive(Bundle)]
pub struct CitizenBundle {
    age: Age,
}

pub fn move_new_population_in<'a>(
    housing: impl Iterator<Item = Housing<'a>>,
    population_distribution: &mut PopulationDistribution,
) -> Vec<CitizenBundle> {
    let vacant_housing = housing
        .filter(|(_, housing_vacancy, _)| **housing_vacancy == HousingVacancy::Vacant)
        .collect::<Vec<_>>();

    let current_population = population_distribution
        .sample((vacant_housing.len() as f64 * HOUSING_TO_POPULATION_FACTOR) as u64);

    if current_population.len() == 0 {
        return vec![];
    }

    let mut vacant_housing_with_best_potential_dwellers: Vec<_> = vacant_housing
        .iter()
        .filter_map(|housing| inject_most_interested_population(&current_population, housing))
        .collect();
    vacant_housing_with_best_potential_dwellers.sort_by_key(|(.., score)| *score);

    let mut rng = rand::thread_rng();
    let occupancy_factor: f32 = rng.gen::<f32>() * (0.15 - 0.05) + 0.05;
    let amount_to_occupy = (vacant_housing.len() as f32 * occupancy_factor) as usize;

    vacant_housing_with_best_potential_dwellers
        .iter_mut()
        .take(amount_to_occupy)
        .for_each(|(housing, ..)| {
            *housing.1 = HousingVacancy::Occupied;
        });
    vacant_housing_with_best_potential_dwellers
        .into_iter()
        .take(amount_to_occupy)
        .map(|(.., citizens_bundles, _)| citizens_bundles)
        .flatten()
        .collect()
}

fn inject_most_interested_population<'a>(
    current_population: &Vec<Box<dyn ScoreHousing>>,
    housing: &Housing<'a>,
) -> Option<(Housing<'a>, Vec<CitizenBundle>, u16)> {
    let (housing_type, _, housing_capacity) = housing;
    let (winning_group, winning_score) = current_population
        .iter()
        .map(|population_group| (population_group, population_group.score_housing(housing)))
        .max_by_key(|(_, score)| *score)?;

    Some((housing, winning_group, winning_score))
}

const HOUSING_TO_POPULATION_FACTOR: f64 = 4.;

type Housing<'a> = (&'a HousingType, &'a mut HousingVacancy, &'a HousingCapacity);

struct PopulationDistribution {
    distributions: Vec<(Box<dyn Sample<Box<dyn ScoreHousing>>>, f64)>,
}
impl Sample<Box<dyn ScoreHousing>> for PopulationDistribution {
    fn sample(&mut self, amount: u64) -> Vec<Box<dyn ScoreHousing>> {
        self.distributions
            .iter_mut()
            .map(|(ref mut distribution, ref factor)| {
                distribution.sample((amount as f64 * factor) as u64)
            })
            .flatten()
            .collect()
    }
}

trait ScoreHousing {
    fn score_housing<'a>(&self, housing: &Housing<'a>) -> u16;
}

trait Hi {
    fn into(&self) -> u8;
}
trait Sample<T> {
    fn sample(&mut self, amount: u64) -> Vec<T>;
}

struct Family {
    num_of_parents: u8,
    num_of_children: u8,
}
impl Into<Vec<CitizenBundle>> for Family {
    fn into(self) -> Vec<CitizenBundle> {}
}

struct FamilyDistribution {
    rng: ThreadRng,
    num_of_parents_distribution: WeightedIndex<u8>,
    num_of_children_distribution: WeightedIndex<u8>,
}
impl FamilyDistribution {
    fn new(num_of_parents_weights: Vec<u8>, num_of_children_weights: Vec<u8>) -> Self {
        Self {
            rng: rand::thread_rng(),
            num_of_parents_distribution: WeightedIndex::new(num_of_parents_weights).unwrap(),
            num_of_children_distribution: WeightedIndex::new(num_of_children_weights).unwrap(),
        }
    }
}
impl Sample<Family> for FamilyDistribution {
    fn sample(&mut self, amount: u64) -> Vec<Family> {
        (0..amount)
            .map(|_| Family {
                num_of_parents: self.num_of_parents_distribution.sample(&mut self.rng) as u8,
                num_of_children: self.num_of_children_distribution.sample(&mut self.rng) as u8,
            })
            .collect()
    }
}

struct Couple {
    age: u8,
}
struct CoupleDistribution {
    rng: ThreadRng,
    min_age: u8,
    max_age: u8,
    age_distribution: Normal<f32>,
}
impl CoupleDistribution {
    fn new(min_age: u8, max_age: u8, mean_age: f32, std_age: f32) -> Self {
        Self {
            rng: rand::thread_rng(),
            min_age,
            max_age,
            age_distribution: Normal::new(mean_age, std_age).unwrap(),
        }
    }
}
impl Sample<Couple> for CoupleDistribution {
    fn sample(&mut self, amount: u64) -> Vec<Couple> {
        (0..amount)
            .map(|_| Couple {
                age: self
                    .age_distribution
                    .sample(&mut self.rng)
                    .floor()
                    .clamp(self.min_age as f32, self.max_age as f32) as u8,
            })
            .collect()
    }
}