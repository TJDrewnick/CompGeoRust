mod input_generation;
mod merge_sort;

use std::env;
use std::time::Instant;
use input_generation::gen_input;
use crate::merge_sort::{par_merge_sort, par_merge};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_size: usize = args[1].parse().unwrap();
    let num_processors: usize = args[2].parse().unwrap();

    let mut input: Vec<i64> = gen_input(input_size);
    let mut scratch: Vec<i64> = input.clone();

    let now = Instant::now();
    par_merge_sort(&mut input, &mut scratch, num_processors);
    let elapsed = now.elapsed();

    println!("Time: {:.3?}", elapsed);
    println!("correct: {:?}", is_sorted(scratch));
}


fn is_sorted(vec: Vec<i64>) -> bool {
    for i in 0..vec.len() - 1{
        if vec[i] > vec[i + 1] {
            return false
        }
    }
    true
}

fn is_equal_vec(vec1: Vec<i64>, vec2: Vec<i64>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use crate::merge_sort::{binary_search, sequential_merge};
    use super::*;

    const NUM_PROCESSORS_TEST: usize = 8;

    // Test sequential merge
    #[test]
    fn merge_sequential_left_smaller() {
        let left: Vec<i64> = vec![1, 2, 3, 4];
        let right: Vec<i64> = vec![5, 6, 7, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn merge_sequential_right_smaller() {
        let left: Vec<i64> = vec![5, 6, 7, 8];
        let right: Vec<i64> = vec![1, 2, 3, 4];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn merge_sequential_alternating() {
        let left: Vec<i64> = vec![1, 3, 5, 7];
        let right: Vec<i64> = vec![2, 4, 6, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn merge_sequential_alternating_large() {
        let left: Vec<i64> = (0..1000).step_by(2).collect();
        let right: Vec<i64> = (1..1000).step_by(2).collect();
        let solution: Vec<i64> = (0..1000).collect();
        let mut output: Vec<i64> = vec![0i64; 1000];
        sequential_merge(&left, &right, &mut output);
        eprintln!("{:?}", output);
        assert_eq!(is_equal_vec(output, solution), true);
    }


    // Test parallel merge sort (using sequential merge)
    #[test]
    fn sort_parallel_sorted() {
        let mut input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        par_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    #[test]
    fn sort_parallel_reversed() {
        let mut input: Vec<i64> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        par_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }


    // Test parallel merge
    #[test]
    fn sort_parallel_random() {
        let mut input: Vec<i64> = vec![5, 7, 2, 9, 1, 3, 8, 4, 6];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        par_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    #[test]
    fn merge_parallel_left_smaller() {
        let left: Vec<i64> = vec![1, 2, 3, 4];
        let right: Vec<i64> = vec![5, 6, 7, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        par_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn merge_parallel_right_smaller() {
        let left: Vec<i64> = vec![5, 6, 7, 8];
        let right: Vec<i64> = vec![1, 2, 3, 4];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        par_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn merge_parallel_alternating() {
        let left: Vec<i64> = vec![1, 3, 5, 7];
        let right: Vec<i64> = vec![2, 4, 6, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        par_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        eprintln!("{:?}", output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn merge_parallel_alternating_large() {
        let left: Vec<i64> = (0..1000).step_by(2).collect();
        let right: Vec<i64> = (1..1000).step_by(2).collect();
        let solution: Vec<i64> = (0..1000).collect();
        let mut output: Vec<i64> = vec![0i64; 1000];
        par_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        eprintln!("{:?}", output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    // 0 even
    // 1-249 uneven
    // 2-250 even
    // 251-499 uneven
    // 252-500 even
    // 501-749 uneven
    // 502-750 even
    // 751-999 uneven
    // 752-998 even

    
    // Test binary search
    #[test]
    fn binary_search_element_missing() {
        let input: Vec<i64> = vec![1, 2, 3, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 4);
        assert_eq!(Err(index), input.binary_search(&4));
    }

    #[test]
    fn binary_search_middle() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 4);
        assert_eq!(Ok(index), input.binary_search(&4));
    }

    #[test]
    fn binary_search_right_edge() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 9);
        assert_eq!(Ok(index), input.binary_search(&9));
    }

    #[test]
    fn binary_search_right_edge_missing() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = binary_search(&*input, 9);
        assert_eq!(Err(index), input.binary_search(&9));
    }

    #[test]
    fn binary_search_left_edge() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 1);
        assert_eq!(Ok(index), input.binary_search(&1));
    }

    #[test]
    fn binary_search_left_edge_missing() {
        let input: Vec<i64> = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 1);
        assert_eq!(Err(index), input.binary_search(&1));
    }
}