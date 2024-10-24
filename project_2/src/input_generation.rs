use crate::types::{Point, PointVector};
use turborand::prelude::*;

// the different types of input
pub struct UniformSquare();
pub struct UniformCircle();
pub struct Curve();
pub struct InverseCurve();
pub struct Line();

// implementing the input generation
impl UniformSquare {
    pub fn get_input(amount: i64) -> PointVector {
        let side_length = f64::sqrt(amount as f64) as i64 * 5;
        // get x and y randomly
        let rand = Rng::new();
        PointVector {
            points: (0..amount)
                .map(|_| Point {
                    x: rand.i64(0..=side_length),
                    y: rand.i64(0..=side_length),
                })
                .collect(),
        }
    }
}

impl UniformCircle {
    pub fn get_input(amount: i64) -> PointVector {
        let radius = f64::sqrt(amount as f64) as i64 * 5;
        // use rejection sampling

        let rand = Rng::new();
        let mut vec: Vec<Point> = Vec::with_capacity(amount as usize);

        while vec.len() < amount as usize {
            let point = Point {
                x: rand.i64(-radius..=radius),
                y: rand.i64(-radius..=radius),
            };

            if point.x.pow(2) + point.y.pow(2) <= radius.pow(2) {
                vec.push(point);
            }
        }
        PointVector { points: vec }
    }
}

impl Curve {
    pub fn get_input(length: i64) -> PointVector {
        PointVector {
            points: (0..length).map(|i| Point { x: i, y: -(i * i) }).collect(),
        }
    }
}

impl InverseCurve {
    pub fn get_input(length: i64) -> PointVector {
        PointVector {
            points: (0..length).map(|i| Point { x: i, y: i * i }).collect(),
        }
    }
}

impl Line {
    pub fn get_input(length: i64) -> PointVector {
        PointVector {
            points: (0..length).map(|i| Point { x: i, y: i }).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input_generation::{Curve, InverseCurve, Point, UniformCircle};

    #[test]
    fn uniform_circle_test() {
        let uniform_circle_input = UniformCircle::get_input(10);
        assert_eq!(uniform_circle_input.points.len(), 10);
    }

    #[test]
    fn curve_test() {
        let curve_input = Curve::get_input(10);
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: -1 },
            Point { x: 2, y: -4 },
            Point { x: 3, y: -9 },
            Point { x: 4, y: -16 },
            Point { x: 5, y: -25 },
            Point { x: 6, y: -36 },
            Point { x: 7, y: -49 },
            Point { x: 8, y: -64 },
            Point { x: 9, y: -81 },
        ];
        assert_eq!(expected, curve_input.points);
    }

    #[test]
    fn inverse_curve_test() {
        let inverse_curve_input = InverseCurve::get_input(10);
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 4 },
            Point { x: 3, y: 9 },
            Point { x: 4, y: 16 },
            Point { x: 5, y: 25 },
            Point { x: 6, y: 36 },
            Point { x: 7, y: 49 },
            Point { x: 8, y: 64 },
            Point { x: 9, y: 81 },
        ];
        assert_eq!(expected, inverse_curve_input.points);
    }
}
