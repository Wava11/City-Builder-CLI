use crate::world::Area;

use super::Point;
mod test;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn from_coordinates(
        (left, top): (usize, usize),
        (right, bottom): (usize, usize),
    ) -> Rectangle {
        Rectangle {
            top_left: Point { x: left, y: top },
            bottom_right: Point {
                x: right,
                y: bottom,
            },
        }
    }
    pub fn from_top_left_and_area(top_left: &Point, area: &Area) -> Rectangle {
        Rectangle {
            bottom_right: Point {
                x: top_left.x + area.width,
                y: top_left.y + area.height,
            },
            top_left: top_left.clone(),
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

    pub fn subtract(&self, other: &Rectangle) -> Vec<Rectangle> {
        if self.intersects(other) {
            let interval_above_other = (0, other.top_left.y);
            let interval_below_other = (other.bottom_right.y, usize::MAX);

            let self_y_interval = (self.top_left.y, self.bottom_right.y);

            let above_other_and_intersects_self = (
                usize::max(self_y_interval.0, interval_above_other.0),
                usize::min(self_y_interval.1, interval_above_other.1),
            );
            let below_other_and_intersects_self = (
                usize::max(self_y_interval.0, interval_below_other.0),
                usize::min(self_y_interval.1, interval_below_other.1),
            );

            let mut result: Vec<Rectangle> = vec![];
            if above_other_and_intersects_self.0 < above_other_and_intersects_self.1 {
                result.push(Rectangle {
                    top_left: self.top_left.clone(),
                    bottom_right: Point {
                        x: self.bottom_right.x,
                        y: above_other_and_intersects_self.1,
                    },
                })
            }
            if below_other_and_intersects_self.0 < below_other_and_intersects_self.1 {
                result.push(Rectangle {
                    top_left: Point {x: self.top_left.x, y: below_other_and_intersects_self.0},
                    bottom_right: self.bottom_right.clone()
                })
            }

            return result;
        }
        vec![self.clone()]
    }
}
