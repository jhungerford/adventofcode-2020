use day11::Grid;

fn main() {
    let mut grid = Grid::load("input.txt");
    grid.adjacent_tick_until_stable();
    println!("Part 1: {}", grid.num_occupied());

    let mut grid = Grid::load("input.txt");
    grid.visible_tick_until_stable();
    println!("Part 2: {}", grid.num_occupied());
}
