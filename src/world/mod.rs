use bevy::prelude::*;
use terrain::TerrainType;

use crate::geometry::Point;

pub mod structure;
pub mod terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, spawn_camera));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera {
        top_left: Point { x: 0, y: 0 },
    });
}

fn spawn_map(mut commands: Commands) {
    println!("Before");
    let initial_map = WorldMap {
        map: Box::new(
            [[MapTile {
                terrain: TerrainType::Ground,
                entity: None,
            }; WORLD_WIDTH]; WORLD_HEIGHT],
        ),
    };
    commands.spawn(initial_map);
}

const WORLD_HEIGHT: usize = 120;
const WORLD_WIDTH: usize = 120;

#[derive(Clone, Copy)]
pub struct MapTile {
    pub terrain: TerrainType,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub struct WorldMap {
    pub map: Box<[[MapTile; WORLD_WIDTH]; WORLD_HEIGHT]>,
}

#[derive(Component)]
struct Position(Point);

//for now only set the top left and what the camera can view will be detemined by the size of the
//screen
#[derive(Component)]
pub struct Camera {
    pub top_left: Point,
}

#[derive(Component)]
pub struct Area {
    pub width: usize,
    pub height: usize,
}

// impl ObjectOnMap for Area {
//     fn place_on_map(&self, self_entity: Entity, map: &mut WorldMap) {
//         for y in self.top_left.y..self.bottom_right.y {
//             for x in self.top_left.x..self.bottom_right.x {
//                 map.map[x][y] = MapTile {
//                     terrain: self.terrain,
//                     entity: Some(self_entity),
//                 }
//             }
//         }
//     }
// }

trait ObjectOnMap {
    fn place_on_map(&self, self_entity: Entity, map: &mut WorldMap);
}

#[derive(Component)]
pub enum Rotation {
    Right,
    Up,
    Left,
    Down,
}
