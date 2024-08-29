use bevy::prelude::*;

use crate::{
    housing::{HousingType, HousingVacancy},
    population::Population,
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
            .add_systems(Update, show_city_population_counter);
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
    population_query: Query<&mut Population, With<City>>,
    housing_query: Query<(&HousingType, &HousingVacancy)>,
) {
    tick!(timer, time);
    let vacant_housing_amount = housing_query
        .iter()
        .filter(|(_, housing_vacancy)| **housing_vacancy == HousingVacancy::Vacant)
        .collect::<Vec<_>>()
        .len();
    
    
    let new_population = vacant_housing_amount as f32 / 20.;

}

#[derive(Resource)]
pub struct PopulationCounterTimer(Timer);

#[derive(Resource)]
pub struct MovingInPopulationTimer(Timer);
