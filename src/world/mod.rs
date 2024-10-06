use bevy::prelude::*;
use camera::CameraPlugin;
use structure::StructurePlugin;
use terrain::TerrainType;

pub mod camera;
pub mod structure;
pub mod terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(( StructurePlugin, CameraPlugin ))
            .add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands) {
    let initial_map = WorldMap {
        map: vec![
            vec![
                MapTile {
                    terrain: TerrainType::Ground,
                    entity: None,
                };
                WORLD_WIDTH
            ];
            WORLD_HEIGHT
        ],
    };
    commands.spawn(initial_map);
}

pub const WORLD_HEIGHT: usize = 1000;
pub const WORLD_WIDTH: usize = 1000;

#[derive(Clone, Copy)]
pub struct MapTile {
    pub terrain: TerrainType,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct WorldMap {
    pub map: Vec<Vec<MapTile>>,
}

#[derive(Component, Clone)]
pub struct Area {
    pub width: usize,
    pub height: usize,
}

impl Area {
    pub fn new(width: usize, height: usize) -> Area {
        Area { width, height }
    }
}

#[derive(Component, Clone)]
pub enum Rotation {
    Right,
    Up,
    Left,
    Down,
}

impl Rotation {
    pub fn next_clockwise(&self) -> Rotation {
        match self {
            Rotation::Right => Rotation::Down,
            Rotation::Down => Rotation::Left,
            Rotation::Left => Rotation::Up,
            Rotation::Up => Rotation::Right,
        }
    }
}
