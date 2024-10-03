use bevy::prelude::*;

#[derive(Component)]
pub enum Structure {
    Building(Building),
    Pathway(Pathway),
}

pub enum Building {
    Residential,
    Commerce,
    Office,
    Industry,
}

pub enum Pathway {
    Road,
}
