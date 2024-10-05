use bevy::prelude::*;

use crate::map::Position;

use super::{Area, Rotation};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StructurePlacedEvent>()
            .add_systems(Update, place_structures);
    }
}

#[derive(Component, Clone)]
pub enum Structure {
    Building(Building),
    Pathway(Pathway),
}

#[derive(Bundle, Clone)]
pub struct StructureBundle {
    pub structure: Structure,
    pub position: Position,
    pub area: Area,
    pub rotation: Rotation,
}

#[derive(Clone)]
pub enum Building {
    Residential,
    Commerce,
    Office,
    Industry,
}

#[derive(Clone)]
pub enum Pathway {
    Road,
}

#[derive(Event)]
pub struct StructurePlacedEvent(pub StructureBundle);

fn place_structures(
    mut structure_placed_events: EventReader<StructurePlacedEvent>,
    mut commands: Commands,
) {
    for StructurePlacedEvent(structure_bundle) in structure_placed_events.read() {
        commands.spawn(structure_bundle.clone());
    }
}
