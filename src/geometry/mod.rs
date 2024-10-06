use std::ops::{Add, AddAssign, Sub};

use ratatui::layout::Rect;

use crate::map::Segment;

pub mod rectangle;

//TODO: I know this doesn't really make geometric sense but it worked for cursor so probably just
//make this have a more specific name
pub fn rect_contains_coords(rect: Rect, x: isize, y: isize) -> bool {
    x >= rect.left() as isize
        && x < rect.right() as isize
        && y >= rect.top() as isize
        && y < rect.bottom() as isize
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn move_x(&mut self, dx: isize) {
        self.x = self.x.saturating_add_signed(dx);
    }

    pub fn move_y(&mut self, dy: isize) {
        self.y = self.y.saturating_add_signed(dy);
    }

    pub fn clamp_x(&mut self, min: usize, max: usize) {
        self.x = self.x.clamp(min, max);
    }

    pub fn clamp_y(&mut self, min: usize, max: usize) {
        self.y = self.y.clamp(min, max);
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<ratatui::layout::Position> for Point {
    fn from(value: ratatui::layout::Position) -> Self {
        Point {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}
