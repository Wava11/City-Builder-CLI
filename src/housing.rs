use bevy::prelude::*;

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_housing);
    }
}

pub fn spawn_housing(mut commands: Commands) {
    for _ in 0..100 {
        commands.spawn(HousingBundle {
            housing_type: HousingType::Apartment,
            housing_vacancy: HousingVacancy::Vacant,
            housing_capacity: HousingCapacity(5),
        });
    }
}

#[derive(Bundle)]
pub struct HousingBundle {
    housing_vacancy: HousingVacancy,
    housing_type: HousingType,
    housing_capacity: HousingCapacity,
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

#[derive(Component, PartialEq, Eq)]
pub struct HousingCapacity(u32);

pub type Housing<'a> = (
    &'a HousingType,
    Mut<'a, HousingVacancy>,
    &'a HousingCapacity,
);

pub fn get_housing_type<'a>(housing: &Housing<'a>) -> &'a HousingType {
    housing.0
}

pub fn get_housing_vacancy<'a>(housing: &'a mut Housing<'a>) -> &mut HousingVacancy {
    &mut housing.1
}

pub fn get_housing_capacity<'a>(housing: &Housing<'a>) -> u32 {
    housing.2 .0
}
