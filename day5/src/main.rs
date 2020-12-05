use std::fs::File;
use std::io::{BufReader, BufRead};

/// Returns the seat id described by the string.  Seats are arranged using binary space
/// partitioning - the first 7 characters are F (front) or B (back) to find the row, and the
/// remaining 3 characters are R (right) or L (left) for the column.
fn seat_id(s: &str) -> usize {
    // Row: first 6 characters are front or back and narrow down the range.
    let mut row = 0..127;

    for r in 0..6 {
        row = match &s[r..r+1] {
            "F" => row.start .. ((row.start + row.end) as f32 / 2.0).floor() as usize,
            "B" => ((row.start + row.end) as f32 / 2.0).ceil() as usize .. row.end,
            _ => panic!("Unknown row code"),
        }
    }

    // Final F or R takes the upper or lower bound to pick out the final row.
    let final_row = match &s[6..7] {
        "F" => row.start,
        "B" => row.end,
        _ => panic!("Unknown row code"),
    };

    // Col
    let mut col = 0..7;

    for c in 0..2 {
        col = match &s[7 + c..8 + c] {
            "R" => ((col.start + col.end) as f32 / 2.0).ceil() as usize .. col.end,
            "L" => col.start .. ((col.start + col.end) as f32 / 2.0).floor() as usize,
            _ => panic!("Unknown column code"),
        }
    }

    // Final R or L picks out the column from the range.
    let final_col = match &s[9..10] {
        "R" => col.end,
        "L" => col.start,
        _ => panic!("Unknown column code"),
    };

    final_row * 8 + final_col
}

fn load_passes(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    f.lines().map(|line| line.unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(357, seat_id("FBFBBFFRLR"));
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }
}

fn main() {
    let passes = load_passes("input.txt");

    let seat_ids: Vec<usize> = passes.iter().map(|pass| seat_id(pass.as_str())).collect();

    let highest_id = *seat_ids.iter().max().unwrap();
    let lowest_id = *seat_ids.iter().min().unwrap();

    println!("Part 1: {}", highest_id);

    for seat_id in lowest_id .. highest_id {
        if ! seat_ids.contains(&seat_id) {
            println!("Part 2: {}", seat_id);
        }
    }
}
