use std::fs::File;
use std::io::{BufReader, BufRead};

struct Notes {
    now: i32,
    bus_ids: Vec<i32>,
}

impl Notes {
    /// Loads notes from the given file.  Notes have the current time on one line,
    /// followed by a comma-separated list of bus ids on the next.  Bus ids are either 'x'
    /// or how often that bus arrives.
    fn load(filename: &str) -> Notes {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut lines = f.lines();
        let earliest = lines.next().unwrap().unwrap().parse().unwrap();

        let bus_ids = lines.next().unwrap().unwrap().split(",")
            .filter(|&bus| bus != "x")
            .map(|bus| bus.parse().unwrap())
            .collect();

        Notes { now: earliest, bus_ids }
    }

    /// Returns the id of the earliest bus you can take multiplied by the time you need to
    /// wait to take that bus.
    fn wait(&self) -> i32 {
        let (earliest_bus_id, earliest_wait) = self.bus_ids.iter()
            .map(|&bus| (bus, (self.now as f32 / bus as f32).ceil() as i32 * bus - self.now))
            .min_by(|&x, &y| x.1.cmp(&y.1)).unwrap();

        earliest_bus_id * earliest_wait
    }
}

#[cfg(test)]
mod notes_tests {
    use super::*;

    #[test]
    fn load_notes_sample() {
        let notes = Notes::load("sample.txt");

        assert_eq!(939, notes.now);
        assert_eq!(5, notes.bus_ids.len());
    }

    #[test]
    fn wait_sample() {
        let notes = Notes::load("sample.txt");

        assert_eq!(295, notes.wait());
    }
}

#[derive(Debug)]
struct Bus {
    id: i64,
    offset: i64,
}

impl Bus {
    /// Loads busses from the given file.  The second line in the file is a comma-separated list
    /// of bus ids.
    fn load(filename: &str) -> Vec<Bus> {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut lines = f.lines();
        lines.next();

        Bus::from_line(lines.next().unwrap().unwrap().as_str())
    }

    /// Parses a list of busses from the given comma-separated line of bus ids.
    fn from_line(line: &str) -> Vec<Bus> {
        let mut busses = Vec::new();
        let mut offset = 0;

        for bus in line.split(",") {
            if bus != "x" {
                busses.push(Bus { id: bus.parse().unwrap(), offset });
            }

            offset += 1;
        }

        busses
    }
}

/// Returns the earliest timestamp where all of the listed busses depart at offsets matching
/// their position in the list.
fn earliest_depart(busses: &Vec<Bus>) -> i64 {
    // Chinese Remainder Theorem - https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving
    // Using sieving (the brute force approach to solving for t with the CRT), we find:
    // express equivalences as `t ≡ a_i % n_i`
    // test a_1, a_1+n, a_1 + 2*n_1, ... mod n_2 to find t_2
    // test t_2, t_2 + n_1 * n_2, x_2 + 2 * n_1 * n_2, ... mod n_3 to find t_3
    // repeat to find the solution.
    // Busses depart at t + offset, so the departures for 17, x, 13, 19 can be expressed as:
    // t ≡ 0 % 17
    // t ≡ (13-2) % 13
    // t ≡ (19-3) % 19
    // Which sieves to (sorting moduli descending to go quicker):
    // 16 + i * 19 % 17 = 0, i = 9
    // 187 + i * 17 * 19 % 13 = 11, i = 10
    // t = 3417

    // t ≡ a % n
    #[derive(Debug)]
    struct Mod {
        a: i64,
        n: i64,
    }

    let mut mods: Vec<Mod> = busses.iter().map(|bus| Mod {
        a: (bus.id * (bus.offset as f64 / bus.id as f64).ceil() as i64 - bus.offset) % bus.id,
        n: bus.id
    }).collect();

    mods.sort_by(|a, b| b.n.cmp(&a.n));

    let mut x = mods[0].a;
    let mut n = mods[0].n;

    for m in 1..mods.len() {
        let i = (0 .. mods[m].n).find(|i| (x + i * n) % mods[m].n == mods[m].a).unwrap();

        x += i * n;
        n *= mods[m].n;
    }

    x
}

#[cfg(test)]
mod bus_tests {
    use super::*;

    #[test]
    fn load() {
        let busses = Bus::load("sample.txt");
    }

    #[test]
    fn earliest_depart_samples() {
        assert_eq!(1068781, earliest_depart(&Bus::from_line("7,13,x,x,59,x,31,19")));
        assert_eq!(3417, earliest_depart(&Bus::from_line("17,x,13,19")));
        assert_eq!(754018, earliest_depart(&Bus::from_line("67,7,59,61")));
        assert_eq!(779210, earliest_depart(&Bus::from_line("67,x,7,59,61")));
        assert_eq!(1261476, earliest_depart(&Bus::from_line("67,7,x,59,61")));
        assert_eq!(1202161486, earliest_depart(&Bus::from_line("1789,37,47,1889")));
    }
}

fn main() {
    let notes = Notes::load("input.txt");
    println!("Part 1: {}", notes.wait());

    let busses = Bus::load("input.txt");
    println!("Part 2: {}", earliest_depart(&busses));
}
