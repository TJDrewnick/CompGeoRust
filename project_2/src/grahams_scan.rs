use crate::types::{Point, PointVector, TurnType};
use crate::utils::turn_type;

pub fn grahams_scan(input: PointVector) -> PointVector {
    // assume the input is already sorted by x coordinate O(n log n)
    // input.points.sort_by_key(|Point { x, y: _ }| *x);

    // init output to empty
    let mut upper_hull: PointVector = PointVector { points: Vec::new() };

    // add two first points from input to upper hull
    upper_hull.points.push(input.points[0]);
    upper_hull.points.push(input.points[1]);
    // store the index of the last element in the upper hull
    let mut uh_last_idx = 1;

    // loopidy loop
    for i in 2..input.points.len() {
        // remove last upper hull point if it makes a left turn with second last and new point
        while upper_hull.points.len() > 1
            && turn_type(
                upper_hull.points[uh_last_idx - 1],
                upper_hull.points[uh_last_idx],
                input.points[i],
            ) != TurnType::Right
        {
            upper_hull.points.pop();
            uh_last_idx -= 1;
        }
        // no longer a left turn -> new point can be added
        upper_hull.points.push(input.points[i]);
        uh_last_idx += 1;
    }
    upper_hull
}

#[cfg(test)]
mod test {
    use crate::grahams_scan::grahams_scan;
    use crate::input_generation::{Curve, Line};
    use crate::types::{Point, PointVector};

    #[test]
    fn line_hull() {
        let upper_hull = grahams_scan(Line::get_input(10));
        assert_eq!(
            upper_hull.points,
            vec![Point { x: 0, y: 0 }, Point { x: 9, y: 9 }]
        );
    }

    #[test]
    fn curve_hull() {
        let upper_hull = grahams_scan(Curve::get_input(10));
        assert_eq!(upper_hull.points, Curve::get_input(10).points);
    }

    /** One time randomly generated but deterministic to be verifiable */
    #[test]
    fn random_hull() {
        let upper_hull = grahams_scan(PointVector {
            points: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 1, y: 3 },
                Point { x: 2, y: 5 },
                Point { x: 3, y: 2 },
                Point { x: 4, y: 4 },
                Point { x: 4, y: 1 },
                Point { x: 5, y: 6 },
                Point { x: 6, y: 4 },
                Point { x: 7, y: 4 },
            ],
        });
        let expected = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 3 },
            Point { x: 2, y: 5 },
            Point { x: 5, y: 6 },
            Point { x: 7, y: 4 },
        ];
        assert_eq!(expected, upper_hull.points);
    }
}
