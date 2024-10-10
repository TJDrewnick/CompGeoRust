use rand::seq::SliceRandom;
use rand::thread_rng;
use std::iter::repeat_with;
use turborand::prelude::*;

type LeftRightSplit = (Vec<i64>, Vec<i64>);
pub type MergeFunction = fn(i64) -> LeftRightSplit;

#[allow(dead_code)]
pub fn gen_input(input_size: usize) -> Vec<i64> {
    let rand = Rng::new();

    repeat_with(|| rand.i64(-(input_size as i64)..(input_size as i64)))
        .take(input_size)
        .collect()
}

pub fn shuffled(input_size: i64) -> Vec<i64> {
    let mut shuffled: Vec<i64> = (0..input_size).collect();
    shuffled.shuffle(&mut thread_rng());
    shuffled
}

#[allow(dead_code)]
pub fn reverse_sorted(input_size: i64) -> Vec<i64> {
    (0..input_size).rev().collect()
}

#[allow(dead_code)]
pub fn all_left_smaller_than_right(input_size: i64) -> Vec<i64> {
    let vec_length = input_size / 2;
    let mut left: Vec<i64> = (0..vec_length).collect();
    let mut right: Vec<i64> = (vec_length..input_size).collect();
    left.shuffle(&mut thread_rng());
    right.shuffle(&mut thread_rng());
    left.extend(right);
    left
}

#[allow(dead_code)]
pub fn all_right_smaller_than_left(input_size: i64) -> Vec<i64> {
    let vec_length = input_size / 2;
    let mut left: Vec<i64> = (vec_length..input_size).collect();
    let mut right: Vec<i64> = (0..vec_length).collect();
    left.shuffle(&mut thread_rng());
    right.shuffle(&mut thread_rng());
    left.extend(right);
    left
}

#[allow(dead_code)]
pub fn right_fits_between_two_elements_in_left(input_size: i64) -> Vec<i64> {
    let mut left_first: Vec<i64> = (0..input_size / 4).collect();
    let left_second: Vec<i64> = ((3 * input_size / 4)..input_size).collect();
    left_first.extend(left_second);
    let right: Vec<i64> = ((input_size / 4)..(3 * input_size / 4)).collect();
    left_first.extend(right);
    left_first
}

pub fn alternating(input_size: i64) -> LeftRightSplit {
    assert_eq!(input_size % 2, 0);
    let left: Vec<i64> = (0..input_size).step_by(2).collect();
    let right: Vec<i64> = (1..input_size).step_by(2).collect();
    (left, right)
}

pub fn sorted(input_size: i64) -> LeftRightSplit {
    let sorted: Vec<i64> = (0..input_size).collect();
    let (left, right) = sorted.split_at((input_size / 2) as usize);
    (left.to_vec(), right.to_vec())
}

pub fn left_fits_between_last_two_elements_in_right(input_size: i64) -> LeftRightSplit {
    let vec_length = (input_size - 1) / 2;
    let left: Vec<i64> = (vec_length..input_size - 1).collect();
    let mut right: Vec<i64> = (0..vec_length).collect();
    right.push(input_size);
    (left, right)
}

pub fn random_sorted_halves(input_size: i64) -> LeftRightSplit {
    let mut input_vector: Vec<i64> = (0..input_size).collect();
    input_vector.shuffle(&mut thread_rng());
    let (left, right) = input_vector.split_at_mut((input_size / 2) as usize);
    left.sort();
    right.sort();

    (left.to_vec(), right.to_vec())
}

#[cfg(test)]
mod tests {
    use crate::input_generation::alternating;

    #[test]
    fn test_alternating() {
        let (mut left, right) = alternating(6);
        let expected: Vec<i64> = vec![0, 2, 4, 1, 3, 5];
        left.extend(right);
        assert_eq!(left, expected);
    }
}
