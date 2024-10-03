use crate::{
    geometry::{rectangle_contains_point, rectangles_intersect, Point},
    world::Camera,
};
use bevy::prelude::*;
use crossterm::terminal;

use crate::{
    map::Position,
    world::{
        structure::{Building, Pathway, Structure},
        Area, Rotation, WorldMap,
    },
};

use super::Terminal;

pub fn create_map_view_sprite(
    terminal: Res<Terminal>,
    map_query: Query<&WorldMap>,
    entities_query: Query<(&Structure, &Area, &Rotation, &Position)>,
    camera_query: Query<&Camera>,
) {
    let camera = camera_query.single();
    let terminal_size = terminal.0.size().unwrap();

    let camera_bottom_right = Point {
        x: camera.top_left.x + terminal_size.width as usize,
        y: camera.top_left.y + terminal_size.height as usize,
    };

    let map = map_query.single();

    let mut map_view_sprite: Vec<Vec<char>> = (0..terminal_size.width)
        .map(|x| {
            (0..terminal_size.height)
                .map(|y| {
                    map.map[camera.top_left.x + x as usize][camera.top_left.y + y as usize]
                        .terrain
                        .to_char()
                })
                .collect()
        })
        .collect();

    for (structure, area, rotation, entity_top_left) in entities_query.iter() {
        let entity_bottom_right = Point {
            x: entity_top_left.0.x + area.width,
            y: entity_top_left.0.y + area.height,
        };

        if !rectangles_intersect(
            &camera.top_left,
            &camera_bottom_right,
            &entity_top_left.0,
            &entity_bottom_right,
        ) {
            continue;
        }

        let sprite = structure.to_sprite(area, rotation);
        for (x, sprite_column) in sprite.iter().enumerate() {
            for (y, sprite_cell) in sprite_column.iter().enumerate() {
                if rectangle_contains_point(&camera.top_left, &camera_bottom_right, &Point { x, y })
                {
                    map_view_sprite[entity_top_left.0.x + x - camera.top_left.x]
                        [entity_top_left.0.y + y - camera.top_left.y] = *sprite_cell;
                }
            }
        }
    }

    
}

impl Structure {
    fn to_sprite(&self, area: &Area, rotation: &Rotation) -> Vec<Vec<char>> {
        match self {
            Structure::Building(building) => building.to_sprite(area, rotation),
            Structure::Pathway(pathway) => pathway.to_sprite(area, rotation),
        }
    }
}

impl Building {
    fn to_sprite(&self, area: &Area, _rotation: &Rotation) -> Vec<Vec<char>> {
        let c = match self {
            Building::Commerce => '$',
            Building::Residential => '@',
            Building::Office => '%',
            Building::Industry => '#',
        };
        let mut result: Vec<Vec<char>> = Vec::with_capacity(area.width);
        for x in 0..area.width {
            result[x] = Vec::with_capacity(area.height);
            for _ in 0..area.height {
                result[x].push(c);
            }
        }
        result
    }
}
impl Pathway {
    fn to_sprite(&self, area: &Area, rotation: &Rotation) -> Vec<Vec<char>> {
        let c = match (self, rotation) {
            (Pathway::Road, Rotation::Right) => '═',
            (Pathway::Road, Rotation::Up) => '║',
            (Pathway::Road, Rotation::Left) => '═',
            (Pathway::Road, Rotation::Down) => '║',
        };
        let mut result: Vec<Vec<char>> = Vec::with_capacity(area.width);
        for x in 0..area.width {
            result[x] = Vec::with_capacity(area.height);
            for _ in 0..area.height {
                result[x].push(c);
            }
        }
        result
    }
}
