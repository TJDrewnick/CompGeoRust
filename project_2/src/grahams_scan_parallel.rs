use crate::grahams_scan::grahams_scan;
use crate::types::{Errors, Point, PointVector, Side, Tangent, TurnType};
use crate::utils::{get_point_side, turn_type};
use std::thread::ScopedJoinHandle;

pub fn grahams_scan_parallel(
    mut input: PointVector,
    sort_input: Option<bool>,
    processors: Option<usize>,
) -> PointVector {
    // most of the time the input is already sorted by x coordinate O(n log n),
    // but allow to also sort if needed
    if sort_input.unwrap_or(true) {
        input.points.sort_by_key(|Point { x, y: _ }| *x);
    }

    let processors = processors.unwrap_or(1);

    let splits = input.points.len() / processors;
    let mut upper_hulls: Vec<PointVector> = Vec::with_capacity(processors);

    // calculate upper hulls in parallel for subsets of all points
    std::thread::scope(|scope| {
        let last_split = PointVector {
            points: input.points[(processors - 1) * splits..].to_vec(),
        };

        let mut handles: Vec<ScopedJoinHandle<PointVector>> = Vec::with_capacity(processors - 1);

        // store handles of each thread for the first p-1 splits
        for i in 1..processors {
            let current_split = PointVector {
                points: input.points[(i - 1) * splits..i * splits].to_vec(),
            };

            let handle = scope.spawn(|| grahams_scan(current_split, Option::from(false), None));
            handles.push(handle);
        }
        // get result from last split of Points
        let last_result = grahams_scan(last_split, Option::from(false), None);

        // collect all results
        for handle in handles {
            if let Ok(result) = handle.join() {
                upper_hulls.push(result);
            }
        }
        upper_hulls.push(last_result);
    });

    // merge upper hulls
    let mut i = 0;
    while i < processors - 1 {
        let mut tangents: Vec<Tangent> = vec![]; // indices of the upper hulls and indices of the points that form the tangent
        let left_hull = upper_hulls[i].clone();

        for (j, right_hull) in upper_hulls.iter().enumerate().take(processors).skip(i + 1) {
            // find tangent between i and j
            let (l, r) = get_tangent(&left_hull, right_hull);
            tangents.push(Tangent {
                left_hull_idx: i,
                right_hull_idx: j,
                left_point_idx: l,
                right_point_idx: r,
            });
        }

        // tangent, k, with the smallest rotation will be used as bridge.
        // tangent k should have all other convex hulls (and also all other points) to the right of it.
        // --> start point of tangent k, end point of tangent k should form a right turn with all other convex hulls (maybe with their end point?)

        // find k
        let mut idx_in_tangents: usize = 0;
        for (k, tangent) in tangents.iter().enumerate() {
            let mut failed: bool = false;

            for Tangent {
                left_hull_idx: _,
                right_hull_idx: hull_idx,
                left_point_idx: _,
                right_point_idx: point_idx,
            } in tangents.iter().skip(k + 1)
            {
                match turn_type(
                    upper_hulls[tangent.left_hull_idx].points[tangent.left_point_idx],
                    upper_hulls[tangent.right_hull_idx].points[tangent.right_point_idx],
                    upper_hulls[*hull_idx].points[*point_idx],
                ) {
                    TurnType::Left => {
                        // only right turns permitted
                        failed = true;
                        break;
                    }
                    TurnType::Right => {
                        continue;
                    }
                    TurnType::Straight => {
                        // next point in line is guaranteed to have the same right turns
                        failed = true;
                        break;
                    }
                }
            }
            if failed {
                continue;
            }
            idx_in_tangents = k;
            break;
        }

        // set line segment as bridge between convex hull i and convex hull idx_in_tangents.
        // keep all points to the left of the first index (including the index) and all the points to the right of the second index (including the index)

        let tangent = &tangents[idx_in_tangents];

        let right_uh_idx = tangent.right_hull_idx;

        // delete points in first upper hull after tangent origin point
        upper_hulls[i].points.truncate(tangent.left_point_idx + 1);

        // delete points in destination upper hull after tangent destination point
        upper_hulls[right_uh_idx]
            .points
            .drain(0..tangent.right_point_idx);

        // delete upper hulls between origin and destination upper hull
        for upper_hull in upper_hulls.iter_mut().take(right_uh_idx).skip(i + 1) {
            upper_hull.points = vec![];
        }

        // continue bridging from the right_uh_idx'th upper hull
        i = right_uh_idx;
    }

    // merge split upper hulls and return full upper hull
    PointVector {
        points: upper_hulls
            .into_iter()
            .fold(vec![], |acc, hull| [acc, hull.points].concat()),
    }
}

