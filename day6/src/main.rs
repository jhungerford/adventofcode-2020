use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;
use std::collections::HashSet;

/// Parses the given file into groups of yes answers.
fn parse(filename: &str) -> Vec<Vec<String>> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut groups = Vec::new();
    let mut lines = Vec::new();
    for line_result in f.lines() {
        let line = line_result.unwrap();

        if line.is_empty() {
            groups.push(lines);
            lines = Vec::new();
        } else {
            lines.push(line.clone());
        }
    }

    if !lines.is_empty() {
        groups.push(lines);
    }

    groups
}

/// Counts the total number of yes answers across all of the groups.  Within a group,
/// a question only counts once if multiple people answer it yes.
fn count_answers_any_yes(groups: &Vec<Vec<String>>) -> usize {
    groups.iter()
        .map(|group| group.iter()
            .flat_map(|line| line.chars())
            .unique()
            .count())
        .fold(0, |sum, count| sum + count)
}

/// Counts the total number of questions that everyone in a group answered yes to.
fn count_answers_all_yes(groups: &Vec<Vec<String>>) -> usize {
    let all_answers: HashSet<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                          'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                          's', 't', 'u', 'v', 'w', 'x', 'y', 'z'].into_iter().collect();

    groups.iter().map(|group| {
        group.iter()
            // Intersection of answers from all of the folks in the group is the number of all-yes
            // answers for that group.
            .map(|person| person.chars().collect::<HashSet<char>>())
            .fold(all_answers.clone(), |yes, person| {
                yes.intersection(&person).cloned().collect()
            })
            .len()
    }).fold(0, |sum, group| sum + group)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample() {
        let answers = parse("sample.txt");

        assert_eq!(5, answers.len());
        assert_eq!(vec!["abc"], answers[0]);
    }

    #[test]
    fn count_answers_any_yes_sample() {
        let answers = parse("sample.txt");

        assert_eq!(11, count_answers_any_yes(&answers));
    }

    #[test]
    fn count_answers_all_yes_sample() {
        let answers = parse("sample.txt");

        assert_eq!(6, count_answers_all_yes(&answers));
    }
}

fn main() {
    let groups = parse("input.txt");

    println!("Part 1: {}", count_answers_any_yes(&groups));
    println!("Part 2: {}", count_answers_all_yes(&groups));
}
