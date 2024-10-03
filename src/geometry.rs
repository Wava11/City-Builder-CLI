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

pub struct Point {
    pub x: usize,
    pub y: usize,
}
