
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
pub fn seg_merge_with_seq(left: &[i64], right: &[i64], output: &mut [i64], num_processors: usize) {

    // split right into size log(num_processors) pieces
    
    
    // binary search for upper bound of each piece - if looking at right[i] place in output array on spot i + rank(right[i], left)
    
    

}