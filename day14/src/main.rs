use day14::{load_instructions, run_instructions, run_instructions_v2};

fn main() {
    let instructions = load_instructions("input.txt");

    let mem = run_instructions(&instructions);
    println!("Part 1: {}", mem.sum());

    let mem = run_instructions_v2(&instructions);
    println!("Part 2: {}", mem.sum());
}