/**
   Given two upper hulls it returns the index of the two points forming a tangent line between them
*/
fn get_tangent(left_hull: &PointVector, right_hull: &PointVector) -> (usize, usize) {
    let mut point_idx = right_hull.points.len() / 2;

    loop {
        // search in left hull
        let result = get_tangent_from_point(left_hull, right_hull.points[point_idx]);
        match result {
            Ok(index) => {
                let new_point_idx = index;
                if is_upper_hull_tangent(right_hull, point_idx, left_hull.points[new_point_idx]) {
                    return (new_point_idx, point_idx);
                }
                point_idx = new_point_idx;
            }
            Err(err) => {
                eprintln!("{:?}", err);
                panic!();
            }
        }
        // search in right hull
        let result = get_tangent_from_point(right_hull, left_hull.points[point_idx]);
        match result {
            Ok(index) => {
                let new_point_idx = index;
                if is_upper_hull_tangent(left_hull, point_idx, right_hull.points[new_point_idx]) {
                    return (point_idx, new_point_idx);
                }
                point_idx = new_point_idx;
            }
            Err(err) => {
                eprintln!("{:?}", err);
                panic!();
            }
        }
    }
}

fn is_upper_hull_tangent(hull: &PointVector, index: usize, point: Point) -> bool {
    let point_side = get_point_side(hull, point);

    if hull.points.len() == 1 {
        return true;
    } else if hull.points.len() == 2 {
        let turn = turn_type(point, hull.points[0], hull.points[1]);
        return match (point_side, turn) {
            (Side::Left, TurnType::Left) => 1 == index,
            (Side::Left, TurnType::Right) => 0 == index,
            (Side::Left, TurnType::Straight) => 1 == index,
            (Side::Right, TurnType::Left) => 0 == index,
            (Side::Right, TurnType::Right) => 1 == index,
            (Side::Right, TurnType::Straight) => 0 == index,
        };
    }

    let tangent_point = hull.points[index];

    // check edges for len 3 or more
    if index == 0 || index == hull.points.len() - 1 {
        let neighbour = match index {
            0 => hull.points[1],
            _ => hull.points[index - 1],
        };

        if point_side == Side::Left {
            return turn_type(point, tangent_point, neighbour) == TurnType::Right;
        } else if point_side == Side::Right {
            return turn_type(point, tangent_point, neighbour) == TurnType::Left;
        }
    }

    // check neighbours
    let left_neighbour = hull.points[index - 1];
    let right_neighbour = hull.points[index + 1];

    let to_left = turn_type(point, tangent_point, left_neighbour);
    let to_right = turn_type(point, tangent_point, right_neighbour);

    match (point_side, to_left, to_right) {
        // whole convex hull below
        (Side::Left, TurnType::Right, TurnType::Right)
        | (Side::Left, TurnType::Straight, TurnType::Right)
        | (Side::Right, TurnType::Left, TurnType::Left)
        | (Side::Right, TurnType::Left, TurnType::Straight) => true,
        // otherwise
        _ => false,
    }
}

