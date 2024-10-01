use std::iter::zip;

/**
* TASK 1
*/
fn sequential_merge(left: &[i64], right: &[i64], output: &mut [i64]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            output[k] = left[i];
            i = i + 1;
        } else {
            output[k] = right[j];
            j = j + 1;
        }
        k = k + 1;
    }

    while i < left.len() {
        output[k] = left[i];
        k = k + 1;
        i = i + 1;
    }

    while j < right.len() {
        output[k] = right[j];
        k = k + 1;
        j = j + 1;
    }
}

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

pub fn par_merge_sort<'input>(input: &'input mut [i64], scratch: &'input mut [i64], num_processors: usize) {
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
            scope.spawn(|| par_merge_sort(left_input, left_scratch, left_processors));
            par_merge_sort(right_input, right_scratch, num_processors - left_processors);
        });

        // merge in sequence
        sequential_merge(left_scratch, right_scratch, input);
        scratch.copy_from_slice(input);
    }
}


/**
* TASK 2
*/
pub fn par_merge(left: &[i64], right: & [i64], output: &mut [i64], num_processors: usize) {

    // allocate array R[0,...,p] and R[0] = 0
    let mut rank_vector = vec![0usize; num_processors + 1];
    let rank_chunks = rank_vector.chunks_mut(1);

    // binary search for upper bound of each piece - if looking at right[i] place in output array on spot i + rank(right[i], left)
    // get rank
    let n = right.len();
    let chunk_size = (n as f64 / (num_processors as f64)).ceil() as usize;

    std::thread::scope(|scope| {
        for (i, rank) in zip(0..num_processors + 1, rank_chunks) {
            if i == 0 {
                // R[0] = 0
                continue
            } else if i == num_processors - 1 {
                rank[0] = binary_search(left, right[(i-1) * chunk_size]);
            } else {
                scope.spawn(move || {
                    rank[0] = binary_search(left, right[(i-1) * chunk_size]);
                });
            }
        }
    });

    let right_chunks = right.chunks(chunk_size);
    // merge each chunk in sequentially ()
    std::thread::scope(|scope| {
        for (i, right_chunk) in zip(0..num_processors, right_chunks) {
            if i == num_processors - 1 {
                let left_slice = &left[rank_vector[i]..];
                let output_slice = &output[ + ];
                sequential_merge(left_slice, right_chunk, output_slice)
            } else {
                let left_slice = &left[rank_vector[i]..rank_vector[i+1]];
                let output_slice = &output[i * chunk_size + rank_vector[i]..  (i+1) * chunk_size + rank_vector[i+1]];
                scope.spawn(|| {
                    sequential_merge(left_slice, right_chunk, output_slice)
                });
            }
        }
    });
    
    
    // TODO: if any elements remain in left, that are larger than all elements in right, put them in output
    
}


pub fn binary_search(input: &[i64], key: i64) -> usize {
    let (mut low, mut high) = (0, input.len() - 1);

    while low < high {
        let mid = (low + high) / 2;
        if key <= input[mid] {
            high = mid
        } else {
            low = mid + 1
        }
    }
    high
}