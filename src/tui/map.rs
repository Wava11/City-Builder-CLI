use std::clone;

use crate::{
    geometry::{rectangle::Rectangle, Point},
    world::{camera::Camera, structure::Pathway},
};
use bevy::prelude::*;
use ratatui::{
    buffer::Buffer,
    style::{Color, Style},
    widgets::WidgetRef,
};

use crate::{
    map::Position,
    world::{
        structure::{Building, Structure},
        zone, Area, Rotation, WorldMap,
    },
};

use super::Terminal;

#[derive(Component)]
pub struct MapView(Vec<Vec<char>>);

impl WidgetRef for &MapView {
    fn render_ref(&self, area: ratatui::layout::Rect, buf: &mut Buffer) {
        for i in 0..area.height {
            //this might crash if the area ratatui asks for is larger that what was rendered
            let line_text: String = self.0[i as usize].iter().collect();
            let line = ratatui::prelude::Line::from(line_text);
            buf.set_style(area, Style::default().fg(Color::DarkGray));
            buf.set_line(area.x, area.y + i, &line, area.width);
        }
    }
}

pub fn create_map_view_sprite(
    terminal: Res<Terminal>,
    map_query: Query<&WorldMap>,
    structures_query: Query<(&Structure, &Area, &Rotation, &Position)>,
    camera_query: Query<&Camera>,
    mut map_view_query: Query<&mut MapView>,
    mut commands: Commands,
) {
    let camera = camera_query.single();

    let terminal_size = terminal.0.size().unwrap();

    let camera_bottom_right = Point {
        x: camera.top_left.x + terminal_size.width as usize,
        y: camera.top_left.y + terminal_size.height as usize,
    };

    let map = map_query.single();

    let mut map_view_sprite: Vec<Vec<char>> = (0..terminal_size.height)
        .map(|y| {
            (0..terminal_size.width)
                .map(|x| {
                    map.map[camera.top_left.y + y as usize][camera.top_left.x + x as usize]
                        .terrain
                        .to_char()
                })
                .collect()
        })
        .collect();

    let camera_view_rectangle = Rectangle {
        bottom_right: camera_bottom_right,
        top_left: camera.top_left.clone(),
    };

    for (structure, area, rotation, Position(structure_top_left)) in structures_query.iter() {
        let structure_bottom_right = Point {
            x: structure_top_left.x + area.width,
            y: structure_top_left.y + area.height,
        };
        let structure_rectangle = Rectangle {
            bottom_right: structure_bottom_right,
            top_left: structure_top_left.clone(),
        };

        if !camera_view_rectangle.intersects(&structure_rectangle) {
            continue;
        }

        let sprite = structure.to_sprite(area, rotation);
        for (y, sprite_row) in sprite.iter().enumerate() {
            for (x, sprite_cell) in sprite_row.iter().enumerate() {
                let absolute_sprite_pixel_point = Point {
                    x: structure_top_left.x + x,
                    y: structure_top_left.y + y,
                };

                if camera_view_rectangle.contains_point(&absolute_sprite_pixel_point) {
                    map_view_sprite[absolute_sprite_pixel_point.y - camera.top_left.y]
                        [absolute_sprite_pixel_point.x - camera.top_left.x] = *sprite_cell;
                }
            }
        }
    }

    match map_view_query.get_single_mut() {
        Ok(mut map_view_entity) => {
            map_view_entity.0 = map_view_sprite;
        }
        Err(_) => {
            commands.spawn(MapView(map_view_sprite));
        }
    };
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
            Building::Zoned(zone::Zone::Commerce(_)) => '$',
            Building::Zoned(zone::Zone::Residential(_)) => '@',
            Building::Zoned(zone::Zone::Office(_)) => '%',
            Building::Zoned(zone::Zone::Industry(_)) => '#',
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
            result.push(Vec::with_capacity(area.height));
            for _ in 0..area.height {
                result[x].push(c);
            }
        }
        result
    }
}
