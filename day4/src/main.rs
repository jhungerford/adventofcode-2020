#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::str::FromStr;
use crate::Field::{BirthYear, IssueYear, ExpirationYear, Height, HairColor, EyeColor, PassportID, CountryID};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct ParseErr {}

#[derive(Debug, Eq, PartialEq)]
enum Field {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportID(String),
    CountryID(String),
}

impl Field {
    fn code(&self) -> &str {
        match self {
            BirthYear(_) => "byr",
            IssueYear(_) => "iyr",
            ExpirationYear(_) => "eyr",
            Height(_) => "hgt",
            HairColor(_) => "hcl",
            EyeColor(_) => "ecl",
            PassportID(_) => "pid",
            CountryID(_) => "cid",
        }
    }

    fn validate(&self) -> bool {
        lazy_static! {
            static ref YEAR_RE: Regex = Regex::new(r"^(\d{4})$").unwrap();
            static ref CM_RE: Regex = Regex::new(r"^(\d+)cm$").unwrap();
            static ref IN_RE: Regex = Regex::new(r"^(\d+)in$").unwrap();
            static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLORS: HashSet<String> = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter().map(|s| s.to_string()).collect();
            static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        match self {
            // four digits; at least 1920 and at most 2002.
            BirthYear(s) => {
                YEAR_RE.captures(s).map(|captures| {
                    let year: i32 = captures[1].parse().unwrap();
                    year >= 1920 && year <= 2002
                }).unwrap_or(false)
            }

            // four digits; at least 2010 and at most 2020.
            IssueYear(s) => {
                YEAR_RE.captures(s).map(|captures| {
                    let year: i32 = captures[1].parse().unwrap();
                    year >= 2010 && year <= 2020
                }).unwrap_or(false)
            }

            // four digits; at least 2020 and at most 2030.
            ExpirationYear(s) => {
                YEAR_RE.captures(s).map(|captures| {
                    let year: i32 = captures[1].parse().unwrap();
                    year >= 2020 && year <= 2030
                }).unwrap_or(false)
            }

            // a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            // If in, the number must be at least 59 and at most 76.
            Height(s) => {
                let cm = CM_RE.captures(s).map(|captures| {
                    let cm: i32 = captures[1].parse().unwrap();
                    cm >= 150 && cm <= 193
                });

                if cm.is_some() {
                    return cm.unwrap();
                }

                IN_RE.captures(s).map(|captures| {
                    let num: i32 = captures[1].parse().unwrap();
                    num >= 59 && num <= 76
                }).unwrap_or(false)
            }

            // a # followed by exactly six characters 0-9 or a-f
            HairColor(s) => HAIR_RE.is_match(s),

            // exactly one of: amb blu brn gry grn hzl oth
            EyeColor(s) => EYE_COLORS.contains(s.as_str()),

            // a nine-digit number, including leading zeroes
            PassportID(s) => PASSPORT_ID_RE.is_match(s),

            // ignored, missing or not.
            CountryID(_) => true,
        }
    }
}

impl FromStr for Field {
    type Err = ParseErr;

    fn from_str<'a>(str: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = str.split(":").collect();
        if parts.len() != 2 {
            return Err(ParseErr {});
        }

