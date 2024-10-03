use bevy::prelude::*;
use terrain::TerrainType;

mod terrain;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera {
        top_left: Point { x: 0, y: 0 },
    });
}

fn spawn_map(mut commands: Commands) {
    let initial_map = WorldMap {
        map: [[MapTile {
            terrain: '.',
            entity: None,
        }; WORLD_WIDTH]; WORLD_HEIGHT],
    };
    commands.spawn(initial_map);
}

const WORLD_HEIGHT: usize = 1000;
const WORLD_WIDTH: usize = 1000;

#[derive(Clone, Copy)]
struct MapTile {
    terrain: char,
    entity: Option<Entity>,
}

#[derive(Component)]
struct WorldMap {
    map: [[MapTile; WORLD_WIDTH]; WORLD_HEIGHT],
}

struct Point {
    x: usize,
    y: usize,
}

//for now only set the top left and what the camera can view will be detemined by the size of the
//screen
#[derive(Component)]
struct Camera {
    top_left: Point,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
    terrain: TerrainType,
}

impl ObjectOnMap for Rectangle {
    fn place_on_map(&self, self_entity: Entity, map: &mut WorldMap) {
        for y in self.top_left.y..self.bottom_right.y {
            for x in self.top_left.x..self.bottom_right.x {
                map.map[x][y] = MapTile {
                    terrain: self.terrain.to_chr(),
                    entity: Some(self_entity),
                }
            }
        }
    }
}

trait ObjectOnMap {
    fn place_on_map(&self, self_entity: Entity, map: &mut WorldMap);
}
