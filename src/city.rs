use bevy::prelude::*;
use rand::Rng;

use crate::{
    housing::{HousingType, HousingVacancy},
    population::{generate_population, Population},
    tick,
};

#[derive(Component)]
pub struct City;

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_city)
            .insert_resource(PopulationCounterTimer(Timer::from_seconds(
                1.,
                TimerMode::Repeating,
            )))
            .insert_resource(MovingInPopulationTimer(Timer::from_seconds(
                1.,
                TimerMode::Repeating,
            )))
            .add_systems(
                Update,
                (show_city_population_counter, move_population_into_city),
            );
    }
}

pub fn spawn_city(mut commands: Commands) {
    commands.spawn((City, Population(0)));
}

pub fn show_city_population_counter(
    mut timer: ResMut<PopulationCounterTimer>,
    time: Res<Time>,
    query: Query<&Population, With<City>>,
) {
    tick!(timer, time);
    let Population(population) = query.single();
    println!("{population} population");
}

fn move_population_into_city(
    mut timer: ResMut<MovingInPopulationTimer>,
    time: Res<Time>,
    mut population_query: Query<&mut Population, With<City>>,
    housing_query: Query<(&HousingType, &HousingVacancy)>,
) {
    tick!(timer, time);
    let vacant_housing_amount = generate_population(housing_query
        .iter());
        
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen::<f64>() * (25. - 10.) + 10.;

    let amount_of_new_population: u64 = (vacant_housing_amount as f64 / y).ceil() as u64;
    let mut population = population_query.single_mut();
    population.0 += amount_of_new_population;

}

#[derive(Resource)]
pub struct PopulationCounterTimer(Timer);

#[derive(Resource)]
pub struct MovingInPopulationTimer(Timer);
