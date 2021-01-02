use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RuleValue {
    Rule(i32),
    Value(char),
}

pub struct Puzzle {
    rules: HashMap<i32, Vec<String>>,
    messages: Vec<String>
}

impl Puzzle {
    /// Loads a puzzle from the given file.
    pub fn load(filename: &str) -> Puzzle {
        // Lines in a puzzle file are rules, an empty line, then messages.
        let f = File::open(filename).unwrap();
        let mut f = BufReader::new(f);

        let mut line = String::new();
        let mut read = f.read_line(&mut line).unwrap();
        let mut rule_lines = Vec::new();
        while line.trim() != "" && read > 0 {
            rule_lines.push(line.trim().to_owned());

            line = String::new();
            read = f.read_line(&mut line).unwrap();
        }


        let mut messages = Vec::new();
        line = String::new();
        read = f.read_line(&mut line).unwrap();
        while read > 0 {
            messages.push(line.trim().to_owned());

            line = String::new();
            read = f.read_line(&mut line).unwrap();
        }

        let rules = Puzzle::parse_rules(rule_lines);

        Puzzle { rules, messages }
    }

    /// Returns the strings that match the given rule.
    pub fn get_rule(&self, num: i32) -> Vec<String> {
        self.rules.get(&num).unwrap().clone()
    }

    /// Parses the given list of raw rules into a map of rule id to matching strings.
    fn parse_rules(rule_lines: Vec<String>) -> HashMap<i32, Vec<String>> {
        // raw_rules is a map of rule id to rule values.
        let mut raw_rules: HashMap<i32, Vec<Vec<RuleValue>>> = HashMap::new();
        // reverse_rules is a map of rule id to the rules that reference the rule.
        let mut reverse_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
        // resolved_rules is a list of rules that were fully resolved this round.
        let mut resolved_rules: HashSet<i32> = HashSet::new();

        // Convert the rules from lines into maps.
        for line in rule_lines {
            // Rule lines are either resolved '4: "a"'
            // A list of rules '0: 4 1 5'
            // Or multiple lists of rules '1: 2 3 | 3 2'
            let colon_index = line.find(':').unwrap();
            let rule_id: i32 = line[0 .. colon_index].parse().unwrap();

            let rule_description = &line[colon_index + 2..];

            if rule_description == "\"a\"" || rule_description == "\"b\"" {
                let rule_description_chars: Vec<char> = rule_description.chars().collect();
                let value = rule_description_chars[1];

                raw_rules.insert(rule_id, vec![vec![RuleValue::Value(value)]]);
                resolved_rules.insert(rule_id);
            } else {
                let mut raw_parts = Vec::new();
                for part in rule_description.split("|") {
                    let rule_refs: Vec<i32> = part.trim().split(" ")
                        .map(|rule_ref| rule_ref.parse().unwrap())
                        .collect();

                    let mut raw_rule_refs = Vec::new();
                    for rule_ref in rule_refs {
                        reverse_rules.entry(rule_ref).or_insert(HashSet::new()).insert(rule_id);
                        raw_rule_refs.push(RuleValue::Rule(rule_ref));
                    }

                    raw_parts.push(raw_rule_refs);
                }

                raw_rules.insert(rule_id, raw_parts);
            }
        }

        // Resolve the rules.  Each round, push the rules that were resolved in the previous
        // round into the rules that reference them.
        while !resolved_rules.is_empty() {
            let no_referenced_rules = HashSet::new();

            // Expand the resolved rules into the rules that reference them.
            for resolved_rule_id in &resolved_rules {
                let resolved_rule = raw_rules.get(resolved_rule_id).unwrap().clone();

                for referenced_rule_id in reverse_rules.get(&resolved_rule_id).unwrap_or(&no_referenced_rules) {
                    raw_rules.entry(*referenced_rule_id)
                        .and_modify(|referenced_rule| Puzzle::expand_rule(referenced_rule, resolved_rule_id, &resolved_rule));
                }
            }

            // New list of resolved rules are the referenced rules that fully became values.
            let new_resolved_rules = resolved_rules.iter()
                .flat_map(|resolved_rule_id| reverse_rules.get(resolved_rule_id).unwrap_or(&no_referenced_rules))
                .filter(|&referenced_rule_id| {
                    raw_rules.get(referenced_rule_id).unwrap().iter()
                        .flat_map(|part| part.iter())
                        .all(|&rv| match rv {
                            RuleValue::Value(_) => true,
                            RuleValue::Rule(_) => false,
                        })
                })
                .cloned()
                .collect();

            resolved_rules = new_resolved_rules;
        }

        // Flatten the resolved rules from lists of values to strings.
        let mut flat_rules = HashMap::new();

        for (rule_id, values) in raw_rules.iter() {
            let flat_values: Vec<String> = values.iter()
                .map(|value| value.iter().map(|v| match v {
                    RuleValue::Value(c) => c,
                    RuleValue::Rule(r) => panic!("Unresolved rule reference {}.", r),
                }).collect())
                .collect();

            flat_rules.insert(*rule_id, flat_values);
        }

        flat_rules
    }