/**
    Given an upper hull and a point, it returns the index in the upper hull of the point that
    forms the tangent with the given point. Assumes that the upper hull points are sorted by
    x-coordinate.
*/
fn get_tangent_from_point(hull: &PointVector, point: Point) -> Result<usize, Errors> {
    let point_side = get_point_side(hull, point);

    if hull.points.len() == 1 {
        Ok(0)
    } else if hull.points.len() == 2 {
        let turn = turn_type(point, hull.points[0], hull.points[1]);
        return match (point_side, turn) {
            (Side::Left, TurnType::Left) => Ok(1),
            (Side::Left, TurnType::Right) => Ok(0),
            (Side::Left, TurnType::Straight) => Ok(1),
            (Side::Right, TurnType::Left) => Ok(0),
            (Side::Right, TurnType::Right) => Ok(1),
            (Side::Right, TurnType::Straight) => Ok(0),
        };
    } else {
        // hull is at least 3 elements long
        let mut candidate = hull.points.len() / 2;

        loop {
            // if moved to edge, this is the connection to the upper hull
            if candidate == 0 {
                return Ok(0);
            } else if candidate == hull.points.len() - 1 {
                return Ok(hull.points.len() - 1);
            }

            let left = candidate - 1;
            let right = candidate + 1;

            let to_left = turn_type(point, hull.points[candidate], hull.points[left]);
            let to_right = turn_type(point, hull.points[candidate], hull.points[right]);

            match (point_side, to_left, to_right) {
                // whole convex hull below
                (Side::Left, TurnType::Right, TurnType::Right)
                | (Side::Left, TurnType::Straight, TurnType::Right)
                | (Side::Right, TurnType::Left, TurnType::Left)
                | (Side::Right, TurnType::Left, TurnType::Straight) => break,
                // move to right
                (Side::Left, TurnType::Right, TurnType::Left)
                | (Side::Left, TurnType::Right, TurnType::Straight)
                | (Side::Left, TurnType::Straight, TurnType::Straight)
                | (Side::Right, TurnType::Left, TurnType::Right) => candidate = right,
                // move to left
                (Side::Left, TurnType::Left, TurnType::Right)
                | (Side::Right, TurnType::Right, TurnType::Left)
                | (Side::Right, TurnType::Straight, TurnType::Left)
                | (Side::Right, TurnType::Straight, TurnType::Straight) => candidate = left,
                // given hull was not an upper hull
                (Side::Left, TurnType::Left, TurnType::Left)
                | (Side::Left, TurnType::Left, TurnType::Straight)
                | (Side::Left, TurnType::Straight, TurnType::Left)
                | (Side::Right, TurnType::Right, TurnType::Right)
                | (Side::Right, TurnType::Right, TurnType::Straight)
                | (Side::Right, TurnType::Straight, TurnType::Right) => {
                    return Err(Errors::LowerHullError)
                }
            }
        }

        Ok(candidate)
    }
}

#[cfg(test)]
mod test {
    use crate::grahams_scan_parallel::{
        get_tangent, get_tangent_from_point, grahams_scan_parallel, is_upper_hull_tangent,
    };
    use crate::input_generation::{Curve, Line};
    use crate::types::{Point, PointVector};

    #[test]
    fn line_hull_2p() {
        let upper_hull: PointVector =
            grahams_scan_parallel(Line::get_input(50), Option::from(false), Option::from(2));
        assert_eq!(
            upper_hull.points,
            vec![Point { x: 0, y: 0 }, Point { x: 49, y: 49 }]
        );
    }

    #[test]
    fn line_hull_6p() {
        let upper_hull: PointVector =
            grahams_scan_parallel(Line::get_input(50), Option::from(false), Option::from(6));
        assert_eq!(
            upper_hull.points,
            vec![Point { x: 0, y: 0 }, Point { x: 49, y: 49 }]
        );
    }

    #[test]
    fn curve_hull_2p() {
        let upper_hull: PointVector =
            grahams_scan_parallel(Curve::get_input(50), Option::from(false), Option::from(2));
        assert_eq!(upper_hull.points, Curve::get_input(50).points);
    }

