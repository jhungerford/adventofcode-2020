use core::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use crate::Seat::{Empty, Floor, Occupied};

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl FromStr for Seat {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Empty),
            "#" => Ok(Occupied),
            "." => Ok(Floor),
            _ => Err(ParseErr{}),
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Empty => "L",
            Occupied => "#",
            Floor => ".",
        };

        write!(f, "{}", symbol)
    }
}

#[cfg(test)]
mod seat_tests {
    use super::*;

    #[test]
    fn parse_seat() {
        assert_eq!(Ok(Empty), "L".parse());
        assert_eq!(Ok(Occupied), "#".parse());
        assert_eq!(Ok(Floor), ".".parse());
        assert_eq!(Err(ParseErr {}), "invalid".parse::<Seat>());
    }
}

#[derive(Eq, PartialEq)]
pub struct Grid {
    seats: Vec<Vec<Seat>>
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.seats.clone() {
            for seat in line {
                write!(f, "{}", seat)?;
            }

            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Grid {
    /// Loads a grid from the given file.
    pub fn load(filename: &str) -> Grid {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let seats: Vec<Vec<Seat>> = f.lines()
            .map(|line| line.unwrap().chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();

        Grid { seats }
    }

    /// Applies seating rules to the grid, advancing it one round.  Returns the number
    /// of seats that changed.
    fn tick_adjacent(&mut self) -> usize {
        let mut new_seats = self.seats.clone();
        let mut num_changed = 0;

        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                let occupied = self.adjacent(row, col);
                let current_seat = self.seats[row][col];

                // Rules:
                // * Seat becomes occupied if it's empty and there are no adjacent occupied seats
                // * Seat becomes empty if it's occupied and there are 4+ occupied seats.
                // * Seat state does not change otherwise.
                let new_seat = match current_seat {
                    Empty if occupied == 0 => Occupied,
                    Occupied if occupied >= 4 => Empty,
                    _ => current_seat,
                };

                if new_seat != current_seat {
                    num_changed += 1;
                    new_seats[row][col] = new_seat;
                }
            }
        }

        self.seats = new_seats;

        num_changed
    }

    fn adjacent(&self, row: usize, col: usize) -> usize {
        let lower_row = row.checked_sub(1).unwrap_or(row);
        let upper_row = usize::min(row + 1, self.seats.len() - 1);

        let lower_col = col.checked_sub(1).unwrap_or(col);
        let upper_col = usize::min(col + 1, self.seats[row].len() - 1);

        let mut occupied = 0;
        for r in lower_row ..= upper_row {
            for c in lower_col ..= upper_col {
                if r != row || c != col {
                    if self.seats[r][c] == Occupied {
                        occupied += 1;
                    }
                }
            }
        }

        occupied
    }

    /// Applies adjacent seating rules to the grid repeatedly until no more seats change state.
    pub fn adjacent_tick_until_stable(&mut self) {
        while self.tick_adjacent() > 0 {}
    }

    /// Applies visible seating rules to the grid, advancing it one round.  Returns the number
    /// of seats that changed states.
    fn tick_visible(&mut self) -> usize {
        let mut new_seats = self.seats.clone();
        let mut num_changed = 0;

        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                let occupied = self.visible(row, col);
                let current_seat = self.seats[row][col];

                // Rules:
                // * Seat becomes occupied if it's empty and there are no visible occupied seats
                // * Seat becomes empty if it's occupied and there are 5+ occupied seats.
                // * Seat state does not change otherwise.
                let new_seat = match current_seat {
                    Empty if occupied == 0 => Occupied,
                    Occupied if occupied >= 5 => Empty,
                    _ => current_seat,
                };

                if new_seat != current_seat {
                    num_changed += 1;
                    new_seats[row][col] = new_seat;
                }
            }
        }

        self.seats = new_seats;

        num_changed
    }

    /// Returns the number of visible occupied seats in all directions.
    fn visible(&self, row: usize, col: usize) -> usize {
        let mut occupied = 0;

        for row_dir in -1..=1 {
            for col_dir in -1..=1 {
                if row_dir == 0 && col_dir == 0 {
                    continue;
                }

                let mut dist = 1;
                let mut square = self.get(
                    row as i32 + row_dir * dist,
                    col as i32 + col_dir * dist);

                while square == Some(Floor) {
                    dist += 1;
                    square = self.get(
                        row as i32 + row_dir * dist,
                        col as i32 + col_dir * dist);
                }

                if square == Some(Occupied) {
                    occupied += 1;
                }
            }
        }

        occupied
    }

    /// Applies visible seating rules to the grid repeatedly until no more seats change state.
    pub fn visible_tick_until_stable(&mut self) {
        while self.tick_visible() > 0 {}
    }

    /// Returns the number of occupied seats in this grid.
    pub fn num_occupied(&self) -> usize {
        self.seats.iter()
            .flat_map(|row| row.iter())
            .filter(|&seat| *seat == Occupied)
            .count()
    }

    /// Returns the seat at the given row and column, or None if they're out of bounds.
    fn get(&self, row: i32, col: i32) -> Option<Seat> {
        if row < 0 || col < 0 {
            return None;
        }

        let row = row as usize;
        let col = col as usize;

        if row >= self.seats.len() || col >= self.seats[row].len() {
            return None;
        }

        Some(self.seats[row][col])
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn load_sample() {
        let grid = Grid::load("sample.txt");

        assert_eq!(10, grid.seats.len());
        assert_eq!(vec![Empty, Floor, Empty, Empty, Floor, Empty, Empty, Floor, Empty, Empty], grid.seats[0]);
    }

    #[test]
    fn tick_adjacent_sample() {
        let mut grid = Grid::load("sample.txt");

        // Tick 1: all seats become occupied.
        assert_eq!(71, grid.tick_adjacent());
        assert_eq!(71, grid.num_occupied());

        // Tick 2: seats around the edges stay occupied, others become empty.
        assert_eq!(51, grid.tick_adjacent());
        assert_eq!(20, grid.num_occupied());
    }

    #[test]
    fn tick_visible_sample() {
        let mut grid = Grid::load("sample.txt");

        // Tick 1: all seats become occupied.
        assert_eq!(71, grid.tick_visible());
        assert_eq!(71, grid.num_occupied());

        // Tick 2: seats near the corners stay occupied, others become empty.
        assert_eq!(64, grid.tick_visible());
        assert_eq!(7, grid.num_occupied());
    }
}