use crate::types::{Point, PointVector, TurnType};
use crate::utils::turn_type;

pub fn gift_wrapping_upper_hull(
    mut input: PointVector,
    _: Option<bool>,
    _: Option<usize>,
) -> PointVector {
    // init - find leftmost point
    let (leftmost_idx, leftmost) = input
        .points
        .iter()
        .enumerate()
        .min_by_key(|(_, Point { x, y: _ })| *x)
        .unwrap();

    // leftmost point is guaranteed to be in upper hull
    let mut upper_hull: PointVector = PointVector {
        points: vec![*leftmost],
    };

    input.points.swap_remove(leftmost_idx);

    // stopping point is rightmost point
    let rightmost = input
        .points
        .clone()
        .into_iter()
        .max_by_key(|Point { x, y: _ }| *x)
        .unwrap();

    // find the next pivot point - check for each point chosen if all other points are to the right of it
    loop {
        // if last upper hull point == rightmost point, we are done
        let last_uh_point = upper_hull.points[upper_hull.points.len() - 1];
        if last_uh_point.x == rightmost.x {
            break;
        }
        let mut p = input.points[0];
        let mut idx_p = 0;
        // look at other points and whether they make a left turn with line that ends at p.
        // If left turn, use this point as the new pivot.
        // If no left turn is found, add p to the upper hull
        for i in 1..input.points.len() {
            let q = input.points[i];
            if turn_type(last_uh_point, p, q) == TurnType::Left {
                // we know that all other points have been right turns, if we have not entered this condition yet.
                // therefore, switch pivot to the point q, as this is left of p and all the other points we have checked so far.
                p = q;
                idx_p = i;
                continue;
            }
        }
        // if p has all points to the right of it, add to convex hull
        upper_hull.points.push(p);
        input.points.swap_remove(idx_p);
    }

    upper_hull
}

#[cfg(test)]
mod test {
    use crate::gift_wrapping::gift_wrapping_upper_hull;
    use crate::input_generation::{Curve, Line};
    use crate::types::{Point, PointVector};

    #[test]
    fn line_hull() {
        let upper_hull = gift_wrapping_upper_hull(Line::get_input(10), None, None);
        assert_eq!(
            upper_hull.points,
            vec![Point { x: 0, y: 0 }, Point { x: 9, y: 9 }]
        );
    }

    #[test]
    fn curve_hull() {
        let upper_hull = gift_wrapping_upper_hull(Curve::get_input(10), None, None);
        assert_eq!(upper_hull.points, Curve::get_input(10).points);
    }

    /** One time randomly generated but deterministic to be verifiable */
    #[test]
    fn random_hull() {
        let upper_hull = gift_wrapping_upper_hull(
            PointVector {
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
            },
            None,
            None,
        );
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
