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
pub struct Tangent {
    pub left_hull_idx: usize,
    pub right_hull_idx: usize,

    pub left_point_idx: usize,
    pub right_point_idx: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TurnType {
    Left,
    Right,
    Straight,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Errors {
    LowerHullError,
}
