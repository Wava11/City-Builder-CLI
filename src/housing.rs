use bevy::prelude::*;

use crate::foo;

pub struct HousingPlugin;

impl Plugin for HousingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_housing);
    }
}

pub fn spawn_housing(mut commands: Commands) {
    for _ in 0..100 {
        commands.spawn(HousingBundle {
            housingType: HousingType::Apartment,
            housingVacancy: HousingVacancy::Vacant,
            housingCapacity: HousingCapacity(5)
        });
    }
}

#[derive(Bundle)]
pub struct HousingBundle {
    housingVacancy: HousingVacancy,
    housingType: HousingType,
    housingCapacity: HousingCapacity,
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
