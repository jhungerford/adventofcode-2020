use std::collections::HashSet;
use std::ops::Add;
use std::str::FromStr;

use crate::Direction::{E, NE, NW, SE, SW, W};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Eq, PartialEq)]
pub struct ParseErr {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction { E, SE, SW, W, NW, NE }

#[derive(Debug, Eq, PartialEq)]
pub struct Instruction {
    directions: Vec<Direction>
}

impl FromStr for Instruction {
    type Err = ParseErr;

    /// Parses a string into an instruction.  An instruction is made up of
    /// directions (se, sw, ne, nw, e, and w) without any separators.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        let mut directions = Vec::new();
        let mut i = 0;

        while i < chars.len() {
            let dir = match chars[i] {
                's' => {
                    i += 2;
                    match chars[i - 1] {
                        'e' => SE,
                        'w' => SW,
                        _ => return Err(ParseErr {}),
                    }
                }
                'n' => {
                    i += 2;
                    match chars[i - 1] {
                        'e' => NE,
                        'w' => NW,
                        _ => return Err(ParseErr {}),
                    }
                }
                'e' => {
                    i += 1;
                    E
                }
                'w' => {
                    i += 1;
                    W
                }
                _ => return Err(ParseErr {}),
            };

            directions.push(dir);
        }

        Ok(Instruction { directions })
    }
}

/// Loads a list of instructions from the given file.
pub fn load_instructions(filename: &str) -> Vec<Instruction> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

/// Tile uses cube coordinates to identify a tile - see: https://www.redblobgames.com/grids/hexagons/
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tile {
    x: i32,
    y: i32,
    z: i32,
}

impl Tile {
    /// Returns the tile at the center of the grid.
    fn origin() -> Tile {
        Tile { x: 0, y: 0, z: 0 }
    }

    /// Returns the tiles around this tile.
    fn neighbors(&self) -> Vec<Tile> {
        [E, SE, SW, W, NW, NE].iter().map(|dir| *self + *dir).collect()
    }
}

impl Add<Direction> for Tile {
    type Output = Tile;

    /// Adds the given direction to this tile.
    /// See: https://www.redblobgames.com/grids/hexagons#coordinates-cube
    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::E => Tile {
                x: self.x + 1,
                y: self.y - 1,
                z: self.z,
            },
            Direction::SE => Tile {
                x: self.x,
                y: self.y - 1,
                z: self.z + 1,
            },
            Direction::SW => Tile {
                x: self.x - 1,
                y: self.y,
                z: self.z + 1,
            },
            Direction::W => Tile {
                x: self.x - 1,
                y: self.y + 1,
                z: self.z,
            },
            Direction::NW => Tile {
                x: self.x,
                y: self.y + 1,
                z: self.z - 1,
            },
            Direction::NE => Tile {
                x: self.x + 1,
                y: self.y,
                z: self.z - 1,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    black_tiles: HashSet<Tile>
}

impl Grid {
    /// Returns a new grid.
    pub fn new(instructions: &Vec<Instruction>) -> Grid {
        let mut grid = Grid { black_tiles: HashSet::new() };

        grid.run_all(instructions);

        grid
    }

    /// Runs all of the instructions, returning the modified grid.
    fn run_all(&mut self, instructions: &Vec<Instruction>) -> &Self {
        for instruction in instructions {
            self.run(instruction);
        }

        self
    }

    /// Runs the given instruction, which flips a tile located by following the directions
    /// from the tile at 0,0,0 in this grid.  Returns this grid, which has been modified.
    fn run(&mut self, instruction: &Instruction) -> &Self {
        let tile = instruction.directions.iter()
            .fold(Tile::origin(), |tile, direction| tile + *direction);

        self.flip(tile);

        self
    }

    /// Flips the given tile - black becomes white, white becomes black.
    fn flip(&mut self, tile: Tile) {
        if self.black_tiles.contains(&tile) {
            self.black_tiles.remove(&tile);
        } else {
            self.black_tiles.insert(tile);
        }
    }

