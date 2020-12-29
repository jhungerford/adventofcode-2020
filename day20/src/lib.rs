use std::fs::File;
use std::io::{BufReader, BufRead, Lines};
use std::collections::{HashMap, HashSet};

pub struct Tile {
    id: i32,
    values: Vec<Vec<char>>
}

impl Tile {
    const SIZE: usize = 10;

    fn new(id: i32, values: Vec<Vec<char>>) -> Tile {
        assert!(values.len() == 10 && values[0].len() == 10);

        Tile { id, values }
    }

    /// Returns all of the edges of this tile in all rotations / orientations.
    fn all_edges(&self) -> Vec<u32> {
        let width = Tile::SIZE - 1;

        (0..=width)
            .fold(vec!["".to_string(); 8], |acc, i| {
                vec![
                    // Top
                    format!("{}{}", acc[0], self.values[0][i]),
                    format!("{}{}", acc[1], self.values[0][width - i]),
                    // Bottom
                    format!("{}{}", acc[2], self.values[width][i]),
                    format!("{}{}", acc[3], self.values[width][width - i]),
                    // Left
                    format!("{}{}", acc[4], self.values[i][0]),
                    format!("{}{}", acc[5], self.values[width - i][0]),
                    // Right
                    format!("{}{}", acc[6], self.values[i][width]),
                    format!("{}{}", acc[7], self.values[width - i][width]),
                ]
            }).iter().map(|edge| edge_num(edge))
            .collect()
    }
}

/// Converts an edge string like '..##.#..#.' into a number like 0b0011010010
fn edge_num(str: &str) -> u32 {
    let mut num = 0;

    for c in str.chars() {
        num <<= 1;

        if c == '#' {
            num |= 1;
        }
    }

    num
}

struct TileReader {
    lines: Lines<BufReader<File>>
}

impl TileReader {
    fn new(lines: Lines<BufReader<File>>) -> TileReader {
        TileReader { lines }
    }
}

impl Iterator for TileReader {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let id: i32 = match self.lines.next() {
            Some(Ok(line)) if line.starts_with("Tile") => {
                line[5..line.find(":").unwrap()].parse().unwrap()
            }

            _ => return None,
        };

        let values: Vec<Vec<char>> = (0..10)
            .map(|_| self.lines.next().unwrap().unwrap().chars().collect())
            .collect();

        // Skip the blank line after each tile, if it's present.
        let _ = self.lines.next();

        return Some(Tile::new(id, values))
    }
}

pub struct Tiles {
    tiles: Vec<Tile>
}

impl Tiles {
    /// Loads a puzzle from tiles in the given file.
    pub fn load(filename: &str) -> Tiles {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        Tiles { tiles: TileReader::new(f.lines()).collect() }
    }

    /// Forms a picture by flipping and rotating tiles until they all fit together.
    /// The returned picture will have an arbitrary orientation.
    pub fn to_picture(&self) -> Picture {
        let all_edges: Vec<u32> = self.tiles.iter()
            .flat_map(Tile::all_edges)
            .collect();

        unimplemented!()
    }

    /// Returns the ids of the four corners multiplied together.
    pub fn corners(&self) -> i64 {
        // Map of edge -> list of tiles that have that edge.
        let mut edge_tiles = HashMap::new();

        for tile in &self.tiles {
            for edge in tile.all_edges() {
                edge_tiles.entry(edge).or_insert(Vec::new()).push(tile.id);
            }
        }

        // Map of tile -> neighbors
        let mut tile_neighbors = HashMap::new();

        for tiles in edge_tiles.values() {
            if tiles.len() != 2 {
                continue;
            }

            tile_neighbors.entry(tiles[0]).or_insert(HashSet::new()).insert(tiles[1]);
            tile_neighbors.entry(tiles[1]).or_insert(HashSet::new()).insert(tiles[0]);
        }

        // Corners have 2 neighbors.
        tile_neighbors.iter()
            .filter(|(tile, neighbors)| neighbors.len() == 2)
            .map(|(tile, neighbors)| *tile as i64)
            .fold(1, |product, corner| product * corner)
    }
}

pub struct Picture {
    values: Vec<Vec<char>>
}

impl Picture {
    /// Finds sea monsters in this picture, and counts the number of '#' values that aren't
    /// part of a sea monster.
    pub fn roughness(&self) -> usize {
        unimplemented!()
    }

