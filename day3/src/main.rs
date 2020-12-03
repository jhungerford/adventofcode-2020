use std::str::FromStr;
use crate::Square::{Empty, Tree};
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Square {
    Empty, Tree
}

impl FromStr for Square {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Empty),
            "#" => Ok(Tree),
            _ => Err(ParseErr {}),
        }
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(".".parse(), Ok(Empty));
        assert_eq!("#".parse(), Ok(Tree));
        assert_eq!("...".parse::<Square>(), Err(ParseErr {}));
    }
}

struct Grid {
    width: usize,
    height: usize,
    squares: Vec<Vec<Square>>
}

impl Grid {

    /// Loads a grid from the given file, panicking if the grid is invalid or the file doesn't exist.
    fn load(filename: &str) -> Grid {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let squares: Vec<Vec<Square>> = f.lines()
            .map(|line_result| {
                let line = line_result.unwrap();
                (0..line.len())
                    .map(|i| line[i..i + 1].parse::<Square>().unwrap())
                    .collect()})
            .collect();

        Grid {
            width: squares[0].len(),
            height: squares.len(),
            squares,
        }
    }

    /// Returns the value of the square at the given position.  The grid is infinitely wide, so
    /// x may be greater than the grid's width.
    fn get(&self, x: usize, y: usize) -> Square {
        self.squares[y][x % self.width]
    }

    /// Returns the number of trees encountered starting at the top left corner of this grid
    /// and sliding right and down.
    fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut count = 0;

        while y < self.height {
            if self.get(x, y) == Tree {
                count += 1;
            }

            x += right;
            y += down;
        }

        count
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn load() {
        let grid = Grid::load("sample.txt");

        assert_eq!(11, grid.squares.len());
        assert_eq!(11, grid.squares[0].len());

        assert_eq!(11, grid.width);
        assert_eq!(11, grid.height);

        assert_eq!(vec![Empty, Empty, Tree, Tree, Empty, Empty, Empty, Empty, Empty, Empty, Empty], grid.squares[0]);
    }

    #[test]
    fn get() {
        let grid = Grid::load("sample.txt");

        assert_eq!(Empty, grid.get(0, 0));
        assert_eq!(Tree, grid.get(11, 1));
    }

    #[test]
    fn count_trees() {
        let grid = Grid::load("sample.txt");

        assert_eq!(7, grid.count_trees(3, 1));
    }
}

fn main() {
    let grid = Grid::load("input.txt");

    println!("Part 1: {}", grid.count_trees(3, 1));

    // In part 2, find the product of the number of trees encountered in several slopes.
    let part2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|(right, down)| grid.count_trees(*right, *down))
        .fold(1, |product, value| product * value);

    println!("Part 2: {}", part2);
}
