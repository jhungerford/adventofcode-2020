use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::Token::{Num, Plus, Times};
use crate::Mode::AddBeforeTimes;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    LeftToRight,
    AddBeforeTimes,
}

enum Token {
    Num(i64),
    Plus(Box<Token>, Box<Token>),
    Times(Box<Token>, Box<Token>),
}

impl Token {
    fn run(&self) -> i64 {
        match self {
            Num(value) => *value,
            Plus(a, b) => a.run() + b.run(),
            Times(a, b) => a.run() * b.run(),
        }
    }
}

/// Evaluates the expression in the given string.
fn parse(s: &str, mode: Mode) -> Token {
    let mut chars: Vec<char> = s.replace(" ", "").chars().collect();

    // For add-before-times, insert parenthesis around addition
    if mode == AddBeforeTimes {
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '+' {
                let left = match chars[i - 1] {
                    num if num >= '0' && num <= '9' => i - 1,
                    ')' => matching_open_paren(&chars, i - 1),
                    _ => panic!("Plus must have a value to the left."),
                };

                chars.insert(left, '(');
                i += 1;

                let right = match chars[i + 1] {
                    num if num >= '0' && num <= '9' => i + 2,
                    '(' => matching_close_paren(&chars, i + 1) + 1,
                    _ => panic!("Plus must have a value to the right."),
                };

                chars.insert(right, ')');

                i += 2;
            } else {
                i += 1;
            }
        }
    }

    parse_section(&chars, 0, chars.len())
}

/// Evaluates a section of the given expression.
fn parse_section(chars: &Vec<char>, from: usize, to: usize) -> Token {
    let (mut left, mut i) = parse_value(chars, from);

    while i < to {
        let (right, new_i) = parse_value(chars, i + 1);

        left = match chars[i] {
            '+' => Plus(Box::from(left), Box::from(right)),
            '*' => Times(Box::from(left), Box::from(right)),
            _ => panic!("Section {}..{} must have an operator", from, to),
        };

        i = new_i;
    }

    left
}

/// Parses a number or parenthesis value starting at the given position.
/// Returns the parsed token and the index of the next unparsed character.
fn parse_value(chars: &Vec<char>, from: usize) -> (Token, usize) {
    match chars[from] {
        '(' => {
            let close = matching_close_paren(chars, from);
            (parse_section(chars, from + 1, close), close + 1)
        },

        num if num >= '0' && num <= '9' => {
            (Num(num as i64 - '0' as i64), from + 1)
        },

        _ => panic!("Section must start with a number or parenthesis."),
    }
}

/// Returns the index of the close parentheses that matches the one under from, looking right.
fn matching_close_paren(chars: &Vec<char>, from: usize) -> usize {
    let mut i = from + 1;
    let mut num_parens = 1;

    while num_parens > 0 {
        match chars[i] {
            '(' => num_parens += 1,
            ')' => num_parens -= 1,
            _ => {},
        }

        if num_parens == 0 {
            return i;
        }

        i += 1;
    }

    unreachable!()
}

/// Returns the index of the open parentheses that matches the one under to, looking left.
fn matching_open_paren(chars: &Vec<char>, to: usize) -> usize {
    let mut i = to - 1;
    let mut num_parens = 1;

    while num_parens > 0 {
        match chars[i] {
            ')' => num_parens += 1,
            '(' => num_parens -= 1,
            _ => {},
        }

        if num_parens == 0 {
            return i;
        }

        i -= 1;
    }

    unreachable!()
}

fn eval(s: &str, mode: Mode) -> i64 {
    let expr = parse(s, mode);
    expr.run()
}

/// Loads expressions from the given file (one per line) and returns their sum.
pub fn sum_expressions(filename: &str, mode: Mode) -> i64 {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().fold(0, |sum, line| sum + eval(line.unwrap().as_str(), mode))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode::{LeftToRight, AddBeforeTimes};

    #[test]
    fn samples_left_to_right() {
        assert_eq!(71, eval("1 + 2 * 3 + 4 * 5 + 6", LeftToRight));
        assert_eq!(51, eval("1 + (2 * 3) + (4 * (5 + 6))", LeftToRight));
        assert_eq!(26, eval("2 * 3 + (4 * 5)", LeftToRight));
        assert_eq!(437, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", LeftToRight));
        assert_eq!(12240, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", LeftToRight));
        assert_eq!(13632, eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", LeftToRight));
    }

    #[test]
    fn samples_add_before_times() {
        assert_eq!(231, eval("1 + 2 * 3 + 4 * 5 + 6", AddBeforeTimes));
        assert_eq!(51, eval("1 + (2 * 3) + (4 * (5 + 6))", AddBeforeTimes));
        assert_eq!(46, eval("2 * 3 + (4 * 5)", AddBeforeTimes));
        assert_eq!(1445, eval("5 + (8 * 3 + 9 + 3 * 4 * 3)", AddBeforeTimes));
        assert_eq!(669060, eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", AddBeforeTimes));
        assert_eq!(23340, eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", AddBeforeTimes));
    }
}
