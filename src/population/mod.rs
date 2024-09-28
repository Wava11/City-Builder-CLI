use bevy::prelude::*;
use rand::Rng;

pub mod distributions;
mod groups;

use crate::{
    housing::{Housing, HousingVacancy},
    statistics::Sample,
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

#[derive(Component, Clone)]
pub struct Age(f32);
impl Age {
    fn to_age_group(&self) -> AgeGroup {
        match self.0 {
            ..=3. => AgeGroup::toddler,
            4.0..=13. => AgeGroup::child,
            14.0..=18. => AgeGroup::teen,
            19.0..=25. => AgeGroup::young_adult,
            26.0..=60. => AgeGroup::adult,
            _ => AgeGroup::elderly
        }
    }
}

#[derive(Bundle, Clone)]
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

    let amount_of_potential_new_population =
        (vacant_housing.len() as f64 * HOUSING_TO_POPULATION_FACTOR) as u64;
    let current_population = population_distribution.sample(amount_of_potential_new_population);

    if current_population.len() == 0 {
        return vec![];
    }

    let mut rng = rand::thread_rng();
    let occupancy_factor: f32 = rng.gen::<f32>() * (0.15 - 0.05) + 0.05;
    let amount_to_occupy = (vacant_housing.len() as f32 * occupancy_factor) as usize;

    let mut vacant_housing_with_best_potential_dwellers: Vec<_> = vacant_housing
        .into_iter()
        .filter_map(|housing| inject_most_interested_population(&current_population, housing))
        .collect();
    vacant_housing_with_best_potential_dwellers.sort_by_key(|(.., score)| *score);

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

trait ScoreHousingAndToCitizensBundles: ScoreHousing + ToCitizensBundles {}

fn inject_most_interested_population<'a>(
    current_population: &Vec<Box<dyn ScoreHousingAndToCitizensBundles>>,
    housing: Housing<'a>,
) -> Option<(Housing<'a>, Vec<CitizenBundle>, u16)> {
    let (winning_group, winning_score) = current_population
        .iter()
        .map(|population_group| (population_group, population_group.score_housing(&housing)))
        .max_by_key(|(_, score)| *score)?;

    Some((housing, winning_group.to_citizens_bundles(), winning_score))
}

const HOUSING_TO_POPULATION_FACTOR: f64 = 4.;


pub struct PopulationDistribution {
    distributions: Vec<(
        Box<dyn Sample<Box<dyn ScoreHousingAndToCitizensBundles>>>,
        f64,
    )>,
}
impl Sample<Box<dyn ScoreHousingAndToCitizensBundles>> for PopulationDistribution {
    fn sample(&mut self, amount: u64) -> Vec<Box<dyn ScoreHousingAndToCitizensBundles>> {
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

trait ToCitizensBundles {
    fn to_citizens_bundles(&self) -> Vec<CitizenBundle>;
}
