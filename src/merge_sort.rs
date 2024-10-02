use crate::merge::{parallel_merge, sequential_merge};

pub fn sequential_merge_sort(input: &mut [i64], scratch: &mut [i64]) {
    let len = input.len();
    if len == 1 {
        return;
    }
    let mid = len / 2;

    let (left_input, right_input) = input.split_at_mut(mid);
    let (left_scratch, right_scratch) = scratch.split_at_mut(mid);

    sequential_merge_sort(left_input, left_scratch);
    sequential_merge_sort(right_input, right_scratch);

    sequential_merge(left_scratch, right_scratch, input);
    scratch.copy_from_slice(input);
}

/** TASK 1 */
pub fn parallel_merge_sort<'input>(
    input: &'input mut [i64],
    scratch: &'input mut [i64],
    num_processors: usize,
) {
    let len = input.len();
    if len == 1 {
        return;
    }
    if num_processors < 2 {
        sequential_merge_sort(input, scratch);
    } else {
        let mid = len / 2;
        let left_processors = num_processors / 2;

        let (left_input, right_input) = input.split_at_mut(mid);
        let (left_scratch, right_scratch) = scratch.split_at_mut(mid);

        std::thread::scope(|scope| {
            scope.spawn(|| parallel_merge_sort(left_input, left_scratch, left_processors));
            parallel_merge_sort(right_input, right_scratch, num_processors - left_processors);
        });

        // merge in sequence
        sequential_merge(left_scratch, right_scratch, input);

        scratch.copy_from_slice(input);
    }
}

/** TASK 3 */
pub fn fully_parallel_merge_sort<'input>(
    input: &'input mut [i64],
    scratch: &'input mut [i64],
    num_processors: usize,
) {
    let len = input.len();
    if len == 1 {
        return;
    }
    if num_processors < 2 {
        sequential_merge_sort(input, scratch);
    } else {
        let mid = len / 2;
        let left_processors = num_processors / 2;

        let (left_input, right_input) = input.split_at_mut(mid);
        let (left_scratch, right_scratch) = scratch.split_at_mut(mid);

        std::thread::scope(|scope| {
            scope.spawn(|| parallel_merge_sort(left_input, left_scratch, left_processors));
            parallel_merge_sort(right_input, right_scratch, num_processors - left_processors);
        });

        // merge in parallel
        parallel_merge(left_scratch, right_scratch, input, num_processors);

        scratch.copy_from_slice(input);
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_sort::{fully_parallel_merge_sort, parallel_merge_sort};
    use crate::utils::is_sorted;

    const NUM_PROCESSORS_TEST: usize = 8;

    // Test parallel merge sort (using sequential merge)
    #[test]
    fn parallel_sort_sequential_merge_sorted() {
        let mut input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        parallel_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    #[test]
    fn parallel_sort_sequential_merge_reversed() {
        let mut input: Vec<i64> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        parallel_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    #[test]
    fn parallel_sort_sequential_merge_random() {
        let mut input: Vec<i64> = vec![5, 7, 2, 9, 1, 3, 8, 4, 6];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        parallel_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    // Test parallel merge sort (using parallel merge)
    #[test]
    fn parallel_sort_parallel_merge_sorted() {
        let mut input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        fully_parallel_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    #[test]
    fn parallel_sort_parallel_merge_reversed() {
        let mut input: Vec<i64> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        fully_parallel_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }

    #[test]
    fn parallel_sort_parallel_merge_random() {
        let mut input: Vec<i64> = vec![5, 7, 2, 9, 1, 3, 8, 4, 6];
        let mut scratch: Vec<i64> = vec![0i64; 9];
        fully_parallel_merge_sort(&mut input, &mut scratch, NUM_PROCESSORS_TEST);
        assert_eq!(is_sorted(scratch), true);
    }
}
