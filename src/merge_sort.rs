
/**
* TASK 1
*/
fn merge(left: &[i64], right: &[i64], output: &mut [i64]) {
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

    merge(left_scratch, right_scratch, input);
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
        merge(left_scratch, right_scratch, input);
        scratch.copy_from_slice(input);
    }
}


/**
* TASK 2
*/
pub fn par_merge(left: &[i64], right: &[i64], output: &mut [i64], num_processors: usize) {
    
    // allocate array R[0,...,p] and R[0] = 0
    let mut rank_vector = vec![0i64; num_processors];
    let rank_chunks = rank_vector.chunks_mut(1);
    
    // binary search for upper bound of each piece - if looking at right[i] place in output array on spot i + rank(right[i], left)
    // get rank
    std::thread::scope(|scope| {
        for rank in rank_chunks {
                scope.spawn(|| {
                rank[0] = 1;/*TODO get rank: left, right[(i*n)/num_processors]*/
            });
        }
        /*TODO get rank: left, right[(i*n)/num_processors]*/
    });
    
    // merge each chunk in sequentially ()
    std::thread::scope(|scope| {
        for i in 1..num_processors {
            scope.spawn(|| {
                /* TODO indices: merge(left, right, output)*/
            });
        }
        /* TODO indices: merge(left, right, output)*/
    });
    

}