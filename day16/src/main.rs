#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::ops::RangeInclusive;
use std::str::FromStr;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Rule {
    name: String,
    range_a: RangeInclusive<i32>,
    range_b: RangeInclusive<i32>,
}

impl FromStr for Rule {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // departure location: 33-430 or 456-967
            static ref RULE_RE: Regex = Regex::new(r"^([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        }

        RULE_RE.captures(s)
            .map(|captures| Ok(Rule {
                name: captures[1].to_string(),
                range_a: captures[2].parse().unwrap() ..= captures[3].parse().unwrap(),
                range_b: captures[4].parse().unwrap() ..= captures[5].parse().unwrap(),
            }))
            .unwrap_or(Err(ParseErr {}))
    }
}

impl Rule {
    fn matches(&self, value: i32) -> bool {
        self.range_a.contains(&value) || self.range_b.contains(&value)
    }
}

#[cfg(test)]
mod rule_tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!("departure location: 33-430 or 456-967".parse(), Ok(Rule {
            name: "departure location".to_owned(),
            range_a: 33 ..= 430,
            range_b: 456 ..= 967
        }));
        assert_eq!("departure station: 42-864 or 875-957".parse(), Ok(Rule {
            name: "departure station".to_owned(),
            range_a: 42 ..= 864,
            range_b: 875 ..= 957
        }));
        assert_eq!("departure platform: 42-805 or 821-968".parse(), Ok(Rule {
            name: "departure platform".to_owned(),
            range_a: 42 ..= 805,
            range_b: 821 ..= 968
        }));
        assert_eq!("departure track: 34-74 or 93-967".parse(), Ok(Rule {
            name: "departure track".to_owned(),
            range_a: 34 ..= 74,
            range_b: 93 ..= 967
        }));
        assert_eq!("departure date: 40-399 or 417-955".parse(), Ok(Rule {
            name: "departure date".to_owned(),
            range_a: 40 ..= 399,
            range_b: 417 ..= 955
        }));
        assert_eq!("departure time: 30-774 or 797-950".parse(), Ok(Rule {
            name: "departure time".to_owned(),
            range_a: 30 ..= 774,
            range_b: 797 ..= 950
        }));
    }

    #[test]
    fn matches() {
        let rule: Rule = "class: 1-3 or 5-7".parse().unwrap();

        assert!(rule.matches(7));
        assert!(rule.matches(1));
        assert!(!rule.matches(4));
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Ticket {
    values: Vec<i32>
}

impl FromStr for Ticket {
    type Err = ParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 7,1,14
        Ok(Ticket {
            values: s.split(",").map(|value| value.parse().unwrap()).collect()
        })
    }
}

impl Ticket {
    /// Returns a sum of values in this ticket that don't match any rule.
    fn invalid_values(&self, rules: &Vec<Rule>) -> i32 {
        self.values.iter()
            .filter(|&value| !rules.iter().any(|rule| rule.matches(*value)))
            .sum()
    }
}

