use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

/// Loads numbers out of the given file, panicing if the file doesn't exist or is invalid.
fn load_input(filename: &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .flat_map(|line| i32::from_str(line.unwrap().as_str()))
        .collect()
}

/// Finds two numbers that sum to 2020 and returns their product.
fn find_two_2020_product(numbers: &Vec<i32>) -> i32 {
    for number in numbers {
        let complement = 2020 - number;
        if numbers.contains(&complement) {
            return number * complement;
        }
    }

    0
}

/// Finds three numbers that sum to 2020 and returns their product.
fn find_three_2020_product(numbers: &Vec<i32>) -> i32 {
    for combo in numbers.iter().combinations(3) {
        if combo.iter().fold(0, |sum, &num| sum + num) == 2020 {
            return combo.iter().fold(1, |product, &num| product * num);
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_input() {
        let numbers = load_input("input.txt");
        assert!(!numbers.is_empty());
    }

    #[test]
    fn test_find_two_2020() {
        let numbers = load_input("sample.txt");
        assert_eq!(514579, find_two_2020_product(&numbers));
    }

    #[test]
    fn test_find_three_2020() {
        let numbers = load_input("sample.txt");
        assert_eq!(241861950, find_three_2020_product(&numbers));
    }
}

fn main() {
    let lines = load_input("input.txt");

    println!("Part 1: {}", find_two_2020_product(&lines));
    println!("Part 2: {}", find_three_2020_product(&lines));
}
