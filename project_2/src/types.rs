use std::ops::Range;

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

// result collection
pub type InputFunction = fn(i64) -> PointVector;
pub type ConvexHullAlgorithm = fn(PointVector, Option<bool>, Option<usize>) -> PointVector;

#[derive(Debug, PartialEq, Clone)]
pub struct Plot {
    pub title: String,
    pub path: String,
    pub experiments: Vec<Experiment>,
    pub input_sizes: Vec<i64>,
    pub algorithm: ConvexHullAlgorithm,
    pub args: (Option<bool>, Option<usize>),
    pub y_range: Range<f64>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Experiment {
    pub name: String,
    pub run_times: Vec<f64>,
    pub upper_hull_lengths: Vec<usize>,
}
