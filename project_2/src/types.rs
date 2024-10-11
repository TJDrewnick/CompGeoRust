// data types for input generation
#[derive(Debug, PartialEq, Copy, Clone, Ord, Eq, PartialOrd)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointVector {
    pub points: Vec<Point>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TurnType {
    Left,
    Right,
    Straight,
}
