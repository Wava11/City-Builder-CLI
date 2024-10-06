use bevy::prelude::*;
use structure::StructurePlugin;
use terrain::TerrainType;

use crate::geometry::Point;

pub mod structure;
pub mod terrain;
pub mod zone;
pub mod pathway;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StructurePlugin)
            .add_systems(Startup, (spawn_map, spawn_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera {
        top_left: Point { x: 0, y: 0 },
    });
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

const WORLD_HEIGHT: usize = 1000;
const WORLD_WIDTH: usize = 1000;

#[derive(Clone, Copy)]
pub struct MapTile {
    pub terrain: TerrainType,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct WorldMap {
    pub map: Vec<Vec<MapTile>>,
}

//for now only set the top left and what the camera can view will be detemined by the size of the
//screen
#[derive(Component)]
pub struct Camera {
    pub top_left: Point,
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
