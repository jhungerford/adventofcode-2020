use day24::{load_instructions, Grid};

fn main() {
    let instructions = load_instructions("input.txt");

    println!("Part 1: {}", Grid::new(&instructions).num_black());
    println!("Part 2: {}", Grid::new(&instructions).tick_times(100).num_black());
}