    /// Flips tiles according to rules, returning this modified grid.
    pub fn tick(&mut self) -> &Self {
        // Tiles are flipped simultaneously based on the following rules:
        // - Black tiles with zero or >2 adjacent black tiles are flipped to white.
        // - White tiles with 2 adjacent black tiles are flipped to black.

        // Consider the black tiles and their neighbors.
        let consider: HashSet<Tile> = self.black_tiles.iter().cloned()
            .chain(self.black_tiles.iter().flat_map(|tile| tile.neighbors()))
            .collect();

        // Figure out which ones to flip based on the rules.
        let flip: Vec<Tile> = consider.into_iter().filter(|tile| {
            let black_neighbors = tile.neighbors().into_iter()
                .filter(|neighbor| self.black_tiles.contains(neighbor))
                .count();

            if self.black_tiles.contains(tile) {
                // Currently a black tile - flip if 0 or >2 neighbors are black tiles.
                black_neighbors == 0 || black_neighbors > 2
            } else {
                // Currently a white tile - flip if exactly 2 neighbors are black tiles.
                black_neighbors == 2
            }
        }).collect();

        // Flip the tiles.
        for tile in flip {
            self.flip(tile);
        }

        self
    }

    /// Flips tiles according to rules the given number of times, returning the final grid.
    pub fn tick_times(&mut self, times: usize) -> &Self {
        for _ in 0..times {
            self.tick();
        }

        self
    }

    /// Returns the number of black tiles on this grid.
    pub fn num_black(&self) -> usize {
        self.black_tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction::{E, NE, NW, SE, SW, W};

    use super::*;

    #[test]
    fn parse_instruction() {
        let instruction = "sesenwnenenewseeswwswswwnenewsewsw".parse();
        let expected = Instruction {
            directions: vec![SE, SE, NW, NE, NE, NE, W, SE, E, SW, W, SW, SW, W, NE, NE, W, SE, W, SW]
        };

        assert_eq!(Ok(expected), instruction);
        assert_eq!(Instruction { directions: vec![E, SE, NE, E] }, "esenee".parse().unwrap());
    }

    #[test]
    fn tile_plus_direction() {
        assert_eq!(Tile { x: 0, y: 0, z: 0 }, Tile::origin());
        assert_eq!(Tile { x: 1, y: -1, z: 0 }, Tile::origin() + E);
        assert_eq!(Tile { x: 0, y: -1, z: 1 }, Tile::origin() + SE);
        assert_eq!(Tile { x: -1, y: 0, z: 1 }, Tile::origin() + SW);
        assert_eq!(Tile { x: -1, y: 1, z: 0 }, Tile::origin() + W);
        assert_eq!(Tile { x: 0, y: 1, z: -1 }, Tile::origin() + NW);
        assert_eq!(Tile { x: 1, y: 0, z: -1 }, Tile::origin() + NE);
    }

    #[test]
    fn run_instruction() {
        let instructions = vec![];
        let mut grid = Grid::new(&instructions);

        grid.run(&"esenee".parse().unwrap());
        assert!(grid.black_tiles.contains(&Tile { x: 3, y: -3, z: 0 }));

        grid.run(&"sesenwnenenewseeswwswswwnenewsewsw".parse().unwrap());
        assert!(grid.black_tiles.contains(&Tile { x: -3, y: 1, z: 2 }));
    }

    #[test]
    fn run_sample() {
        let instructions = load_instructions("sample.txt");
        assert_eq!(10, Grid::new(&instructions).num_black());
    }

    #[test]
    fn tick_sample() {
        let instructions = load_instructions("sample.txt");

        let mut first_grid = Grid::new(&instructions);
        assert_eq!(2208, first_grid.tick_times(100).num_black());

        let mut grid = Grid::new(&instructions);

        assert_eq!(15, grid.tick().num_black());
        assert_eq!(12, grid.tick().num_black());
        assert_eq!(25, grid.tick().num_black());
        assert_eq!(14, grid.tick().num_black());
        assert_eq!(23, grid.tick().num_black());
        assert_eq!(28, grid.tick().num_black());
        assert_eq!(41, grid.tick().num_black());
        assert_eq!(37, grid.tick().num_black());
        assert_eq!(49, grid.tick().num_black());
        assert_eq!(37, grid.tick().num_black());

        assert_eq!(132, grid.tick_times(10).num_black());
        assert_eq!(259, grid.tick_times(10).num_black());
        assert_eq!(406, grid.tick_times(10).num_black());
        assert_eq!(566, grid.tick_times(10).num_black());
        assert_eq!(788, grid.tick_times(10).num_black());
        assert_eq!(1106, grid.tick_times(10).num_black());
        assert_eq!(1373, grid.tick_times(10).num_black());
        assert_eq!(1844, grid.tick_times(10).num_black());
        assert_eq!(2208, grid.tick_times(10).num_black());
    }
}