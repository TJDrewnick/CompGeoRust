use turborand::prelude::*;
use std::iter::repeat_with;

pub fn gen_input(input_size: usize) -> Vec<i64>{
    
    let rand = Rng::new();

    repeat_with(|| rand.i64(-(input_size as i64)..(input_size as i64))).take(input_size).collect()
}

// TODO new cases like: sorted