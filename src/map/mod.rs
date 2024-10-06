use std::ops::Add;

use bevy::prelude::*;

use crate::geometry::Point;

#[derive(Component, Clone)]
pub struct Position(pub Point);

impl Position {
    pub fn new(x: u16, y: u16) -> Position {
        Position(Point {
            x: x as usize,
            y: y as usize,
        })
    }
}

impl From<Position> for ratatui::prelude::Position {
    fn from(value: Position) -> Self {
        ratatui::prelude::Position::new(value.0.x as u16, value.0.y as u16)
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0)
    }
}

#[derive(Component)]
pub struct Segment {
    end_point: Point,
    length: usize,
}