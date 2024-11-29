use crate::world::Area;

use super::{interval::Interval, Point};
mod test;

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn from_coordinates((left, top): (usize, usize), (right, bottom): (usize, usize)) -> Self {
        Self {
            top_left: Point { x: left, y: top },
            bottom_right: Point {
                x: right,
                y: bottom,
            },
        }
    }
    pub fn from_top_left_and_area(top_left: &Point, area: &Area) -> Self {
        Self {
            bottom_right: Point {
                x: top_left.x + area.width,
                y: top_left.y + area.height,
            },
            top_left: top_left.clone(),
        }
    }
    fn from_intervals(x: &Interval, y: &Interval) -> Self {
        Self {
            top_left: Point {
                x: x.start,
                y: y.start,
            },
            bottom_right: Point { x: x.end, y: y.end },
        }
    }

    pub fn to_area(&self) -> Area {
        Area {
            width: self.bottom_right.x - self.top_left.x,
            height: self.bottom_right.y - self.top_left.y,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.top_left.x < other.bottom_right.x
            && self.bottom_right.x > other.top_left.x
            && self.top_left.y < other.bottom_right.y
            && self.bottom_right.y > other.top_left.y
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        let x_intersection = self.x_interval().intersection(&other.x_interval());
        let y_intersection = self.y_interval().intersection(&other.y_interval());
        match (x_intersection, y_intersection) {
            (Some(x_intersection), Some(y_intersection)) => {
                Some(Rectangle::from_intervals(&x_intersection, &y_intersection))
            }
            _ => None,
        }
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.top_left.x
            && point.x <= self.bottom_right.x
            && point.y >= self.top_left.y
            && point.y <= self.bottom_right.y
    }

    pub fn intersects_segment(&self, segment_p1: &Point, segment_p2: &Point) -> bool {
        self.intersects(&Self::from_segment_end_points(segment_p1, segment_p2))
    }

    fn from_segment_end_points(segment_p1: &Point, segment_p2: &Point) -> Self {
        let (top_left, bottom_right) =
            if segment_p1.x <= segment_p2.x && segment_p1.y <= segment_p2.y {
                (segment_p1, segment_p2)
            } else {
                (segment_p2, segment_p1)
            };
        Self {
            top_left: top_left.clone(),
            bottom_right: bottom_right.clone(),
        }
    }

    fn top(&self) -> usize {
        self.top_left.y
    }
    fn right(&self) -> usize {
        self.bottom_right.x
    }
    fn bottom(&self) -> usize {
        self.bottom_right.y
    }
    fn left(&self) -> usize {
        self.top_left.x
    }

    fn complement(&self) -> Vec<Self> {
        let mut result: Vec<Self> = Vec::with_capacity(4);
        if self.top() > 0 {
            result.push(Self::from_intervals(
                &Interval::MAX,
                &Interval::new(0, self.top() - 1),
            ));
        }
        if self.right() < usize::MAX {
            result.push(Self::from_intervals(
                &Interval::new(self.right() + 1, usize::MAX),
                &self.y_interval(),
            ));
        }
        if self.bottom() < usize::MAX {
            result.push(Self::from_intervals(
                &Interval::MAX,
                &Interval::new(self.bottom() + 1, usize::MAX),
            ));
        }
        if self.left() > 0 {
            result.push(Self::from_intervals(
                &Interval::new(0, self.left() - 1),
                &self.y_interval(),
            ));
        }

        result
    }

    pub fn subtract(&self, other: &Rectangle) -> Vec<Rectangle> {
        other
            .complement()
            .into_iter()
            .filter_map(|other_complement| self.intersection(&other_complement))
            .collect()
    }

    fn x_interval(&self) -> Interval {
        Interval::new(self.top_left.x, self.bottom_right.x)
    }
    fn y_interval(&self) -> Interval {
        Interval::new(self.top_left.y, self.bottom_right.y)
    }
}
