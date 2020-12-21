use day18::sum_expressions;
use day18::Mode::{LeftToRight, AddBeforeTimes};

fn main() {
    println!("Part 1: {}", sum_expressions("input.txt", LeftToRight));
    println!("Part 2: {}", sum_expressions("input.txt", AddBeforeTimes));
}
