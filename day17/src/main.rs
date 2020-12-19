use day17::Grid;

fn main() {
    let mut grid = Grid::load("input.txt", 3);
    grid.step_times(6);
    println!("Part 1: {}", grid.active());

    grid = Grid::load("input.txt", 4);
    grid.step_times(6);
    println!("Part 2: {}", grid.active());
}
