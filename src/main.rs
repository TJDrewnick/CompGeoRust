mod input_generation;
mod merge;
mod merge_sort;
mod utils;

use crate::merge_sort::{fully_parallel_merge_sort, parallel_merge_sort, sequential_merge_sort};
use crate::utils::is_sorted;
use input_generation::gen_input;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_size: usize = args[1].parse().unwrap();
    let num_processors: usize = args[2].parse().unwrap();
    let strategy = args[3].clone();

    let mut input: Vec<i64> = gen_input(input_size);
    let mut scratch: Vec<i64> = input.clone();

    let now = Instant::now();
    match strategy.as_str() {
        "SSSM" => {
            sequential_merge_sort(&mut input, &mut scratch);
        }
        "PSSM" => {
            parallel_merge_sort(&mut input, &mut scratch, num_processors);
        }
        "PSPM" => {
            fully_parallel_merge_sort(&mut input, &mut scratch, num_processors);
        }
        _ => {
            println!("Unknown strategy: {}", strategy);
            println!("Know strategies are: SSSM (sequential sort sequential merge), PSSM (parallel sort sequential merge), PSPM (parallel sort parallel merge)");
        }
    }

    let elapsed = now.elapsed();

    println!("Time: {:.3?}", elapsed);
    println!("correct: {:?}", is_sorted(scratch));
}
