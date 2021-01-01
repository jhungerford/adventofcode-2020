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
    /// NorthWest - SouthEast
    x: i32,
    /// NorthEast - SouthWest
    y: i32,
    /// East - West
    z: i32,
}

impl Tile {
    /// Returns the tile at the center of the grid.
    fn origin() -> Tile {
        Tile { x: 0, y: 0, z: 0 }
    }
}

impl Add<Direction> for Tile {
    type Output = Tile;

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
    pub fn new() -> Grid {
        Grid { black_tiles: HashSet::new() }
    }

    /// Runs all of the instructions, returning the modified grid.
    pub fn run_all(&mut self, instructions: &Vec<Instruction>) -> &Self {
        for instruction in instructions {
            self.run(instruction);
        }

        self
    }

    /// Runs the given instruction, which flips a tile located by following the directions
    /// from the tile at 0,0,0 in this grid.  Returns this grid, which has been modified.
    pub fn run(&mut self, instruction: &Instruction) -> &Self {
        let tile = instruction.directions.iter()
            .fold(Tile::origin(), |tile, direction| tile + *direction);

        if self.black_tiles.contains(&tile) {
            self.black_tiles.remove(&tile);
        } else {
            self.black_tiles.insert(tile);
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
        let mut grid = Grid::new();

        grid.run(&"esenee".parse().unwrap());
        assert!(grid.black_tiles.contains(&Tile { x: 3, y: -3, z: 0 }));

        grid.run(&"sesenwnenenewseeswwswswwnenewsewsw".parse().unwrap());
        assert!(grid.black_tiles.contains(&Tile { x: -3, y: 1, z: 2 }));
    }

    #[test]
    fn run_sample() {
        let instructions = load_instructions("sample.txt");
        assert_eq!(10, Grid::new().run_all(&instructions).num_black());
    }
}