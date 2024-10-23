use crate::grahams_scan::grahams_scan;
use crate::types::{Errors, Point, PointVector, Side, TurnType};
use crate::utils::turn_type;
use std::cmp::Ordering;
use std::thread::ScopedJoinHandle;

pub fn grahams_scan_parallel(input: PointVector, processors: usize) -> PointVector {
    // assume the input is already sorted by x coordinate O(n log n)
    // input.points.sort_by_key(|Point { x, y: _ }| *x);

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

            let handle = scope.spawn(|| grahams_scan(current_split));
            handles.push(handle);
        }
        // get result from last split of Points
        let last_result = grahams_scan(last_split);

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
    while i < processors {
        let mut uh_indices_and_tangent_indices: Vec<((usize, usize), (usize, usize))> = vec![]; // indices of the upper hulls and indices of the points that form the tangent
        let left_hull = upper_hulls[i].clone();

        for j in (i + 1)..processors {
            let right_hull = upper_hulls[j].clone();
            // find tangent between i and j
            uh_indices_and_tangent_indices.push(((i, j), get_tangent(&left_hull, &right_hull)));
        }
        // tangent, k, with the smallest rotation will be used as bridge.
        // tangent k should have all other convex hulls (and also all other points) to the right of it.
        // --> start point of tangent k, end point of tangent k should form a right turn with all other convex hulls (maybe with their end point?)

        // find k
        for k in 0..uh_indices_and_tangent_indices.len() {
            let uh_idx_and_tangent_idx: ((usize, usize), (usize, usize)) =
                uh_indices_and_tangent_indices[k];
            for idx_next in (k + 1)..uh_indices_and_tangent_indices.len() {
                let next_tangent: ((usize, usize), (usize, usize)) =
                    uh_indices_and_tangent_indices[idx_next];
                if turn_type(
                    upper_hulls[uh_idx_and_tangent_idx.0 .0].points[uh_idx_and_tangent_idx.1 .0],
                    upper_hulls[uh_idx_and_tangent_idx.0 .1].points[uh_idx_and_tangent_idx.1 .1],
                    upper_hulls[next_tangent.0 .1].points[next_tangent.1 .1],
                ) == TurnType::Left
                {
                    // bail out fast: if forms left turn, try next tangent
                    continue;
                }
            }
            // set line segment as bridge between convex hull i and convex hull k.
            // keep all points to the left of the first index (including the index) and all the points to the right of the second index (including the index)
            upper_hulls[uh_idx_and_tangent_idx.0 .0].points =
                upper_hulls[uh_idx_and_tangent_idx.0 .0].points[0..uh_idx_and_tangent_idx.1 .0 + 1]
                    .to_vec(); // keep left_uh points up to and including the tangent idx
            upper_hulls[uh_idx_and_tangent_idx.0 .1].points =
                upper_hulls[uh_idx_and_tangent_idx.0 .1].points[uh_idx_and_tangent_idx.1 .1..]
                    .to_vec(); // keep right_uh points from and including the tangent idx
            // remove all upper hulls between the bridged upper hulls
            for remove_idx in (uh_idx_and_tangent_idx.0.0 + 1) .. uh_idx_and_tangent_idx.0.1 {
                upper_hulls.swap_remove(remove_idx);
            }

            // continue bridging from the k'th upper hull
            i = k
        }
    }
    // merge split upper hulls and return full upper hull
    let mut result: PointVector = PointVector {points: vec![]};
    for uh in upper_hulls {
        result.points.extend(uh.points.iter());
    }
    result
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
                if is_upper_hull_tangent(right_hull, left_hull.points[new_point_idx]) {
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
                if is_upper_hull_tangent(left_hull, right_hull.points[new_point_idx]) {
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

fn is_upper_hull_tangent(hull: &PointVector, point: Point) -> bool {
    
    unimplemented!()
}

/**
    Given an upper hull and a point, it returns the index in the upper hull of the point that
    forms the tangent with the given point. Assumes that the upper hull points are sorted by
    x-coordinate.
*/
fn get_tangent_from_point(hull: &PointVector, point: Point) -> Result<usize, Errors> {
    let point_side = match hull.points[0].x.cmp(&(point.x)) {
        Ordering::Greater => Side::Left,
        Ordering::Less => Side::Right,
        Ordering::Equal => Side::Left,
    };

    if hull.points.len() == 1 {
        return Ok(0);
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
    }

    // hull is at least 3 elements long
    let mut candidate = hull.points.len() / 2;

    loop {
        let left = candidate - 1;
        let right = candidate + 1;

        // if moved to edge, this is the connection to the upper hull
        if candidate == 0 {
            return Ok(0);
        } else if candidate == hull.points.len() - 1 {
            return Ok(hull.points.len() - 1);
        }

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
            | (Side::Right, TurnType::Right, TurnType::Straight) => candidate = right,
            (Side::Right, TurnType::Straight, TurnType::Right) => {
                return Err(Errors::LowerHullError)
            }
        }
    }

    Ok(candidate)
}

#[cfg(test)]
mod test {
    use crate::grahams_scan_parallel::{get_tangent_from_point, grahams_scan_parallel};
    use crate::input_generation::{Curve, Line};
    use crate::types::{Point, PointVector};

    #[test]
    fn line_hull_2p() {
        let upper_hull: PointVector = grahams_scan_parallel(Line::get_input(50), 2);
        assert_eq!(
            upper_hull.points,
            vec![Point { x: 0, y: 0 }, Point { x: 49, y: 49 }]
        );
    }

    #[test]
    fn line_hull_6p() {
        let upper_hull: PointVector = grahams_scan_parallel(Line::get_input(50), 6);
        assert_eq!(
            upper_hull.points,
            vec![Point { x: 0, y: 0 }, Point { x: 49, y: 49 }]
        );
    }

    #[test]
    fn curve_hull_2p() {
        let upper_hull: PointVector = grahams_scan_parallel(Curve::get_input(50), 2);
        assert_eq!(upper_hull.points, Curve::get_input(50).points);
    }

    #[test]
    fn curve_hull_6p() {
        let upper_hull: PointVector = grahams_scan_parallel(Curve::get_input(50), 6);
        assert_eq!(upper_hull.points, Curve::get_input(50).points);
    }

    #[test]
    fn tangent_from_point() {
        let upper_hull: PointVector = PointVector { points: vec![
            Point { x: 6, y: 1 },
            Point { x: 8, y: 3 },
            Point { x: 11, y: 4 },
            Point { x: 12, y: 3 },
            Point { x: 13, y: 1 }]
        };
        let point: Point = Point { x: 1, y: 3 };
        assert_eq!(get_tangent_from_point(&upper_hull, point), Ok(2));
    }
}
