use std::collections::HashMap;

struct Numbers {
    start: Vec<i32>,
    round: usize,
    last_num: i32,
    num_round: HashMap<i32, usize>,
}

impl Numbers {
    /// Creates a new Numbers that starts with the given numbers.
    fn new(start: Vec<i32>) -> Numbers {
        Numbers {
            start,
            round: 0,
            last_num: 0,
            num_round: HashMap::new(),
        }
    }
}

impl Iterator for Numbers {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // Players take turns saying numbers.  They first read the starting numbers, then each
        // turn considers the most recently spoken number:
        // * If it's the first time the number has been spoken, the player says 0.
        // * Otherwise, the player says how many turns ago it was previously spoken.

        if self.round < self.start.len() {
            let num = self.start[self.round];

            if self.round > 0 {
                self.num_round.insert(self.start[self.round - 1], self.round - 1);
            }

            self.last_num = num;
            self.round += 1;

            return Some(num);
        }

        let prev_round = self.num_round.get(&self.last_num);

        let num = match prev_round {
            Some(&round) if round < self.round - 1 => (self.round - round - 1) as i32,
            _ => 0,
        };

        self.num_round.insert(self.last_num, self.round - 1);
        self.last_num = num;
        self.round += 1;

        Some(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_samples() {
        let nums = Numbers::new(vec![0, 3, 6]);
        assert_eq!(vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0], nums.take(10).collect::<Vec<i32>>());

        assert_eq!(1, Numbers::new(vec![1, 3, 2]).skip(2019).next().unwrap());
        assert_eq!(10, Numbers::new(vec![2, 1, 3]).skip(2019).next().unwrap());
        assert_eq!(27, Numbers::new(vec![1, 2, 3]).skip(2019).next().unwrap());
        assert_eq!(78, Numbers::new(vec![2, 3, 1]).skip(2019).next().unwrap());
        assert_eq!(438, Numbers::new(vec![3, 2, 1]).skip(2019).next().unwrap());
        assert_eq!(1836, Numbers::new(vec![3, 1, 2]).skip(2019).next().unwrap());
    }

    #[test]
    fn numbers_samples_thirty_millionth() {
        assert_eq!(175594, Numbers::new(vec![0, 3, 6]).skip(29999999).next().unwrap());
        assert_eq!(2578, Numbers::new(vec![1, 3, 2]).skip(29999999).next().unwrap());
        assert_eq!(3544142, Numbers::new(vec![2, 1, 3]).skip(29999999).next().unwrap());
        assert_eq!(261214, Numbers::new(vec![1, 2, 3]).skip(29999999).next().unwrap());
        assert_eq!(6895259, Numbers::new(vec![2, 3, 1]).skip(29999999).next().unwrap());
        assert_eq!(18, Numbers::new(vec![3, 2, 1]).skip(29999999).next().unwrap());
        assert_eq!(362, Numbers::new(vec![3, 1, 2]).skip(29999999).next().unwrap());
    }
}

fn main() {
    let part1_nums = Numbers::new(vec![0, 13, 1, 16, 6, 17]);
    let part2_nums = Numbers::new(vec![0, 13, 1, 16, 6, 17]);

    println!("Part 1: {}", part1_nums.skip(2019).next().unwrap());
    println!("Part 2: {}", part2_nums.skip(29999999).next().unwrap());
}