#[cfg(test)]
mod ticket_tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!("7,3,47".parse(), Ok(Ticket { values: vec![7, 3, 47] }));
        assert_eq!("40,4,50".parse(), Ok(Ticket { values: vec![40, 4, 50] }));
    }

    #[test]
    fn invalid_values() {
        let rules: Vec<Rule> = vec![
            "class: 1-3 or 5-7".parse().unwrap(),
            "row: 6-11 or 33-44".parse().unwrap(),
            "seat: 13-40 or 45-50".parse().unwrap(),
        ];

        assert_eq!(0, Ticket { values: vec![7, 3, 47] }.invalid_values(&rules));
        assert_eq!(4, Ticket { values: vec![40, 4, 50] }.invalid_values(&rules));
        assert_eq!(55, Ticket { values: vec![55, 2, 20] }.invalid_values(&rules));
        assert_eq!(12, Ticket { values: vec![38, 6, 12] }.invalid_values(&rules));
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Input {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input {
    /// Loads input from the given file.
    fn load(filename: &str) -> Input {
        // Input is rules, followed by a blank line
        // 'your ticket:' followed by your ticket and a blank line
        // 'nearby tickets:' followed by nearby tickets.
        let f = File::open(filename).unwrap();

        let mut f = BufReader::new(f);
        let mut line = String::new();


        // Rules
        let mut rules = Vec::new();
        let mut line_len = f.read_line(&mut line);

        while line_len.map(|len| len > 1).unwrap_or(false) {
            rules.push(line.trim().parse().unwrap());

            line.clear();
            line_len = f.read_line(&mut line);
        }

        // Your ticket
        f.read_line(&mut line);
        if line.trim() != "your ticket:" {
            panic!("Input is missing your ticket.")
        }

        line.clear();
        f.read_line(&mut line);
        let ticket = line.trim().parse().unwrap();

        f.read_line(&mut line); // Blank line

        // Nearby tickets
        line.clear();
        f.read_line(&mut line);
        if line.trim() != "nearby tickets:" {
            panic!("Input is missing nearby tickets.")
        }

        let mut nearby_tickets = Vec::new();
        line.clear();
        line_len = f.read_line(&mut line);

        while line_len.map(|len| len > 1).unwrap_or(false) {
            nearby_tickets.push(line.trim().parse().unwrap());

            line.clear();
            line_len = f.read_line(&mut line);
        }

        Input { rules, ticket, nearby_tickets }
    }

    /// Returns a sum of values in nearby tickets in the input that don't match any rules.
    fn error_rate(&self) -> i32 {
        self.nearby_tickets.iter()
            .map(|ticket| ticket.invalid_values(&self.rules))
            .sum()
    }

    /// Finds fields that start with 'departure' in your ticket,
    /// and returns the product of their values.
    fn departure_fields(&self) -> i64 {
        // Map of rule to the index of the field it applies to.
        let rule_fields = self.rule_fields();

        rule_fields.keys()
            .filter(|name| name.starts_with("departure"))
            .map(|name| self.ticket.values[*rule_fields.get(name).unwrap()] as i64)
            .product()
    }

    /// Returns a map of rule name to the field that it applies to.
    fn rule_fields(&self) -> HashMap<String, usize> {
        // Discard tickets that have values that don't match any field.
        let valid_tickets: Vec<&Ticket> = self.nearby_tickets.iter()
            .filter(|ticket| ticket.invalid_values(&self.rules) == 0)
            .collect();

        let mut rule_fields: HashMap<String, usize> = HashMap::new();
        while rule_fields.len() != self.rules.len() {
            let known_fields: Vec<usize> = rule_fields.values().cloned().collect();
            let rules: Vec<&Rule> = self.rules.iter()
                .filter(|rule| !rule_fields.contains_key(&rule.name))
                .collect();
            let fields: Vec<usize> = (0..self.rules.len())
                .filter(|field| !known_fields.contains(field))
                .collect();

            for rule in rules {
                let valid_rule_fields: Vec<usize> = fields.iter()
                    .filter(|&f| valid_tickets.iter().all(|ticket| rule.matches(ticket.values[*f])))
                    .cloned()
                    .collect();

                if valid_rule_fields.len() == 0 {
                    panic!("Rule '{}' doesn't match any tickets.", rule.name);
                }

                if valid_rule_fields.len() == 1 {
                    rule_fields.insert(rule.name.clone(), valid_rule_fields[0]);
                }
            }
        }

        rule_fields
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;

    #[test]
    fn load() {
        let input = Input::load("sample.txt");

        assert_eq!(3, input.rules.len());
        assert_eq!(vec![7, 1, 14], input.ticket.values);
        assert_eq!(4, input.nearby_tickets.len());
    }

    #[test]
    fn error_rate() {
        let input = Input::load("sample.txt");
        assert_eq!(input.error_rate(), 71);
    }

    #[test]
    fn rule_fields() {
        let input = Input::load("sample2.txt");

        let expected: HashMap<String, usize> = [
            ("class".to_owned(), 1 as usize),
            ("row".to_owned(), 0 as usize),
            ("seat".to_owned(), 2 as usize),
        ].iter().cloned().collect();

        assert_eq!(expected, input.rule_fields());
    }
}

fn main() {
    let input = Input::load("input.txt");

    println!("Part 1: {}", input.error_rate());
    println!("Part 2: {}", input.departure_fields());
}
