use std::ops::{Add, AddAssign};

use crate::map::Segment;

pub mod rectangle;
mod test;

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
