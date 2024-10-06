use bevy::prelude::*;

use crate::{
    geometry::{rectangle::Rectangle, rectangles_intersect, segment_intersects_rectangle, Point},
    map::{Position, Segment},
};

use super::{Area, Rotation};

pub struct ZonePlugin;

impl Plugin for ZonePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, );
    }
}

#[derive(Bundle, Clone)]
pub struct ZoneBundle {
    zone: Zone,
    area: Area,
    top_left: Position,
}

#[derive(Component, Clone)]
pub enum Zone {
    Residential(Residential),
    Commerce(Commerce),
    Office(Office),
    Industry(Industry),
}

#[derive(Clone)]
pub enum Residential {
    LowDensity,
    MediumDensity,
    MediumDensityMixedUse,
    HighDensity,
    HighDensityMixedUse,
}

#[derive(Clone)]
pub enum Commerce {
    LowDensity,
    HighDensity,
}

#[derive(Clone)]
pub enum Office {
    LowDensity,
    HighDensity,
}

#[derive(Clone)]
pub enum Industry {}

fn zone_area(
    mut commands: Commands,
    zones_query: Query<(&Zone, &Area, &Position, Entity)>,
    top_left: Point,
    bottom_right: Point,
    zone: Zone,
) {
    let zone_rectangle = Rectangle {
        top_left,
        bottom_right,
    };
    for (curr_zone, curr_area, Position(curr_position), curr_entity) in zones_query.into_iter() {
        let curr_rectangle = Rectangle::from_top_left_and_area(&curr_position, &curr_area);

        if !zone_rectangle.intersects(&curr_rectangle) {
            continue;
        }

        let split_current_rectangle: Vec<Rectangle> = curr_rectangle.subtract(&zone_rectangle);
        commands.entity(curr_entity).despawn();

        for rect in split_current_rectangle.iter() {
            commands.spawn(ZoneBundle {
                area: rect.to_area(),
                top_left: Position(rect.top_left.clone()),
                zone: curr_zone.clone(),
            });
        }
    }

    commands.spawn(ZoneBundle {
        area: zone_rectangle.to_area(),
        top_left: Position(zone_rectangle.top_left),
        zone: zone.clone(),
    });
}
