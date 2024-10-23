use crate::types::{Point, PointVector, Side, TurnType};
use std::cmp::Ordering;

pub fn turn_type(p1: Point, p2: Point, p3: Point) -> TurnType {
    // determinant
    let determinant = p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y);

    match determinant.cmp(&0) {
        Ordering::Greater => TurnType::Left,
        Ordering::Less => TurnType::Right,
        Ordering::Equal => TurnType::Straight,
    }
}

pub fn get_point_side(hull: &PointVector, point: Point) -> Side {
    match hull.points[0].x.cmp(&(point.x)) {
        Ordering::Greater => Side::Left,
        Ordering::Less => Side::Right,
        Ordering::Equal => Side::Left,
    }
}

#[cfg(test)]

mod tests {
    use crate::types::{Point, PointVector, Side, TurnType};
    use crate::utils::{get_point_side, turn_type};

    #[test]
    fn left_turn() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = Point { x: 3, y: 4 };
        assert_eq!(turn_type(p1, p2, p3), TurnType::Left);
    }

    #[test]
    fn right_turn() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = Point { x: 3, y: 1 };
        assert_eq!(turn_type(p1, p2, p3), TurnType::Right);
    }

    #[test]
    fn straight() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = Point { x: 3, y: 3 };
        assert_eq!(turn_type(p1, p2, p3), TurnType::Straight);
    }

    #[test]
    fn straight_reverse() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 3, y: 3 };
        let p3 = Point { x: 2, y: 2 };
        assert_eq!(turn_type(p1, p2, p3), TurnType::Straight);
    }

    #[test]
    fn point_left_of_hull() {
        let upper_hull: PointVector = PointVector {
            points: vec![
                Point { x: 6, y: 1 },
                Point { x: 8, y: 3 },
                Point { x: 11, y: 4 },
            ],
        };
        let point = Point { x: 1, y: 1 };
        assert_eq!(get_point_side(&upper_hull, point), Side::Left);
    }

    #[test]
    fn point_right_of_hull() {
        let upper_hull: PointVector = PointVector {
            points: vec![
                Point { x: 6, y: 1 },
                Point { x: 8, y: 3 },
                Point { x: 11, y: 4 },
            ],
        };
        let point = Point { x: 20, y: 1 };
        assert_eq!(get_point_side(&upper_hull, point), Side::Right);
    }
}
