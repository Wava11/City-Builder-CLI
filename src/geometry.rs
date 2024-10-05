use std::ops::{Add, AddAssign};

pub fn rectangles_intersect(
    top_left1: &Point,
    bottom_right1: &Point,
    top_left2: &Point,
    bottom_right2: &Point,
) -> bool {
    top_left1.x < bottom_right2.x
        && bottom_right1.x > top_left2.x
        && top_left1.y > bottom_right2.y
        && bottom_right1.y < top_left2.y
}

pub fn rectangle_contains_point(top_left: &Point, bottom_right: &Point, point: &Point) -> bool {
    point.x >= top_left.x
        && point.x <= bottom_right.x
        && point.y >= top_left.y
        && point.y <= bottom_right.y
}

#[derive(Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn move_x(&mut self, x: isize) {
        let new_x = self.x.saturating_add_signed(x);
        self.x = new_x;
    }

    pub fn move_y(&mut self, y: isize) {
        let new_y = self.y.saturating_add_signed(y);
        self.y = new_y;
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
