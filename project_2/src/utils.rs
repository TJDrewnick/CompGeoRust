use std::cmp::Ordering;
use crate::types::{Point, TurnType};

pub fn turn_type(p1: Point, p2: Point, p3: Point) -> TurnType {
    // determinant
    let determinant = p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y);
    
    match determinant.cmp(&0) {
        Ordering::Greater => TurnType::Left,
        Ordering::Less => TurnType::Right,
        Ordering::Equal => TurnType::Straight,
    }
}
#[cfg(test)]

mod tests {
    use crate::types::{Point, TurnType};
    use crate::utils::turn_type;

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

    fn straight() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = Point { x: 3, y: 3 };
        assert_eq!(turn_type(p1, p2, p3), TurnType::Straight);
    }
}