    fn expand_rule(referenced_rule: &mut Vec<Vec<RuleValue>>, resolved_rule_id: &i32, resolved_rule: &Vec<Vec<RuleValue>>) {
        let mut new_referenced_rule = Vec::new();
        for referenced_rule_part in referenced_rule.clone() {
            let mut new_parts: Vec<Vec<RuleValue>> = Vec::new();

            for rv in referenced_rule_part {
                if rv == RuleValue::Rule(*resolved_rule_id) {
                    if new_parts.is_empty() {
                        new_parts.append(resolved_rule.clone().as_mut());
                    } else {
                        let mut new_new_parts: Vec<Vec<RuleValue>> = Vec::new();

                        for resolved_part in resolved_rule {
                            for old_new_part in new_parts.clone() {

                                let new_new_part = old_new_part.iter().cloned()
                                    .chain(resolved_part.iter().cloned())
                                    .collect::<Vec<RuleValue>>();

                                new_new_parts.push(new_new_part);
                            }
                        }

                        new_parts = new_new_parts;
                    }

                } else if new_parts.is_empty() {
                    new_parts.push(vec![rv.clone()]);

                } else {
                    for np in new_parts.iter_mut() {
                        np.push(rv.clone());
                    }
                }
            }

            new_referenced_rule.append(&mut new_parts);
        }

        // raw_rules.insert(*referenced_rule_id, new_referenced_rule);
        referenced_rule.clear();
        referenced_rule.append(&mut new_referenced_rule);
    }

    /// Returns the number of messages that completely match the given rule.
    pub fn matches(&self, rule_num: i32) -> usize {
        self.messages.iter()
            .filter(|message| self.message_matches(message, rule_num))
            .count()
    }

    /// Returns whether the message matches the given rule.
    fn message_matches(&self, message: &String, rule_num: i32) -> bool {
        self.get_rule(rule_num).contains(message)
    }

    /// Returns the number of messages that completely match rule 0 recursively.
    pub fn recursive_matches(&self) -> usize {
        self.messages.iter()
            .filter(|message| self.message_matches(message, 0) || self.message_matches_rule_0_recursively(message))
            .count()
    }

    fn message_matches_rule_0_recursively(&self, message: &String) -> bool {
        // In the recursive part, rules change to:
        // 0: 8 11
        // 8: 42 | 42 8
        // 11: 42 31 | 42 11 31
        // Looking at the question and sample data, 8 = 42, and 42 and 31 have either half of the
        // possible combinations of their space.  42 and 31 have length 5 in the sample data,
        // and they have length 8 in the question.
        // To match, the input needs to be some number of chunks that match 42, followed by
        // at least one fewer chunk that matches 31.

        let chunk_size = self.get_rule(42)[0].len();
        if message.len() % chunk_size != 0 {
            return false;
        }

        let chunks: Vec<String> = message.as_bytes().chunks(chunk_size)
            .map(|chunk| String::from_utf8(Vec::from(chunk)).unwrap())
            .collect();

        // Count the number of 42 chunks followed by the number of 31 chunks.
        let num_42_chunks = chunks.iter()
            .take_while(|chunk| self.message_matches(chunk, 42))
            .count();

        let num_31_chunks = chunks[num_42_chunks..].iter()
            .take_while(|chunk| self.message_matches(chunk, 31))
            .count();

        num_42_chunks + num_31_chunks == chunks.len()
            && num_42_chunks > 0
            && num_31_chunks > 0
            && num_31_chunks < num_42_chunks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sample() {
        let puzzle = Puzzle::load("sample.txt");

        assert_eq!(6, puzzle.rules.len());
        assert_eq!(5, puzzle.messages.len());
    }

    #[test]
    fn matches_sample() {
        let puzzle = Puzzle::load("sample.txt");

        assert_eq!(2, puzzle.matches(0));
    }

    #[test]
    fn recursive_matches_sample() {
        let puzzle = Puzzle::load("recursive_sample.txt");

        assert_eq!(12, puzzle.recursive_matches());
    }

    #[test]
    fn recursive_matches_sample_messages() {
        let puzzle = Puzzle::load("recursive_sample.txt");

        assert!(puzzle.message_matches_rule_0_recursively(&"babbbbaabbbbbabbbbbbaabaaabaaa".to_owned()));
        assert!(!puzzle.message_matches_rule_0_recursively(&"abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_owned()));
        assert!(!puzzle.message_matches_rule_0_recursively(&"aaaabbaaaabbaaa".to_owned()));
        assert!(!puzzle.message_matches_rule_0_recursively(&"babaaabbbaaabaababbaabababaaab".to_owned()));
    }
}