use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::Heading::{East, North, South, West};
use crate::Instruction::{E, F, L, N, R, S, W};

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    R(i32),
    L(i32),
    F(i32),
}

impl FromStr for Instruction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "N" => Ok(N(s[1..].parse().unwrap())),
            "S" => Ok(S(s[1..].parse().unwrap())),
            "E" => Ok(E(s[1..].parse().unwrap())),
            "W" => Ok(W(s[1..].parse().unwrap())),
            "R" => Ok(R(s[1..].parse().unwrap())),
            "L" => Ok(L(s[1..].parse().unwrap())),
            "F" => Ok(F(s[1..].parse().unwrap())),

            _ => Err(ParseErr {})

        }
    }
}

/// Loads instructions from the given file, panicking if it doesn't exist or can't be loaded.
fn load_instructions(filename: &str) -> Vec<Instruction> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(Ok(N(3)), "N3".parse());
        assert_eq!(Ok(S(10)), "S10".parse());
        assert_eq!(Ok(E(4)), "E4".parse());
        assert_eq!(Ok(W(2)), "W2".parse());
        assert_eq!(Ok(L(90)), "L90".parse());
        assert_eq!(Ok(R(180)), "R180".parse());
        assert_eq!(Ok(F(7)), "F7".parse());
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Heading {
    North, South, East, West,
}

impl Heading {
    /// Returns this heading as degrees.  North starts at 0, and degrees go clockwise (East is 90).
    fn to_degrees(&self) -> i32 {
        match self {
            North => 0,
            East => 90,
            South => 180,
            West => 270,
        }
    }

    /// Converts the given degrees into a heading.
    fn from_degrees(degrees: i32) -> Heading {
        match (360 + degrees) % 360 {
            0 => North,
            90 => East,
            180 => South,
            270 => West,

            _ => panic!("Invalid degrees: {}", degrees),
        }
    }
}

#[cfg(test)]
mod heading_tests {
    use crate::Heading::{North, South, West};

    use super::*;

    #[test]
    fn to_degrees() {
        assert_eq!(0, North.to_degrees());
        assert_eq!(90, East.to_degrees());
        assert_eq!(180, South.to_degrees());
        assert_eq!(270, West.to_degrees());
    }

    #[test]
    fn from_degrees() {
        assert_eq!(North, Heading::from_degrees(-360));
        assert_eq!(East, Heading::from_degrees(-270));
        assert_eq!(South, Heading::from_degrees(-180));
        assert_eq!(West, Heading::from_degrees(-90));

        assert_eq!(North, Heading::from_degrees(0));
        assert_eq!(East, Heading::from_degrees(90));
        assert_eq!(South, Heading::from_degrees(180));
        assert_eq!(West, Heading::from_degrees(270));

        assert_eq!(North, Heading::from_degrees(360));
        assert_eq!(East, Heading::from_degrees(450));
        assert_eq!(South, Heading::from_degrees(540));
        assert_eq!(West, Heading::from_degrees(630));
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Ship {
    heading: Heading,
    x: i32,
    y: i32,
}

impl Ship {
    /// Returns a new ship which starts at 0,0 facing East.
    fn new() -> Ship {
        Ship { heading: East, x: 0, y: 0 }
    }

    /// Runs this instruction, changing this ship in place
    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            N(distance) => self.y += distance,
            S(distance) => self.y -= distance,
            E(distance) => self.x += distance,
            W(distance) => self.x -= distance,

            R(degrees) =>
                self.heading = Heading::from_degrees(self.heading.to_degrees() + degrees),

            L(degrees) =>
                self.heading = Heading::from_degrees(self.heading.to_degrees() - degrees),

            F(distance) => {
                match self.heading {
                    North => self.run(&N(*distance)),
                    South => self.run(&S(*distance)),
                    East => self.run(&E(*distance)),
                    West => self.run(&W(*distance)),
                }
            }
        }
    }

    /// Returns the Manhattan distance between the ships starting position 0,0 and its current position.
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[cfg(test)]
mod ship_tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(0, Ship { heading: East, x: 0, y: 0 }.distance());
        assert_eq!(14, Ship { heading: East, x: 10, y: -4 }.distance());
    }

    #[test]
    fn run_example() {
        let instructions = load_instructions("sample.txt");
        let mut ship = Ship::new();

        for instruction in instructions {
            ship.run(&instruction);
        }

        assert_eq!(25, ship.distance());
    }

    #[test]
    fn run() {
        let mut ship = Ship::new();

        ship.run(&N(3));
        assert_eq!(Ship { heading: East, x: 0, y: 3 }, ship);

        ship.run(&S(10));
        assert_eq!(Ship { heading: East, x: 0, y: -7 }, ship);

        ship.run(&E(4));
        assert_eq!(Ship { heading: East, x: 4, y: -7 }, ship);

        ship.run(&W(2));
        assert_eq!(Ship { heading: East, x: 2, y: -7 }, ship);

        ship.run(&L(90));
        assert_eq!(Ship { heading: North, x: 2, y: -7 }, ship);

        ship.run(&R(180));
        assert_eq!(Ship { heading: South, x: 2, y: -7 }, ship);

        ship.run(&F(7));
        assert_eq!(Ship { heading: South, x: 2, y: -14 }, ship);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Waypoint {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct ShipWaypoint {
    ship: Ship,
    waypoint: Waypoint,
}

impl ShipWaypoint {
    /// Returns a new ship and waypoint.
    fn new() -> ShipWaypoint {
        ShipWaypoint { ship: Ship::new(), waypoint: Waypoint { x: 10, y: 1 }}
    }

    /// Runs the given instruction, modifying this ship and waypoint.
    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            N(distance) => self.waypoint.y += distance,
            S(distance) => self.waypoint.y -= distance,
            E(distance) => self.waypoint.x += distance,
            W(distance) => self.waypoint.x -= distance,

            L(90) | R(270) => {
                let new_y = self.waypoint.x;
                self.waypoint.x = -1 * self.waypoint.y;
                self.waypoint.y = new_y;
            }

            R(90) | L(270) => {
                let new_y = -1 * self.waypoint.x;
                self.waypoint.x = self.waypoint.y;
                self.waypoint.y = new_y;
            }

            R(180) | L(180) => {
                self.waypoint.x *= -1;
                self.waypoint.y *= -1;
            }

            F(times) => {
                self.ship.x += times * self.waypoint.x;
                self.ship.y += times * self.waypoint.y;
            }

            _ => panic!("Unknown instruction {:?}", instruction),
        }
    }
}

#[cfg(test)]
mod ship_waypoint_tests {
    use super::*;

    #[test]
    fn run_sample() {
        let mut ship_waypoint = ShipWaypoint::new();

        let instructions = load_instructions("sample.txt");
        for instruction in instructions {
            ship_waypoint.run(&instruction);
        }

        assert_eq!(286, ship_waypoint.ship.distance());
    }
}

fn main() {
    let instructions = load_instructions("input.txt");
    let mut ship = Ship::new();

    for instruction in &instructions {
        ship.run(instruction);
    }

    println!("Part 1: {}", ship.distance());
    
    let mut ship_waypoint = ShipWaypoint::new();

    for instruction in &instructions {
        ship_waypoint.run(instruction);
    }

    println!("Part 2: {}", ship_waypoint.ship.distance());
}
