use bevy::prelude::*;

use crate::{
    housing::{HousingCapacity, HousingType, HousingVacancy},
    population::{distributions, move_new_population_in, Population},
    tick,
};

#[derive(Component)]
pub struct City;

pub struct CityPlugin;

impl Plugin for CityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_city)
            .insert_resource(MovingInPopulationTimer(Timer::from_seconds(
                1.,
                TimerMode::Repeating,
            )))
            .add_systems(Update, move_population_into_city);
    }
}

pub fn spawn_city(mut commands: Commands) {
    commands.spawn((City, Population(0)));
}

fn move_population_into_city(
    mut timer: ResMut<MovingInPopulationTimer>,
    time: Res<Time>,
    mut population_query: Query<&mut Population, With<City>>,
    mut housing_query: Query<(&HousingType, &mut HousingVacancy, &HousingCapacity)>,
) {
    tick!(timer, time);

    let new_citizen_bundle = move_new_population_in(
        housing_query.iter_mut(),
        &mut distributions::create_world_population(),
    );

    let amount_of_new_population: u64 = new_citizen_bundle.len() as u64;

    let mut population = population_query.single_mut();
    population.0 += amount_of_new_population;
}

#[derive(Resource)]
pub struct MovingInPopulationTimer(Timer);
