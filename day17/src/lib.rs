use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

pub struct Grid {
    active: HashSet<Position>
}

impl Grid {
    /// Loads a Grid from the given file.
    pub fn load(filename: &str) -> Grid {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut active = HashSet::new();

        for (y, line) in f.lines().enumerate() {
            for (x, c) in line.unwrap().chars().enumerate() {
                if c == '#' {
                    active.insert(Position { x: x as i32, y: y as i32, z: 0 });
                }
            }
        }

        Grid { active }
    }

    /// Runs this grid a given number of cycles, modifying it in the process.
    pub fn step_times(&mut self, times: usize) {
        (0..times).for_each(|_i| self.step());
    }

    /// Advances this grid by one step.
    pub fn step(&mut self) {
        // All cubes simultaneously change state by considering their immediate neighbors:
        // * If a cube is active and exactly 2 or 3 neighbors are active, the cube remains active.
        //   Otherwise it becomes inactive.
        // * If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.

        let min_position = self.active.iter().fold(
            Position { x: 0, y: 0, z: 0 }, |min, p| Position {
                x: min.x.min(p.x),
                y: min.y.min(p.y),
                z: min.z.min(p.z),
            });

        let max_position = self.active.iter().fold(
            Position { x: 0, y: 0, z: 0 }, |max, p| Position {
                x: max.x.max(p.x),
                y: max.y.max(p.y),
                z: max.z.max(p.z),
            });

        let mut new_active = HashSet::new();
        for z in min_position.z - 1 ..= max_position.z + 1 {
            for y in min_position.y - 1 ..= max_position.y + 1 {
                for x in min_position.x - 1 ..= max_position.x + 1 {
                    let pos = Position { x, y, z };
                    let active = self.active.contains(&pos);

                    let neighbors = self.active_neighbors(&pos);

                    if active && (neighbors == 2 || neighbors == 3) {
                        new_active.insert(pos);
                    } else if !active && neighbors == 3 {
                        new_active.insert(pos);
                    }
                }
            }
        }

        self.active = new_active;
    }

    /// Returns the number of active neighbors around the given position.
    fn active_neighbors(&self, p: &Position) -> usize {
        let mut num = 0;

        for z in -1 ..= 1 {
            for y in -1 ..= 1 {
                for x in -1 ..= 1 {
                    let n_pos = Position {
                        x: p.x + x,
                        y: p.y + y,
                        z: p.z + z,
                    };

                    if (x != 0 || y != 0 || z != 0) && self.active.contains(&n_pos) {
                        num += 1;
                    }
                }
            }
        }

        num
    }

    /// Returns the number of active cubes in this grid.
    pub fn active(&self) -> usize {
        self.active.len()
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn load() {
        let grid = Grid::load("sample.txt");
        assert_eq!(grid.active(), 5);
    }

    #[test]
    fn run_sample() {
        let mut grid = Grid::load("sample.txt");

        grid.step_times(6);

        assert_eq!(112, grid.active());
    }
}