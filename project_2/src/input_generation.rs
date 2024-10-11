use crate::types::{Point, PointVector};
use turborand::prelude::*;

// the different types of input
pub struct UniformSquare();
pub struct UniformCircle();
pub struct Curve();
pub struct Line();

// implementing the input generation
impl UniformSquare {
    pub fn get_input(amount: i64, side_length: i64) -> PointVector {
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
    pub fn get_input(amount: usize, radius: i64) -> PointVector {
        // use rejection sampling

        let rand = Rng::new();
        let mut vec: Vec<Point> = Vec::with_capacity(amount);

        while vec.len() < amount {
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

impl Line {
    pub fn get_input(length: i64) -> PointVector {
        PointVector {
            points: (0..length).map(|i| Point { x: i, y: i }).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input_generation::{Curve, Point, UniformCircle, UniformSquare};

    #[test]
    fn uniform_square_test() {
        let amount = 10000;
        let mut side_length = 10;
        let uniform_square_input = UniformSquare::get_input(amount, side_length);

        side_length += 1;

        let mut x_bins = vec![0; side_length as usize];
        let mut y_bins = vec![0; side_length as usize];

        uniform_square_input.points.iter().for_each(|point| {
            x_bins[point.x as usize] += 1;
            y_bins[point.y as usize] += 1;
        });
        for i in 0..side_length as usize {
            assert!((x_bins[i] - (amount / side_length)).abs() <= (amount / (side_length * 10)));
            assert!((y_bins[i] - (amount / side_length)).abs() <= (amount / (side_length * 10)));
        }

        assert_eq!(uniform_square_input.points.len(), amount as usize);
    }

    #[test]
    fn uniform_circle_test() {
        let uniform_circle_input = UniformCircle::get_input(10, 5);
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
}
