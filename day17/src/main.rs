use day17::Grid;

fn main() {
    let mut grid = Grid::load("input.txt");

    grid.step_times(6);

    println!("Part 1: {}", grid.active());
}
