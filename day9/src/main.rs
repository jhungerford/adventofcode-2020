use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

/// Loads numbers from the given file.
fn load_nums(filename: &str) -> Vec<i64> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

/// Finds the first number in the list that isn't a sum of two previous numbers
fn first_non_sum(nums: &Vec<i64>, preamble: usize) -> i64 {
    for i in preamble..nums.len() {
        let num = nums[i];
        let is_sum = nums[(i-preamble)..].iter()
            .combinations(2)
            .any(|combo| combo[0] + combo[1] == nums[i]);

        if !is_sum {
            return num;
        }
    }

    panic!("All numbers are sums.");
}

/// Finds the sum of the smallest and largest numbers in a contiguous sequence that sums to
/// the given number.
fn contiguous_sum(nums: &Vec<i64>, num: i64) -> i64 {
    for i in 0..nums.len() {
        for j in i + 1 .. nums.len() {
            let sum: i64 = nums[i..j].iter().sum();

            if sum == num {
                return nums[i..j].iter().min().unwrap() + nums[i..j].iter().max().unwrap();
            } else if sum > num {
                continue;
            }
        }
    }

    panic!("No sequence sums to the given number.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_non_sum_sample() {
        let nums = load_nums("sample.txt");

        assert_eq!(127, first_non_sum(&nums, 5))
    }

    #[test]
    fn contiguous_sum_sample() {
        let nums = load_nums("sample.txt");

        assert_eq!(62, contiguous_sum(&nums, 127));
    }
}

fn main() {
    let nums = load_nums("input.txt");

    let non_sum = first_non_sum(&nums, 25);
    println!("Part 1: {}", non_sum);
    println!("Part 2: {}", contiguous_sum(&nums, non_sum));
}