        match parts[0] {
            "byr" => Ok(BirthYear(parts[1].to_owned())),
            "iyr" => Ok(IssueYear(parts[1].to_owned())),
            "eyr" => Ok(ExpirationYear(parts[1].to_owned())),
            "hgt" => Ok(Height(parts[1].to_owned())),
            "hcl" => Ok(HairColor(parts[1].to_owned())),
            "ecl" => Ok(EyeColor(parts[1].to_owned())),
            "pid" => Ok(PassportID(parts[1].to_owned())),
            "cid" => Ok(CountryID(parts[1].to_owned())),
            _ => Err(ParseErr {}),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Passport {
    fields: Vec<Field>
}

impl Passport {
    /// Returns whether this passport is valid (has all of the required fields).
    fn validate_has_fields(&self) -> bool {
        // cid is optional, all other fields are required.
        let expected: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter().cloned().collect();

        let actual: HashSet<&str> = self.fields.iter()
            .map(|f| f.code())
            .collect();

        let missing: HashSet<_> = expected.difference(&actual).collect();

        missing.is_empty()
    }

    /// Returns whether this passport is valid (has the required fields, and valid values for fields).
    fn validate_field_values(&self) -> bool {
        self.fields.iter().fold(true, |valid, field| valid && field.validate())
    }
}

/// Loads passports from the given file.
fn load_passports(filename: &str) -> Vec<Passport> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut passports = Vec::new();

    // Passports are a series of fields separated by an empty line.
    let mut field_strs = Vec::new();
    for line_result in f.lines() {
        let line = line_result.unwrap();

        if line.is_empty() {
            let fields = field_strs.iter()
                .map(|f: &String| f.parse().unwrap())
                .collect();

            passports.push(Passport { fields });

            field_strs = Vec::new();
        } else {
            line.split(" ").for_each(|f| field_strs.push(f.to_owned()));
        }
    }

    if !field_strs.is_empty() {
        let fields = field_strs.iter()
            .map(|f: &String| f.parse().unwrap())
            .collect();

        passports.push(Passport { fields });
    }

    passports
}

/// Counts the number of valid passports in the list.
fn count_valid(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|p| p.validate_has_fields())
        .count()
}

/// Counts the number of valid passports in the list.
fn count_valid_field_values(passports: &Vec<Passport>) -> usize {
    passports.iter()
        .filter(|p| p.validate_has_fields())
        .filter(|p| p.validate_field_values())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Field::{EyeColor, BirthYear, PassportID, ExpirationYear, HairColor, IssueYear, CountryID, Height};

    #[test]
    fn parse_field() {
        assert_eq!(Ok(EyeColor("gry".to_owned())), "ecl:gry".parse());
        assert_eq!(Ok(PassportID("860033327".to_owned())), "pid:860033327".parse());
        assert_eq!(Ok(ExpirationYear("2020".to_owned())), "eyr:2020".parse());
        assert_eq!(Ok(HairColor("#fffffd".to_owned())), "hcl:#fffffd".parse());
        assert_eq!(Ok(BirthYear("1937".to_owned())), "byr:1937".parse());
        assert_eq!(Ok(IssueYear("2017".to_owned())), "iyr:2017".parse());
        assert_eq!(Ok(CountryID("147".to_owned())), "cid:147".parse());
        assert_eq!(Ok(Height("183cm".to_owned())), "hgt:183cm".parse());

        assert!("invalid".parse::<Field>().is_err())
    }

    #[test]
    fn load_passports_sample() {
        let passports = load_passports("sample.txt");

        assert_eq!(4, passports.len());

        let expected = Passport {
            fields: vec![
                EyeColor("gry".to_owned()),
                PassportID("860033327".to_owned()),
                ExpirationYear("2020".to_owned()),
                HairColor("#fffffd".to_owned()),
                BirthYear("1937".to_owned()),
                IssueYear("2017".to_owned()),
                CountryID("147".to_owned()),
                Height("183cm".to_owned()),
            ]
        };

        assert_eq!(expected, passports[0]);
    }

    #[test]
    fn count_valid_sample() {
        let passports = load_passports("sample.txt");
        assert_eq!(2, count_valid(&passports));
    }

    #[test]
    fn validate_field() {
        assert!("byr:2002".parse::<Field>().unwrap().validate());
        assert!(!"byr:2003".parse::<Field>().unwrap().validate());

        assert!("hgt:60in".parse::<Field>().unwrap().validate());
        assert!("hgt:190cm".parse::<Field>().unwrap().validate());
        assert!(!"hgt:190in".parse::<Field>().unwrap().validate());
        assert!(!"hgt:190".parse::<Field>().unwrap().validate());

        assert!("hcl:#123abc".parse::<Field>().unwrap().validate());
        assert!(!"hcl:#123abz".parse::<Field>().unwrap().validate());
        assert!(!"hcl:123abc".parse::<Field>().unwrap().validate());

        assert!("ecl:brn".parse::<Field>().unwrap().validate());
        assert!(!"ecl:wat".parse::<Field>().unwrap().validate());

        assert!("pid:000000001".parse::<Field>().unwrap().validate());
        assert!(!"pid:0123456789".parse::<Field>().unwrap().validate());
    }

    #[test]
    fn count_valid_fields() {
        let passports = load_passports("sample_valid.txt");

        assert_eq!(4, passports.len());
        assert_eq!(4, count_valid_field_values(&passports));
    }

    #[test]
    fn count_invalid_fields() {
        let passports = load_passports("sample_invalid.txt");

        assert_eq!(4, passports.len());
        assert_eq!(0, count_valid_field_values(&passports));
    }
}

fn main() {
    let passports = load_passports("input.txt");

    println!("Part 1: {}", count_valid(&passports));
    println!("Part 2: {}", count_valid_field_values(&passports));
}
