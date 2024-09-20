mod input_generation;
mod merge_sort;

use std::env;
use input_generation::gen_input;
use crate::merge_sort::merge_sort;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num_processors: usize = usize::from(&args[1]);
    let input_size: usize = usize::from(&args[2]);

    let input = gen_input(input_size);

    merge_sort(num_processors, input);
}
