use crate::utils::binary_search;
use std::cmp::min;
use std::iter::zip;

pub fn sequential_merge(left: &[i64], right: &[i64], output: &mut [i64]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            output[k] = left[i];
            i += 1;
        } else {
            output[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        output[k] = left[i];
        k += 1;
        i += 1;
    }

    while j < right.len() {
        output[k] = right[j];
        k += 1;
        j += 1;
    }
}

/** TASK 2 */
pub fn parallel_merge(left: &[i64], right: &[i64], output: &mut [i64], num_processors: usize) {
    let n = right.len();

    // if there are fewer elements in right array than the number of processors available, use only right.len() processors
    let threads = min(num_processors, n);

    // allocate array R[0,...,p] and R[0] = 0
    let mut rank_vector = vec![0usize; threads + 1];
    let rank_chunks = rank_vector.chunks_mut(1);
    let chunk_size = (n as f64 / (threads as f64)).ceil() as usize;

    // binary search for upper bound of each piece - if looking at right[i] place in output array on spot i + rank(right[i], left)
    // get rank
    std::thread::scope(|scope| {
        for (i, rank) in zip(0..threads + 1, rank_chunks) {
            if i == 0 {
                // R[0] = 0
                continue;
            } else if i == threads {
                // find rank of last element in right - this is the last upper bound
                rank[0] = binary_search(left, right[n - 1]);
            } else {
                scope.spawn(move || {
                    rank[0] = binary_search(left, right[i * chunk_size]);
                });
            }
        }
    });

    // if any elements remain in left, that are larger than all elements in right, put them in output
    let last_rank = rank_vector[rank_vector.len() - 1];
    if last_rank < left.len() {
        let output_len = output.len();
        output[output_len - left[last_rank..].len()..].copy_from_slice(&left[last_rank..]);
    }

    let right_chunks = right.chunks(chunk_size);
    // merge each chunk in sequentially ()
    std::thread::scope(|scope| {
        let mut current_chunk;
        let mut rest = output;

        for (i, right_chunk) in zip(0..threads, right_chunks) {
            // split remaining chunk of the output into chunk corresponding to the elements of left
            // and right to me merged now (using this approach so rust knows that the slices of output do not overlap)

            if i == threads - 1 {
                let left_slice = &left[rank_vector[i]..];
                sequential_merge(left_slice, right_chunk, rest);
            } else {
                (current_chunk, rest) =
                    rest.split_at_mut(chunk_size + rank_vector[i + 1] - rank_vector[i]);
                let left_slice = &left[rank_vector[i]..rank_vector[i + 1]];
                // let output_slice = &mut full_chunks[i * chunk_size + rank_vector[i]..  (i+1) * chunk_size + rank_vector[i+1]];
                scope.spawn(move || {
                    sequential_merge(left_slice, right_chunk, current_chunk);
                });
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::merge::{parallel_merge, sequential_merge};
    use crate::utils::is_equal_vec;
    use crate::NUM_PROCESSORS_TEST;

    // Test sequential merge
    #[test]
    fn sequential_merge_left_smaller() {
        let left: Vec<i64> = vec![1, 2, 3, 4];
        let right: Vec<i64> = vec![5, 6, 7, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn sequential_merge_right_smaller() {
        let left: Vec<i64> = vec![5, 6, 7, 8];
        let right: Vec<i64> = vec![1, 2, 3, 4];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn sequential_merge_alternating() {
        let left: Vec<i64> = vec![1, 3, 5, 7];
        let right: Vec<i64> = vec![2, 4, 6, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn sequential_merge_alternating_large() {
        let left: Vec<i64> = (0..1000).step_by(2).collect();
        let right: Vec<i64> = (1..1000).step_by(2).collect();
        let solution: Vec<i64> = (0..1000).collect();
        let mut output: Vec<i64> = vec![0i64; 1000];
        sequential_merge(&left, &right, &mut output);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    // Test parallel merge
    #[test]
    fn parallel_merge_left_smaller() {
        let left: Vec<i64> = vec![1, 2, 3, 4];
        let right: Vec<i64> = vec![5, 6, 7, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        parallel_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn parallel_merge_right_smaller() {
        let left: Vec<i64> = vec![5, 6, 7, 8];
        let right: Vec<i64> = vec![1, 2, 3, 4];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        parallel_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn parallel_merge_alternating() {
        let left: Vec<i64> = vec![1, 3, 5, 7];
        let right: Vec<i64> = vec![2, 4, 6, 8];
        let solution: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut output: Vec<i64> = vec![0i64; 8];
        parallel_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        assert_eq!(is_equal_vec(output, solution), true);
    }

    #[test]
    fn parallel_merge_alternating_large() {
        let left: Vec<i64> = (0..1000).step_by(2).collect();
        let right: Vec<i64> = (1..1000).step_by(2).collect();
        let solution: Vec<i64> = (0..1000).collect();
        let mut output: Vec<i64> = vec![0i64; 1000];
        parallel_merge(&left, &right, &mut output, NUM_PROCESSORS_TEST);
        assert_eq!(is_equal_vec(output, solution), true);
    }
}
