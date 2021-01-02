use day19::Puzzle;

fn main() {
    let puzzle = Puzzle::load("input.txt");

    println!("Part 1: {}", puzzle.matches(0));
    println!("Part 2: {}", puzzle.recursive_matches());
}
