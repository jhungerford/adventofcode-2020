#[macro_use]
extern crate lazy_static;
extern crate regex;

use core::fmt;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::MaskValue::{One, Unchanged, Zero};

#[derive(Debug)]
pub struct ParseErr {}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum MaskValue {
    Zero, One, Unchanged,
}

pub struct Mask {
    values: [MaskValue; 36],
}

impl FromStr for Mask {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
        if !s.starts_with("mask = ") || s.len() != 43 {
            return Err(ParseErr {});
        }

        let mut values = [Zero; 36];

        s[7..43].chars().enumerate().for_each(|(i, c)| {
            values[i] = match c {
                '0' => Zero,
                '1' => One,
                'X' => Unchanged,
                other => panic!("Invalid mask char '{}'", other),
            }
        });

        Ok(Mask { values })
    }
}

impl Display for Mask {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..self.values.len() {
            write!(f, "{}", match self.values[i] {
                Zero => "0",
                One => "1",
                Unchanged => "X",
            })?;
        }

        Ok(())
    }
}

impl Mask {
    /// Returns the value that results by applying this mask to the given value.
    fn value(&self, value: i64) -> i64 {
        let mut result = value;

        for i in 0..self.values.len() {
            result = match self.values[i] {
                Zero => result & !(1 << (35 - i)),
                One => result | (1 << (35 - i)),
                Unchanged => result,
            };
        }

        result
    }

    /// Returns a list of locations by applying this mask to the given location.
    fn locations(&self, location: i64) -> Vec<i64> {
        // In the mask, 0 means unchanged, 1 means overwrite with 1, and X means floating.
        let mut base_mask = self.values.clone();
        for i in 0..base_mask.len() {
            base_mask[i] = match self.values[i] {
                Unchanged => Zero,
                Zero => Unchanged,
                One => One,
            }
        }

        // base_location is the location with all unchanged positions set to 0.
        let base_location = Mask { values: base_mask }.value(location);

        let mut locations = Vec::new();
        locations.push(base_location);

        // Loop through the combinations of floating bits, setting them to 1.
        let unchanged_indexes: Vec<usize> = (0..self.values.len())
            .filter(|&i| self.values[i] == Unchanged)
            .collect();

        for num_ones in 1 ..= unchanged_indexes.len() {
            for combo in unchanged_indexes.iter().combinations(num_ones) {
                let mut combo_mask = base_mask.clone();

                for i in combo {
                    combo_mask[*i] = One;
                }

                locations.push(Mask { values: combo_mask }.value(base_location));
            }
        }

        locations
    }
}

#[cfg(test)]
mod mask_tests {
    use super::*;

    #[test]
    fn parse() {
        let expected_mask = [
            Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged,
            Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged,
            Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, Unchanged,
            Unchanged, Unchanged, Unchanged, Unchanged, Unchanged, One, Unchanged, Unchanged,
            Unchanged, Unchanged, Zero, Unchanged,
        ];

        let parsed_mask: Mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();

        assert_eq!(expected_mask.len(), parsed_mask.values.len());

        let mut values_match = true;
        for i in 0..parsed_mask.values.len() {
            values_match = values_match && expected_mask[i] == parsed_mask.values[i];
        }

        assert!(values_match);
    }

    #[test]
    fn value() {
        let mask: Mask = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".parse().unwrap();

        assert_eq!(73, mask.value(11));
        assert_eq!(101, mask.value(101));
        assert_eq!(64, mask.value(0));
    }

