pub fn par_merge_sort<'input>(input: &'input mut [i64], scratch: &'input mut [i64]) {
    let len = input.len();
    if len == 1 {
        return;
    }
    let mid = len / 2;
    
    let (left_input, right_input) = input.split_at_mut(mid);
    let (left_scratch, right_scratch) = scratch.split_at_mut(mid);
    
    std::thread::scope(|scope| {
        scope.spawn(|| par_merge_sort(left_input, left_scratch));
        scope.spawn(|| par_merge_sort(right_input, right_scratch));
    });
    
    
    // merge in sequence
    let left_len = mid;
    let right_len = len - mid;
    let (mut i, mut j, mut k) = (0, 0, 0);
    
    while i < left_len && j < right_len {
        if left_scratch[i] < right_scratch[j] {
            input[k] = left_scratch[i];
            i = i + 1;
        } else {
            input[k] = right_scratch[j];
            j = j + 1;
        }
        k = k + 1;
    }
    
    while i < left_len {
        input[k] = left_scratch[i];
        k = k + 1;
        i = i + 1;
    }

    while j < right_len {
        input[k] = right_scratch[j];
        k = k + 1;
        j = j + 1;
    }
    
    // might be a big slowdown -> try to find a more efficient solution or return input and scratch reversed like in pseudocode
    for l in 0..len {
        scratch[l] = input[l]
    }
}
