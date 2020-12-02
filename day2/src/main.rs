use std::ops::Range;
use std::str::FromStr;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct ParsePasswordErr {}

#[derive(Debug, Eq, PartialEq)]
struct PasswordPolicy {
    password: String,
    policy: Policy
}

impl PasswordPolicy {
    /// Returns whether the password is valid by checking whether the count of the letter
    /// in the password fits in the range.
    fn validate_range(&self) -> bool {
        let letter_count = self.password.chars()
            .filter(|c| *c == self.policy.letter)
            .count();

        letter_count >= self.policy.count.start && letter_count <= self.policy.count.end
    }

    /// Returns whether the password is valid by checking whether the character appears exactly
    /// once at the range positions.  Positions are 1-indexed, and `1-3 a: abcde` is valid because
    /// `a` appears at position 1 (and not at 3).
    fn validate_position(&self) -> bool {
        let start_letter = self.password.as_bytes()[self.policy.count.start - 1] as char;
        let end_letter = self.password.as_bytes()[self.policy.count.end - 1] as char;

        (start_letter == self.policy.letter && end_letter != self.policy.letter)
            || (start_letter != self.policy.letter && end_letter == self.policy.letter)
    }
}

impl FromStr for PasswordPolicy {
    type Err = ParsePasswordErr;

    /// Parses a PasswordPolicy from the given string.  Policies look like `1-3 a: abcde`,
    /// and consist of a range, a letter, and a password.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$");
        if re.is_err() {
            return Err(ParsePasswordErr {});
        }

        let re = re.unwrap();
        match re.captures(s) {
            Some(captures) => Ok(PasswordPolicy {
                password: String::from(&captures[4]),
                policy: Policy {
                    count: captures[1].parse::<usize>().unwrap() .. captures[2].parse::<usize>().unwrap(),
                    letter: captures[3].parse().unwrap(),
                }
            }),
            None => Err(ParsePasswordErr {}),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Policy {
    count: Range<usize>,
    letter: char,
}

/// Loads passwords and policies from the given file.
fn load(filename: &str) -> Vec<PasswordPolicy> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| line.unwrap().parse::<PasswordPolicy>().unwrap())
        .collect()
}

/// Counts the number of valid passwords in the list using the range validator.
fn count_valid_range(passwords: &Vec<PasswordPolicy>) -> usize {
    passwords.iter()
        .filter(|p| p.validate_range())
        .count()
}

/// Counts the number of valid passwords in the list using the position validator.
fn count_valid_position(passwords: &Vec<PasswordPolicy>) -> usize {
    passwords.iter()
        .filter(|p| p.validate_position())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_password_policy() {
        let parsed = "1-3 a: abcde".parse::<PasswordPolicy>().unwrap();
        let expected = PasswordPolicy {
            password: String::from("abcde"),
            policy: Policy { count: 1..3, letter: 'a' },
        };

        assert_eq!(expected, parsed);
    }

    #[test]
    fn validate_password_policy_range() {
        let a = PasswordPolicy {
            password: String::from("abcde"),
            policy: Policy { count: 1..3, letter: 'a' },
        };

        let b = PasswordPolicy {
            password: String::from("cdefg"),
            policy: Policy { count: 1..3, letter: 'b' },
        };

        assert!(a.validate_range());
        assert!(!b.validate_range());
    }

    #[test]
    fn validate_password_policy_position() {
        let a = PasswordPolicy {
            password: String::from("abcde"),
            policy: Policy { count: 1..3, letter: 'a' },
        };

        let b = PasswordPolicy {
            password: String::from("cdefg"),
            policy: Policy { count: 1..3, letter: 'b' },
        };

        assert!(a.validate_position());
        assert!(!b.validate_position());
    }

    #[test]
    fn test_count_valid_range() {
        let passwords = load("sample.txt");
        assert_eq!(count_valid_range(&passwords), 2);
    }

    #[test]
    fn test_count_valid_position() {
        let passwords = load("sample.txt");
        assert_eq!(count_valid_position(&passwords), 1);
    }
}

fn main() {
    let passwords = load("input.txt");

    println!("Part 1: {}", count_valid_range(&passwords));
    println!("Part 2: {}", count_valid_position(&passwords));
}
