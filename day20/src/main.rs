use day20::Tiles;

fn main() {
    let puzzle = Tiles::load("input.txt");

    println!("Part 1: {}", puzzle.corners());
}