    #[test]
    fn curve_hull_6p() {
        let upper_hull: PointVector =
            grahams_scan_parallel(Curve::get_input(50), Option::from(false), Option::from(6));
        assert_eq!(upper_hull.points, Curve::get_input(50).points);
    }

    #[test]
    fn tangent_from_point() {
        let upper_hull: PointVector = PointVector {
            points: vec![
                Point { x: 6, y: 1 },
                Point { x: 8, y: 3 },
                Point { x: 11, y: 4 },
                Point { x: 12, y: 3 },
                Point { x: 13, y: 1 },
            ],
        };
        let point: Point = Point { x: 1, y: 3 };
        assert_eq!(get_tangent_from_point(&upper_hull, point), Ok(2));
    }

    #[test]
    fn is_upper_hull_tangent_len4() {
        let upper_hull: PointVector = PointVector {
            points: vec![
                Point { x: 3, y: 0 },
                Point { x: 6, y: 4 },
                Point { x: 8, y: 5 },
                Point { x: 10, y: 3 },
            ],
        };
        let point: Point = Point { x: 2, y: 1 };
        assert!(is_upper_hull_tangent(&upper_hull, 1, point));
    }

    #[test]
    fn is_upper_hull_tangent_len2() {
        let upper_hull: PointVector = PointVector {
            points: vec![Point { x: 3, y: 0 }, Point { x: 6, y: 4 }],
        };
        let point: Point = Point { x: 2, y: 1 };
        assert!(is_upper_hull_tangent(&upper_hull, 1, point));
    }

    #[test]
    fn is_upper_hull_tangent_len1() {
        let upper_hull: PointVector = PointVector {
            points: vec![Point { x: 3, y: 1 }],
        };
        let point: Point = Point { x: 0, y: 0 };
        assert!(is_upper_hull_tangent(&upper_hull, 0, point));
    }

    #[test]
    fn tangent_between_two_hulls_random() {
        let left_hull: PointVector = PointVector {
            points: vec![
                Point { x: 2, y: 1 },
                Point { x: 3, y: 3 },
                Point { x: 5, y: 4 },
                Point { x: 9, y: 3 },
                Point { x: 11, y: 1 },
            ],
        };
        let right_hull: PointVector = PointVector {
            points: vec![
                Point { x: 12, y: 5 },
                Point { x: 14, y: 7 },
                Point { x: 16, y: 6 },
                Point { x: 17, y: 2 },
            ],
        };
        assert_eq!(get_tangent(&left_hull, &right_hull), (2, 1));
    }

    #[test]
    fn tangent_between_two_hulls_curve() {
        let left_hull: PointVector = PointVector {
            points: vec![
                Point { x: 1, y: -1 },
                Point { x: 2, y: -4 },
                Point { x: 3, y: -9 },
                Point { x: 4, y: -16 },
            ],
        };
        let right_hull: PointVector = PointVector {
            points: vec![
                Point { x: 5, y: -25 },
                Point { x: 6, y: -36 },
                Point { x: 7, y: -49 },
                Point { x: 8, y: -64 },
            ],
        };
        assert_eq!(get_tangent(&left_hull, &right_hull), (3, 0));
    }

    #[test]
    fn tangent_between_two_hulls_curve_skip1() {
        let left_hull: PointVector = PointVector {
            points: vec![
                Point { x: 1, y: -1 },
                Point { x: 2, y: -4 },
                Point { x: 3, y: -9 },
                Point { x: 4, y: -16 },
            ],
        };
        let right_hull: PointVector = PointVector {
            points: vec![
                Point { x: 6, y: -36 },
                Point { x: 7, y: -49 },
                Point { x: 8, y: -64 },
                Point { x: 9, y: -81 },
            ],
        };
        assert_eq!(get_tangent(&left_hull, &right_hull), (3, 0));
    }
}
