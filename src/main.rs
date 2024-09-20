mod input_generation;

use input_generation::gen_input;

fn main() {
    println!("{:?}", gen_input(16));
}
