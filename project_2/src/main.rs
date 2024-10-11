use crate::gift_wrapping::gift_wrapping_upper_hull;
use crate::input_generation::Line;

mod gift_wrapping;
mod grahams_scan;
mod input_generation;
mod types;
mod utils;

fn main() {
    gift_wrapping_upper_hull(Line::get_input(10));
}