    /// Returns the number of sea monsters in this picture's current orientation.  A sea monster
    /// looks like this:
    ///
    /// ```text
    ///                    #
    /// #    ##    ##    ###
    ///  #  #  #  #  #  #
    /// ```
    ///
    /// Empty spaces can be anything (either rough seas '#' or calm seas '.').
    fn count_sea_monsters(&self) -> usize {
        unimplemented!()
    }

    /// Returns a new picture that's the same as this picture, but rotated 90 degrees clockwise.
    fn rotate(&self) -> self {
        unimplemented!();

        self
    }

    /// Returns a new picture that's the same as this picture, but flipped horizontally.
    fn flip(&self) -> self {
        unimplemented!();

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_edge_num() {
        assert_eq!(0b0011010010, edge_num("..##.#..#."));
        assert_eq!(0b1100100000, edge_num("##..#....."));
        assert_eq!(0b1000110010, edge_num("#...##..#."));
        assert_eq!(0b1111010001, edge_num("####.#...#"));
        assert_eq!(0b1101101110, edge_num("##.##.###."));
        assert_eq!(0b1100010111, edge_num("##...#.###"));
        assert_eq!(0b0101010011, edge_num(".#.#.#..##"));
        assert_eq!(0b0010000100, edge_num("..#....#.."));
        assert_eq!(0b1110001010, edge_num("###...#.#."));
    }

    #[test]
    fn tile_edges() {
        let tile = Tile::new(2311, vec![
            vec!['.', '.', '#', '#', '.', '#', '.', '.', '#', '.'],
            vec!['#', '#', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['#', '#', '#', '#', '.', '#', '.', '.', '.', '#'],
            vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '.'],
            vec!['#', '#', '.', '.', '.', '#', '.', '#', '#', '#'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '#', '#'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '.'],
            vec!['#', '#', '#', '.', '.', '.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '#', '#', '.', '.', '#', '#', '#'],
        ]);

        let expected_edges: HashSet<u32> = vec![
            0b0011010010u32,
            0b0100101100u32,
            0b0111110010u32,
            0b0100111110u32,
            0b0011100111u32,
            0b1110011100u32,
            0b0001011001u32,
            0b1001101000u32,
        ].iter().cloned().collect();

        let actual_edges: HashSet<u32> = tile.all_edges().iter().cloned().collect();

        assert_eq!(expected_edges, actual_edges);
    }

    #[test]
    fn load_sample() {
        let puzzle = Tiles::load("sample.txt");

        assert_eq!(9, puzzle.tiles.len());
    }

    #[test]
    fn to_picture_sample() {
        let puzzle = Tiles::load("sample.txt");

        let expected: Vec<Vec<char>> = vec![
            vec!['.', '#', '.', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '.', '#', '#', '#', '#', '#'],
            vec!['#', '#', '#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '.', '#', '#', '#', '#', '#', '#', '.', '.', '.'],
            vec!['#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '.', '#'],
            vec!['#', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#'],
            vec!['.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '.', '.', '#', '#', '#', '#', '#', '.', '#'],
            vec!['.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.', '#', '.', '#', '#', '#', '.', '.'],
            vec!['.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '.', '#', '.', '#', '#', '.', '.', '.', '.'],
            vec!['#', '.', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '#', '#'],
            vec!['#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#'],
            vec!['#', '#', '.', '.', '#', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '.', '.'],
            vec!['.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '#', '.', '.', '#', '#', '#', '.', '#', '#', '#'],
            vec!['.', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '#', '#', '#', '.', '#', '#', '.'],
            vec!['#', '#', '#', '.', '#', '.', '.', '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '.', '#', '#', '.', '#', '.', '.', '#', '.', '#', '#', '.', '.'],
            vec!['.', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '.', '.', '.', '#', '#', '#', '.', '#', '.', '.', '#', '.', '#'],
            vec!['.', '.', '#', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '.', '#', '#', '#'],
            vec!['#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '#', '.'],
            vec!['#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#'],
            vec!['#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#'],
            vec!['.', '#', '.', '#', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '#', '#', '#'],
        ];

        let mut picture = puzzle.to_picture();

        assert_eq!(expected, picture.values);
    }

    #[test]
    fn corners_sample() {
        let puzzle = Tiles::load("sample.txt");

        assert_eq!(20899048083289, puzzle.corners());
    }
}
