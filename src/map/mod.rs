use bevy::prelude::*;

use crate::geometry::Point;

#[derive(Component)]
pub struct Position(pub Point);

#[derive(Component)]
enum Area {
    
}