#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet, BinaryHeap};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    num: usize,
    color: String,
}

/// Loads rules (map of bag color to the bags inside) from the given file, panicking if the
/// file is invalid or can't be loaded.
fn load_rules(filename: &str) -> HashMap<String, Vec<Bag>> {
    // bright white bags contain 1 shiny gold bag.
    // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    lazy_static! {
        static ref BAG_COUNT_RE: Regex = Regex::new(r"(\d+) ([a-z ]+) bags?").unwrap();
    }

    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut rules = HashMap::new();

    for line_result in f.lines() {
        let line = line_result.unwrap();

        let bag_color_idx = line.find(" bags contain");
        if bag_color_idx.is_none() {
            println!("No bag color: '{}'", line);
            continue;
        }

        let color = line[0..bag_color_idx.unwrap()].to_owned();

        let inside = if line.ends_with("contain no other bags.") {
            vec![]
        } else {
            BAG_COUNT_RE.captures_iter(line.as_str()).map(|capture| Bag {
                num: capture[1].parse().unwrap(),
                color: capture[2].to_owned(),
            }).collect()
        };

        rules.insert(color, inside);
    }

    rules
}

/// Counts the number of bag colors that contain at least one shiny gold bag.
fn count_shiny_gold(rules: &HashMap<String, Vec<Bag>>) -> usize {
    let mut bag_containers = HashMap::new();

    for container in rules.keys() {
        for inside in &rules[container] {
            bag_containers.entry(inside.color.clone()).or_insert(Vec::new()).push(container.clone());
        }
    }

    let mut to_visit = BinaryHeap::new();
    to_visit.push("shiny gold".to_owned());

    let mut containers = HashSet::new();

    while let Some(color) = to_visit.pop() {
        if !bag_containers.contains_key(&color) {
            continue;
        }

        for container in bag_containers.get(&color).unwrap() {
            if !containers.contains(container) {
                containers.insert(container);
                to_visit.push(container.to_owned());
            }
        }
    }

    containers.len()
}

/// Counts the number of bags required inside of the shiny gold bag, not counting the gold bag.
fn count_bags_in_shiny_gold(rules: &HashMap<String, Vec<Bag>>) -> usize {
    count_bags(rules, "shiny gold") - 1
}

/// Counts the number of bags inside the given bag, including the bag.
fn count_bags(rules: &HashMap<String, Vec<Bag>>, color: &str) -> usize {
    let count = match rules.get(color) {
        None => 1,
        Some(bags) => {
            let mut count = 1;

            for bag in bags {
                count += bag.num * count_bags(rules, &bag.color);
            }

            count
        }
    };

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sample() {
        let rules = load_rules("sample.txt");

        assert_eq!(9, rules.len());

        assert_eq!(Some(&vec![
            Bag { num: 1, color: "bright white".to_owned() },
            Bag { num: 2, color: "muted yellow".to_owned() },
        ]), rules.get("light red"));

        assert_eq!(
            Some(&vec![Bag { num: 1, color: "shiny gold".to_owned() }]),
            rules.get("bright white"));
    }

    #[test]
    fn count_sample() {
        let rules = load_rules("sample.txt");

        assert_eq!(4, count_shiny_gold(&rules));
    }

    #[test]
    fn count_bags_inside_sample() {
        let rules = load_rules("sample.txt");

        assert_eq!(32, count_bags_in_shiny_gold(&rules));
    }

    #[test]
    fn count_bags_inside_sample2() {
        let rules = load_rules("sample2.txt");

        assert_eq!(126, count_bags_in_shiny_gold(&rules));
    }
}

fn main() {
    let rules = load_rules("input.txt");

    println!("Part 1: {}", count_shiny_gold(&rules));
    println!("Part 2: {}", count_bags_in_shiny_gold(&rules));
}
