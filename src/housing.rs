use bevy::prelude::*;

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_housing);
    }
}

pub fn spawn_housing(mut commands: Commands) {
    for _ in 0..100  {
        commands.spawn(
            HousingBundle {
                housingType: HousingType::Apartment,
                housingVacancy: HousingVacancy::Vacant,
            },
        );
    }
}

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
