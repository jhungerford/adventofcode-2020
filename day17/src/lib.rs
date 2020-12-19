use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Grid {
    dimensions: usize,
    active: HashSet<Vec<i32>>,
}

impl Grid {
    /// Loads a Grid with the given number of dimensions from a file.
    pub fn load(filename: &str, dimensions: usize) -> Grid {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut active = HashSet::new();

        for (y, line) in f.lines().enumerate() {
            for (x, c) in line.unwrap().chars().enumerate() {
                if c == '#' {
                    let mut point = vec![x as i32, y as i32];

                    for _ in 2..dimensions {
                        point.push(0);
                    }

                    active.insert(point);
                }
            }
        }

        Grid { dimensions, active }
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
        let mut to_check: Vec<Vec<i32>> = Vec::new();
        for i in 0..self.dimensions {
            let min = self.active.iter().map(|a| a[i]).min().unwrap() - 1;
            let max = self.active.iter().map(|a| a[i]).max().unwrap() + 1;

            let mut new_to_check= Vec::new();

            for j in min ..= max {
                if to_check.is_empty() {
                    new_to_check.push(vec![j]);
                } else {
                    for partial_pos in &to_check {
                        let mut pos = partial_pos.clone();
                        pos.push(j);
                        new_to_check.push(pos);
                    }
                }
            }

            to_check = new_to_check;
        }

        let mut new_active = HashSet::new();

        for pos in to_check {
            let active = self.active.contains(&pos);
            let neighbors = self.neighbors(&pos);

            if active && (neighbors == 2 || neighbors == 3) {
                new_active.insert(pos);
            } else if !active && neighbors == 3 {
                new_active.insert(pos);
            }
        }

        self.active = new_active;
    }

    /// Returns the number of active neighbors around the given position.
    fn neighbors(&self, pos: &Vec<i32>) -> usize {
        let mut neighbors: Vec<Vec<i32>> = Vec::new();
        for i in 0..self.dimensions {
            let mut new_neighbors = Vec::new();

            for j in -1 ..= 1 {
                if neighbors.is_empty() {
                    new_neighbors.push(vec![pos[i] + j])
                } else {
                    for partial_neighbor in &neighbors {
                        let mut neighbor = partial_neighbor.clone();
                        neighbor.push(pos[i] + j);
                        new_neighbors.push(neighbor);
                    }
                }
            }

            neighbors = new_neighbors;
        }

        neighbors.iter()
            .filter(|&n| n != pos && self.active.contains(n))
            .count()
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
        let grid = Grid::load("sample.txt", 3);
        assert_eq!(grid.active(), 5);
    }

    #[test]
    fn run_sample() {
        let mut grid = Grid::load("sample.txt", 3);

        grid.step_times(6);

        assert_eq!(112, grid.active());
    }

    #[test]
    fn run_sample_4D() {
        let mut grid = Grid::load("sample.txt", 4);

        grid.step_times(6);

        assert_eq!(848, grid.active());
    }
}