    #[test]
    fn locations() {
        let mask: Mask = "mask = 000000000000000000000000000000X1001X".parse().unwrap();
        assert_eq!(vec![26, 58, 27, 59], mask.locations(42));

        let mask: Mask = "mask = 00000000000000000000000000000000X0XX".parse().unwrap();
        assert_eq!(vec![16, 24, 18, 17, 26, 25, 19, 27], mask.locations(26));
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MemSet {
    location: i64,
    value: i64,
}

impl FromStr for MemSet {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // mem[8] = 11
        lazy_static! {
            static ref MEM_SET_RE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
        }

        MEM_SET_RE.captures(s)
            .map(|captures| Ok(MemSet {
                location: captures[1].parse().unwrap(),
                value: captures[2].parse().unwrap(),
            }))
            .unwrap_or(Err(ParseErr {}))
    }
}

#[cfg(test)]
mod mem_set_tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(MemSet { location: 8, value: 11 }, "mem[8] = 11".parse().unwrap());
        assert_eq!(MemSet { location: 7, value: 101 }, "mem[7] = 101".parse().unwrap());
        assert_eq!(MemSet { location: 8, value: 0 }, "mem[8] = 0".parse().unwrap());
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Memory {
    values: HashMap<i64, i64>
}

impl Memory {
    /// Returns a new Memory with all values set to 0.
    pub fn new() -> Memory {
        Memory { values: HashMap::new() }
    }

    /// Sets a value in this memory by applying a mask to a value.
    pub fn set(&mut self, location: i64, value: i64) {
        self.values.insert(location, value);
    }

    /// Returns the value at the given location in memory.
    pub fn get(&mut self, location: i64) -> i64 {
        *self.values.get(&location).unwrap_or(&0)
    }

    /// Returns the sum of all of the values in memory.
    pub fn sum(&self) -> i64 {
        self.values.values().sum()
    }
}

#[cfg(test)]
mod memory_tests {
    use super::*;

    #[test]
    fn memory_operations() {
        let mut mem = Memory::new();

        mem.set(8, 73);
        assert_eq!(73, mem.get(8));

        let mem_set = MemSet { location: 7, value: 101 };
        mem.set(7, 101);
        assert_eq!(101, mem.get(7));

        let mem_set = MemSet { location: 8, value: 0 };
        mem.set(8, 64);
        assert_eq!(64, mem.get(8));

        assert_eq!(165, mem.sum());
    }

}

pub struct Instruction {
    mask: Mask,
    sets: Vec<MemSet>
}

/// Loads instructions from the given file.
pub fn load_instructions(filename: &str) -> Vec<Instruction> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut instructions = Vec::new();

    let mut mask: Option<Mask> = None;
    let mut sets = Vec::new();

    for line_result in f.lines() {
        let line = line_result.unwrap();

        if line.starts_with("mask") {
            if mask.is_some() && !sets.is_empty() {
                instructions.push(Instruction { mask: mask.unwrap(), sets });
            }

            mask = Some(line.parse().unwrap());
            sets = Vec::new();
        } else if line.starts_with("mem") {
            sets.push(line.parse().unwrap());
        }
    }

    if mask.is_some() && !sets.is_empty() {
        instructions.push(Instruction { mask: mask.unwrap(), sets });
    }

    instructions
}

/// Runs the given instructions on uninitialized memory, returning the resulting memory.
/// Instructions apply a mask to a value to set a single memory address.
pub fn run_instructions(instructions: &Vec<Instruction>) -> Memory {
    let mut mem = Memory::new();

    for instruction in instructions {
        for set in &instruction.sets {
            mem.set(set.location, instruction.mask.value(set.value));
        }
    }

    mem
}

/// Runs the given instructions on uninitialized memory, returning the resulting memory.
/// Instructions apply a mask to a memory address and set a value.
pub fn run_instructions_v2(instructions: &Vec<Instruction>) -> Memory {
    let mut mem = Memory::new();

    for instruction in instructions {
        for set in &instruction.sets {
            for loc in instruction.mask.locations(set.location) {
                mem.set(loc, set.value);
            }
        }
    }

    mem
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn sample_load_instructions() {
        let instructions = load_instructions("sample.txt");

        assert_eq!(1, instructions.len());
        assert_eq!(3, instructions[0].sets.len());
    }

    #[test]
    fn sample_run() {
        let instructions = load_instructions("sample.txt");
        let mem = run_instructions(&instructions);

        assert_eq!(165, mem.sum());
    }

    #[test]
    fn sample_run_v2() {
        let instructions = load_instructions("sample_v2.txt");
        let mem = run_instructions_v2(&instructions);

        assert_eq!(208, mem.sum());
    }
}