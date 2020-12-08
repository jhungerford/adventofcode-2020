use day8::{Computer, find_terminating_computer};

fn main() {
    let mut part1_comp = Computer::load("input.txt");
    let part2_comp = Computer::load("input.txt");

    part1_comp.run_until_loop();

    println!("Part 1: {}", part1_comp.acc);
    println!("Part 2: {}", find_terminating_computer(&part2_comp));
}
