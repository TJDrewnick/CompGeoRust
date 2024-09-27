mod input_generation;
mod merge_sort;

use std::env;
use std::time::Instant;
use input_generation::gen_input;
use crate::merge_sort::par_merge_sort;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_size: usize = args[1].parse().unwrap();
    let num_processors: usize = args[2].parse().unwrap(); 

    let mut input: Vec<i64> = gen_input(input_size);
    let mut scratch: Vec<i64> = input.clone();
    
    let now = Instant::now();
    println!("Input {:?}", input);
    par_merge_sort(&mut input, &mut scratch, num_processors);
    println!("Sorted {:?}", input);  
    let elapsed = now.elapsed();
    println!("Time: {:.3?}", elapsed);
    println!("correct: {:?}", is_sorted(scratch));
}


fn is_sorted(vec: Vec<i64>) -> bool {
    for i in 0..vec.len() - 1{
        if vec[i] > vec[i + 1] {
            return false
        }
    }
    true
}