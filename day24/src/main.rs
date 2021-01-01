use day24::{load_instructions, Grid};

fn main() {
    let instructions = load_instructions("input.txt");

    println!("Part 1: {}", Grid::new().run_all(&instructions).num_black());

}
