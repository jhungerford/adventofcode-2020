use std::str::FromStr;

use crate::Instruction::{Acc, Jmp, Nop};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (&s[0..3], s[4..].parse::<i32>()) {
            ("nop", Ok(value)) => Ok(Nop(value)),
            ("acc", Ok(value)) => Ok(Acc(value)),
            ("jmp", Ok(value)) => Ok(Jmp(value)),
            _ => Err(ParseErr {})
        }
    }
}

impl Instruction {
    /// Runs the given instruction, returning whether the computer has more instructions to execute.
    fn run(&self, comp: &mut Computer) -> bool {
        let new_pc = match self {
            Nop(_) => {
                comp.pc as i32 + 1
            }

            Acc(value) => {
                comp.acc += *value;
                comp.pc as i32 + 1
            }

            Jmp(value) => {
                comp.pc as i32 + *value
            }
        };

        if new_pc < 0 {
            return false;
        }

        comp.pc = new_pc as usize;
        comp.pc < comp.instructions.len()
    }

    /// Returns the opposite of this instruction.  jmp becomes nop, nop becomes jmp,
    /// and acc stays the same.
    fn toggle(&self) -> Instruction {
        match self {
            Jmp(value) => Nop(*value),
            Nop(value) => Jmp(*value),
            Acc(value) => Acc(*value),
        }
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!("nop +0".parse(), Ok(Nop(0)));
        assert_eq!("acc +1".parse(), Ok(Acc(1)));
        assert_eq!("jmp +4".parse(), Ok(Jmp(4)));
        assert_eq!("jmp -3".parse(), Ok(Jmp(-3)));

        assert_eq!("invalid".parse::<Instruction>(), Err(ParseErr {}));
    }

    #[test]
    fn run_nop() {
        let mut comp = Computer::new(vec![Nop(3), Nop(3)]);
        let instruction = Nop(3);
        instruction.run(&mut comp);

        assert_eq!(comp.pc, 1);
        assert_eq!(comp.acc, 0);
    }

    #[test]
    fn run_acc() {
        let mut comp = Computer::new(vec![Acc(3), Acc(3)]);
        let instruction = Acc(3);
        instruction.run(&mut comp);

        assert_eq!(comp.pc, 1);
        assert_eq!(comp.acc, 3);
    }

    #[test]
    fn run_jmp() {
        let mut comp = Computer::new(vec![Jmp(2), Jmp(1), Jmp(1)]);
        let instruction = Jmp(2);
        instruction.run(&mut comp);

        assert_eq!(comp.pc, 2);
        assert_eq!(comp.acc, 0);
    }

    #[test]
    fn toggle() {
        let instructions = vec![
            Nop(0),
            Acc(1),
            Jmp(3),
        ];

        let toggled: Vec<Instruction> = instructions.iter().map(|i| i.toggle()).collect();

        assert_eq!(toggled, vec![Jmp(0), Acc(1), Nop(3)]);
    }
}

pub struct Computer {
    instructions: Vec<Instruction>,
    pc: usize,
    pub acc: i32,
}

impl Computer {
    /// Returns a new computer initialized at the start state with the given instructions.
    fn new(instructions: Vec<Instruction>) -> Computer {
        Computer {
            instructions,
            pc: 0,
            acc: 0,
        }
    }

    /// Loads the instructions in the given file into a new computer, panicking if the
    /// file can't be loaded or contains invalid instructions.
    pub fn load(filename: &str) -> Computer {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let instructions = f.lines()
            .map(|line| line.unwrap().parse::<Instruction>().unwrap())
            .collect();

        Computer::new(instructions)
    }

    /// Runs the computer until immediately before an instruction would be run for a second time.
    pub fn run_until_loop(&mut self) {
        let mut run_pcs = HashSet::new();
        let mut keep_going = true;

        while keep_going && !run_pcs.contains(&self.pc) {
            run_pcs.insert(self.pc);

            let instruction = self.instructions[self.pc];
            keep_going = instruction.run(self);
        }
    }
}

/// Finds a computer that terminates successfully with the pc immediately past the last instruction
/// by toggling a single instruction in the computer and returns the value of the accumulator.
pub fn find_terminating_computer(comp: &Computer) -> i32 {
    for i in 0..comp.instructions.len() {
        let mut toggled_instructions = comp.instructions.clone();
        toggled_instructions[i] = toggled_instructions[i].toggle();

        let mut toggled_comp = Computer::new(toggled_instructions);
        toggled_comp.run_until_loop();

        if toggled_comp.pc == comp.instructions.len() {
            return toggled_comp.acc;
        }
    }

    panic!("No toggled instructions allow the computer to terminate.")
}

#[cfg(test)]
mod computer_tests {
    use super::*;

    #[test]
    fn load_sample() {
        let computer = Computer::load("sample.txt");

        assert_eq!(computer.pc, 0);
        assert_eq!(computer.acc, 0);
        assert_eq!(computer.instructions.len(), 9);
    }

    #[test]
    fn run_sample_until_loop() {
        let mut computer = Computer::load("sample.txt");
        computer.run_until_loop();

        assert_eq!(computer.acc, 5);
    }

    #[test]
    fn find_terminating_computer_sample() {
        let comp = Computer::load("sample.txt");

        assert_eq!(find_terminating_computer(&comp), 8);
    }
}