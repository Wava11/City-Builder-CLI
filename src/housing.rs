use bevy::prelude::*;

// pub struct HousingPlugin;

// impl Plugin for HousingPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, spawn_housing)
//             .insert_resource(PopulationCounterTimer(Timer::from_seconds(
//                 1.,
//                 TimerMode::Repeating,
//             )))
//             .add_systems(Update, show_city_population_counter);
//     }
// }

#[derive(Bundle)]
pub struct HousingBundle {
    housingVacancy: HousingVacancy,
    housingType: HousingType,
}

#[derive(Component, PartialEq, Eq)]
pub enum HousingVacancy {
    Vacant,
    Occupied,
}

#[derive(Component, PartialEq, Eq)]
pub enum HousingType {
    Apartment,
    SingleFamilyHome,
}

