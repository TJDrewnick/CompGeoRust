use std::cmp::min;
use std::iter::zip;

/**
* TASK 1
*/
pub fn sequential_merge(left: &[i64], right: &[i64], output: &mut [i64]) {
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
        // merge in parallel
        //par_merge(left_scratch, right_scratch, input, num_processors);

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

    // if there are fewer elements in right array than the number of processors available, use only right.len() processors
    let threads = min(num_processors, right.len());

    std::thread::scope(|scope| {
        for (i, rank) in zip(0..threads, rank_chunks) {
            
                println!("loop 1");
                if i == 0 {
                    // R[0] = 0
                    continue
                } else if i == threads - 1 {
                    rank[0] = binary_search(left, right[(i-1) * chunk_size]);
                } else {
                    scope.spawn(move || {
                        rank[0] = binary_search(left, right[(i-1) * chunk_size]);
                    });
                }
        }
    });

    // if any elements remain in left, that are larger than all elements in right, put them in output
    let last_rank = rank_vector[threads];
    if last_rank < left.len() {
        let len = (&output).len();
        output[len - left.len()..].copy_from_slice(&left[last_rank..]);
    }

    let right_chunks = right.chunks(chunk_size);
    // merge each chunk in sequentially ()
    std::thread::scope(|scope| {

        let mut current_chunk;
        let mut rest= output;

        for (i, right_chunk) in zip(0..threads, right_chunks) {

            // split remaining chunk of the output into chunk corresponding to the elements of left
            // and right to me merged now (using this approach so rust knows that the slices of output do not overlap)
            println!("hi I am a loop");
            (current_chunk, rest) = rest.split_at_mut(chunk_size + rank_vector[i+1] - rank_vector[i]);



            if i == threads 
            {   
                println!("sup");
                let left_slice = &left[rank_vector[i]..];
                sequential_merge(left_slice, right_chunk, rest);
            } else {
                let left_slice = &left[rank_vector[i]..rank_vector[i+1]];
                // let output_slice = &mut full_chunks[i * chunk_size + rank_vector[i]..  (i+1) * chunk_size + rank_vector[i+1]];
                scope.spawn(|| {
                    sequential_merge(left_slice, right_chunk, current_chunk);
                });
            }
        }
    });
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