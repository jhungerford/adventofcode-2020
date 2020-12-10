use std::fs::File;
use std::io::{BufReader, BufRead};

/// Loads jolts from the given file.
fn load(filename: &str) -> Vec<i32> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

/// Returns the product of the 1-jolt differences multiplied by the number of 3-jolt differences.
fn differences(jolts: &Vec<i32>) -> i32 {
    let mut sorted_jolts = jolts.clone();

    sorted_jolts.sort();

    sorted_jolts.insert(0, 0);
    sorted_jolts.push(sorted_jolts[sorted_jolts.len() - 1] + 3);

    let differences: Vec<i32> = sorted_jolts.windows(2)
        .map(|t| t[1] - t[0])
        .collect();

    let ones = differences.iter().filter(|&d| *d == 1).count();
    let threes = differences.iter().filter(|&d| *d == 3).count();

    (ones * threes) as i32
}

/// Returns the number of valid ways that the jolt adapters can be combined.  Adapters can
/// transform up to 3 jolts.
fn combinations(jolts: &Vec<i32>) -> i64 {
    let mut sorted_jolts = jolts.clone();

    sorted_jolts.sort();

    // Seat is 0 jolts and device is biggest adapter + 3
    sorted_jolts.insert(0, 0);
    sorted_jolts.push(sorted_jolts[sorted_jolts.len() - 1] + 3);

    let differences: Vec<i32> = sorted_jolts.windows(2)
        .map(|t| t[1] - t[0])
        .collect();

    let mut consecutive_ones = Vec::new();
    let mut ones = 0;

    for difference in &differences {
        if *difference == 1 {
            ones += 1;
        } else if ones > 0 {
            consecutive_ones.push(ones);
            ones = 0;
        }
    }

    consecutive_ones.iter().map(|consecutive| match consecutive {
        4 => 7,
        3 => 4,
        2 => 2,
        _ => 1
    }).fold(1, |product, combos| product * combos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differences_sample() {
        let jolts = load("sample.txt");

        assert_eq!(35, differences(&jolts));
    }

    #[test]
    fn differences_sample2() {
        let jolts = load("sample2.txt");

        assert_eq!(220, differences(&jolts));
    }

    #[test]
    fn combinations_sample() {
        let jolts = load("sample.txt");

        assert_eq!(8, combinations(&jolts));
    }

    #[test]
    fn combinations_sample2() {
        let jolts = load("sample2.txt");

        assert_eq!(19208, combinations(&jolts));
    }
}

fn main() {
    let jolts = load("input.txt");

    println!("Part 1: {}", differences(&jolts));
    println!("Part 2: {}", combinations(&jolts));
}
