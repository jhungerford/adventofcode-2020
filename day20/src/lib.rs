use core::fmt;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use itertools::Itertools;

use crate::Direction::{Bottom, Left, Right, Top};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    Top, Bottom, Left, Right
}

impl Direction {
    /// Returns whether this side is directly clockwise from the other side.
    /// For instance, `Right.is_clockwise(Top) == true`
    fn is_clockwise(&self, other: &Direction) -> bool {
        match self {
            Top if *other == Left => true,
            Bottom if *other == Right => true,
            Left if *other == Bottom => true,
            Right if *other == Top => true,
            _ => false
        }
    }

    /// Returns the number of clockwise turns to make this direction face the given direction
    fn turns(&self, to: &Direction) -> usize {
        let num = |dir: &Direction| match dir {
            Top => 1,
            Right => 2,
            Bottom => 3,
            Left => 4,
        };

        let self_num = num(self);
        let to_num = num(to);

        (to_num + 4 - self_num) % 4
    }

    /// Returns true if this direction is horizontal (left or right), or false if it's vertical.
    fn is_horizontal(&self) -> bool {
        self == &Left || self == &Right
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Side {
    num: u32,
    edge: String,
    direction: Direction,
    flipped: bool,
}

/// Tile is a numbered grid.
#[derive(Eq, PartialEq, Clone)]
pub struct Tile {
    id: i32,
    values: Vec<Vec<char>>,
}

impl Tile {
    const SIZE: usize = 10;

    /// Constructs a new Tile.
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
                    format!("{}{}", acc[2], self.values[width][width - i]),
                    format!("{}{}", acc[3], self.values[width][i]),
                    // Left
                    format!("{}{}", acc[4], self.values[width - i][0]),
                    format!("{}{}", acc[5], self.values[i][0]),
                    // Right
                    format!("{}{}", acc[6], self.values[i][width]),
                    format!("{}{}", acc[7], self.values[width - i][width]),
                ]
            }).iter().map(|edge| edge_num(edge))
            .collect()
    }

    /// Returns the side that corresponds to the given edge.
    fn edge_side(&self, edge: u32) -> Side {
        self.sides().into_iter().find(|side| side.num == edge).unwrap()
    }

    /// Returns a list of all of this tile's edges.
    fn sides(&self) -> Vec<Side> {
        self.all_edges().iter()
            .zip([(Top, false), (Top, true), (Bottom, false), (Bottom, true), (Left, false), (Left, true), (Right, false), (Right, true)].iter())
            .map(|(&edge, &(direction, flipped))| Side { num: edge, edge: num_edge(edge), direction, flipped })
            .collect()
    }

    /// Returns this tile's right edge in it's current orientation.
    fn right_edge(&self) -> u32 {
        let width = Tile::SIZE - 1;

        edge_num((0..=width).fold(String::new(), |edge, i| format!("{}{}", edge, self.values[i][width])).as_str())
    }

    /// Returns this tile's bottom edge in it's current orientation.
    fn bottom_edge(&self) -> u32 {
        let width = Tile::SIZE - 1;

        edge_num((0..=width).fold(String::new(), |edge, i| format!("{}{}", edge, self.values[width][width - i])).as_str())
    }

    /// Returns a copy of this tile oriented so the given side is facing the given direction.
    fn orient(&self, side: &Side, direction: &Direction) -> Tile {
        let mut tile = self.clone();

        for _ in 0..side.direction.turns(direction) {
            tile = tile.rotate();
        }

        tile
    }

    /// Returns a copy of this tile flipped so the given side will align with another tile.
    fn flip_mirror(&self, side: &Side, direction: &Direction) -> Tile {
        let mut tile = self.clone();

        if !side.flipped {
            if direction.is_horizontal() {
                tile = tile.flip_vertical();
            } else {
                tile = tile.flip_horizontal();
            }
        }

        tile
    }

    /// Rotates the tile 90 degrees clockwise.
    fn rotate(mut self) -> Self {
        self.values = rotate(self.values);

        self
    }

    /// Flips this tile horizontally.
    fn flip_horizontal(mut self) -> Self {
        self.values = flip_horizontal(self.values);

        self
    }

    /// Flips this tile vertically.
    fn flip_vertical(mut self) -> Self {
        self.values = flip_vertical(self.values);

        self
    }

    /// Returns the middle of this tile in the given orientation, with edges removed.
    fn without_edges(&self) -> Vec<Vec<char>> {
        (1..self.values.len() - 1)
            .map(|i| self.values[i][1..self.values.len() - 1].iter().cloned().collect())
            .collect()
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id);

        for row in &self.values {
            for col in row {
                write!(f, "{}", col);
            }
            writeln!(f);
        }

        Ok(())
    }
}

/// Rotates the given grid 90 degrees clockwise in place.
fn rotate(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();

    for x in 0..n / 2 {
        for y in x..n - x - 1 {
            let tmp = grid[x][y];
            grid[x][y] = grid[n - 1 - y][x];
            grid[n - 1 - y][x] = grid[n - 1 - x][n - 1 - y];
            grid[n - 1 - x][n - 1 - y] = grid[y][n - 1 - x];
            grid[y][n - 1 - x] = tmp;
        }
    }

    grid
}

/// Flips the given grid horizontally in place.
fn flip_horizontal(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>>{
    for line in 0..grid.len() {
        let line_len = grid[line].len();
        for i in 0..line_len / 2 {
            let tmp = grid[line][i];
            grid[line][i] = grid[line][line_len - i - 1];
            grid[line][line_len - i - 1] = tmp;
        }
    }

    grid
}

/// Flips the given grid vertically in place.
fn flip_vertical(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let size = grid.len();

    for line in 0..size / 2 {
        for col in 0..size {
            let tmp = grid[line][col];
            grid[line][col] = grid[size - line - 1][col];
            grid[size - line - 1][col] = tmp;
        }
    }

    grid
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

/// Converts an edge number like 0b0011010010 into a string like '..##.#..#.'
fn num_edge(edge: u32) -> String {
    let mut str = String::new();
    let mut num = edge;

    while str.len() < 10 {
        if num % 2 == 0 {
            str = format!(".{}", str);
        } else {
            str = format!("#{}", str);
        }

        num >>= 1;
    }

    str
}

/// TileReader is an iterator that parses a tile at a time from a file.
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
        let id_to_tile: HashMap<i32, &Tile> = self.tiles.iter()
            .map(|tile| (tile.id, tile))
            .collect();

        // Map of edge -> list of tiles that share that edge.
        let mut edge_to_tiles = HashMap::new();
        for tile in &self.tiles {
            for edge in tile.all_edges() {
                edge_to_tiles.entry(edge).or_insert(Vec::new()).push(tile.id);
            }
        }

        // Map of tile -> list of neighboring tiles
        let mut tile_neighbors = HashMap::new();
        // Map of (tile, tile) -> list of edges they share (should be the same edge, flipped).
        let mut neighbor_edges = HashMap::new();
        for (&edge, tiles) in &edge_to_tiles {
            if tiles.len() != 2 {
                continue;
            }

            let tile_a = tiles[0];
            let tile_b = tiles[1];

            tile_neighbors.entry(tile_a).or_insert(HashSet::new()).insert(tile_b);
            tile_neighbors.entry(tile_b).or_insert(HashSet::new()).insert(tile_a);

            neighbor_edges.entry((tile_a, tile_b)).or_insert(Vec::new()).push(edge);
            neighbor_edges.entry((tile_b, tile_a)).or_insert(Vec::new()).push(edge);
        }

        // Pick an arbitrary corner for the top left piece.
        let corner = tile_neighbors.iter()
            .find(|(tile, neighbors)| neighbors.len() == 2)
            .map(|(&tile, neighbors)| tile)
            .unwrap();

        let corner_tile = id_to_tile.get(&corner).unwrap();

        // List of sides of the corner tile that share neighbors and aren't flipped.
        let corner_sides: Vec<Side> = tile_neighbors.get(&corner).unwrap().iter()
            .map(|&neighbor| {
                let edges = neighbor_edges.get(&(corner, neighbor)).unwrap();
                corner_tile.sides().iter()
                    // .find(|&side| side.edge == edge && !side.flipped)
                    .find(|&side| edges.contains(&side.num) && !side.flipped)
                    .unwrap()
                    .clone()
            }).collect();

        let mut to_process = Vec::new();

        #[derive(Debug)]
        struct ToProcess {
            row: usize,
            col: usize,
            tile_id: i32,
            side: Side,
            direction: Direction,
        }

        to_process.push(ToProcess {
            row: 0, col: 0,
            tile_id: corner,
            side: right_side_for_top_left_corner(corner_sides),
            direction: Right,
        });

        // Figure out where the pieces fit.
        let dimension = (self.tiles.len() as f32).sqrt() as usize;
        let mut values = vec![vec![' '; dimension * 8]; dimension * 8];

        while let Some(piece) = to_process.pop() {

            let tile = *id_to_tile.get(&piece.tile_id).unwrap();
            let mut oriented_tile = tile.orient(&piece.side, &piece.direction);
            if piece.direction != Right {
                oriented_tile = oriented_tile.flip_mirror(&piece.side, &piece.direction);
            }

            if piece.col < dimension - 1 {
                let right_edge = oriented_tile.right_edge();

                let neighbor_id = edge_to_tiles.get(&right_edge).unwrap().iter()
                    .find(|&neighbor_id| *neighbor_id != piece.tile_id)
                    .unwrap();

                let neighbor_side = id_to_tile.get(neighbor_id).unwrap().edge_side(right_edge);

                to_process.push(ToProcess {
                    row: piece.row,
                    col: piece.col + 1,
                    tile_id: *neighbor_id,
                    side: neighbor_side,
                    direction: Left,
                });
            }

            if piece.row < dimension - 1 {
                let bottom_edge = oriented_tile.bottom_edge();

                let neighbor_id = edge_to_tiles.get(&bottom_edge).unwrap().iter()
                    .find(|&neighbor_id| *neighbor_id != piece.tile_id)
                    .unwrap();

                let neighbor_side = id_to_tile.get(neighbor_id).unwrap().edge_side(bottom_edge);

                to_process.push(ToProcess {
                    row: piece.row + 1,
                    col: piece.col,
                    tile_id: *neighbor_id,
                    side: neighbor_side,
                    direction: Top,
                });
            }

            let middle = oriented_tile.without_edges();
            for r in 0..8 {
                for c in 0..8 {
                    values[piece.row * 8 + r][piece.col * 8 + c] = middle[r][c];
                }
            }
        }

        Picture { values }
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

/// Given a list of sides with neighbors, figures out which side should go on the right
/// to place this piece in the top right corner.
fn right_side_for_top_left_corner(sides: Vec<Side>) -> Side {
    if sides[0].direction.is_clockwise(&sides[1].direction) {
        sides[1].clone()
    } else {
        sides[0].clone()
    }
}

const MONSTER: [[char; 20]; 3] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#', ' '],
    ['#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#', '#', '#'],
    [' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', '#', ' ', ' ', ' '],
];

pub struct Picture {
    values: Vec<Vec<char>>
}

impl Picture {
    /// Finds sea monsters in this picture, and counts the number of '#' values that aren't
    /// part of a sea monster.  Modifies this picture while looking for sea monsters, but
    /// returns it to it's original orientation before returning.
    pub fn roughness(&self) -> usize {
        let num_rough = self.values.iter()
            .flat_map(|line| line.iter())
            .filter(|&square| *square == '#')
            .count();

        num_rough - 15 * self.count_sea_monsters()
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
        let mut picture = Picture { values: self.values.clone() };

        let mut max = 0;

        for transform in [rotate, rotate, rotate, flip_horizontal, rotate, rotate, rotate].iter() {
            let mut count = 0;

            for row in 0..picture.values.len() - MONSTER.len() + 1 {
                for col in 0..picture.values[row].len() - MONSTER[0].len() + 1 {
                    if picture.is_sea_monster(row, col) {
                        count += 1;
                    }
                }
            }

            if count > max {
                max = count;
            }

            let mut values = transform(picture.values.clone());

            picture = Picture { values };
        }

        max
    }

    /// Checks whether there's a sea monster at the given row and column.
    fn is_sea_monster(&self, row: usize, col: usize) -> bool {
        for r in 0..MONSTER.len() {
            for c in 0..MONSTER[r].len() {
                if MONSTER[r][c] == '#' && self.values[row + r][col + c] != '#' {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn direction_turns() {
        assert_eq!(2, Bottom.turns(&Top));
        assert_eq!(2, Top.turns(&Bottom));

        assert_eq!(1, Right.turns(&Bottom));
        assert_eq!(3, Bottom.turns(&Right));

        assert_eq!(1, Top.turns(&Right));
        assert_eq!(3, Top.turns(&Left));

        assert_eq!(0, Right.turns(&Right));
    }

    #[test]
    fn grid_flip_horizontal() {
        let mut grid = Tile {
            id: 1,
            values: vec![
                vec!['.', '.', '#', '#'],
                vec!['#', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
                vec!['#', '#', '#', '#'],
            ]
        };

        let expected = Tile {
            id: 1,
            values: vec![
                vec!['#', '#', '.', '.'],
                vec!['.', '.', '#', '#'],
                vec!['.', '.', '.', '#'],
                vec!['#', '#', '#', '#'],
            ]
        };

        assert_eq!(expected, grid.flip_horizontal());
    }

    #[test]
    fn grid_flip_vertical() {
        let mut grid = Tile {
            id: 1,
            values: vec![
                vec!['.', '.', '#', '#'],
                vec!['#', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
                vec!['#', '#', '#', '#'],
            ]
        };

        let expected = Tile {
            id: 1,
            values: vec![
                vec!['#', '#', '#', '#'],
                vec!['#', '.', '.', '.'],
                vec!['#', '#', '.', '.'],
                vec!['.', '.', '#', '#'],
            ]
        };

        assert_eq!(expected, grid.flip_vertical());
    }

    #[test]
    fn grid_rotate() {
        let mut grid = Tile {
            id: 1,
            values: vec![
                vec!['.', '.', '#', '#'],
                vec!['#', '#', '.', '.'],
                vec!['#', '.', '.', '.'],
                vec!['#', '#', '#', '#'],
            ]
        };

        let expected = Tile {
            id: 1,
            values: vec![
                vec!['#', '#', '#', '.'],
                vec!['#', '.', '#', '.'],
                vec!['#', '.', '.', '#'],
                vec!['#', '.', '.', '#'],
            ]
        };

        assert_eq!(expected, grid.rotate());
    }

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
    fn test_num_edge() {
        assert_eq!("..##.#..#.", num_edge(0b0011010010));
        assert_eq!("##..#.....", num_edge(0b1100100000));
        assert_eq!("#...##..#.", num_edge(0b1000110010));
        assert_eq!("####.#...#", num_edge(0b1111010001));
        assert_eq!("##.##.###.", num_edge(0b1101101110));
        assert_eq!("##...#.###", num_edge(0b1100010111));
        assert_eq!(".#.#.#..##", num_edge(0b0101010011));
        assert_eq!("..#....#..", num_edge(0b0010000100));
        assert_eq!("###...#.#.", num_edge(0b1110001010));
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
    fn tile_sides() {
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

        let expected = vec![
            Side { num: 0b0011010010u32, edge: String::from("..##.#..#."), direction: Top, flipped: false },
            Side { num: 0b0100101100u32, edge: String::from(".#..#.##.."), direction: Top, flipped: true},
            Side { num: 0b1110011100u32, edge: String::from("###..###.."), direction: Bottom, flipped: false },
            Side { num: 0b0011100111u32, edge: String::from("..###..###"), direction: Bottom, flipped: true},
            Side { num: 0b0100111110u32, edge: String::from(".#..#####."), direction: Left, flipped: false },
            Side { num: 0b0111110010u32, edge: String::from(".#####..#."), direction: Left, flipped: true},
            Side { num: 0b0001011001u32, edge: String::from("...#.##..#"), direction: Right, flipped: false },
            Side { num: 0b1001101000u32, edge: String::from("#..##.#..."), direction: Right, flipped: true},
        ];

        assert_eq!(expected, tile.sides());
    }

    #[test]
    fn tile_orient_flip() {
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

        let expected = Tile::new(2311, vec![
            vec!['.', '.', '#', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '#', '#', '.', '.', '.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '#', '#'],
            vec!['#', '#', '.', '.', '.', '#', '.', '#', '#', '#'],
            vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '.'],
            vec!['#', '#', '#', '#', '.', '#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['#', '#', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '#', '.', '#', '.', '.', '#', '.'],
        ]);

        let bottom = Side { num: 0b0011100111u32, edge: String::from("..###..###"), direction: Bottom, flipped: true};

        assert_eq!(expected, tile.orient(&bottom, &Top).flip_horizontal());
    }

    #[test]
    fn tile_without_edges() {
        let tile = Tile::new(2311, vec![
            vec!['.', '.', '#', '#', '#', '.', '.', '#', '#', '#'],
            vec!['#', '#', '#', '.', '.', '.', '#', '.', '#', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.', '#', '.', '.', '#', '#'],
            vec!['#', '#', '.', '.', '.', '#', '.', '#', '#', '#'],
            vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '.'],
            vec!['#', '#', '#', '#', '.', '#', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#', '#', '.', '.', '#', '.'],
            vec!['#', '#', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '#', '.', '#', '.', '.', '#', '.'],
        ]);

        let expected = vec![
            vec!['#', '#', '.', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '#', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '.', '#', '.', '#', '#'],
            vec!['#', '.', '#', '#', '.', '#', '#', '#'],
            vec!['#', '#', '#', '.', '#', '.', '.', '.'],
            vec!['.', '.', '.', '#', '#', '.', '.', '#'],
            vec!['#', '.', '.', '#', '.', '.', '.', '.'],
        ];

        assert_eq!(expected, tile.without_edges());
    }


    #[test]
    fn load_sample() {
        let puzzle = Tiles::load("sample.txt");

        assert_eq!(9, puzzle.tiles.len());
    }

    #[test]
    fn corners_sample() {
        let puzzle = Tiles::load("sample.txt");

        assert_eq!(20899048083289, puzzle.corners());
    }

    #[test]
    fn to_picture_sample() {
        let puzzle = Tiles::load("sample.txt");

        let expected: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.', '#', '#', '#'],
            vec!['.', '#', '.', '#', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '.', '.', '#', '#', '#', '#', '.', '#', '#', '.'],
            vec!['#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#'],
            vec!['#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#'],
            vec!['#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '#', '.'],
            vec!['.', '.', '#', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '.', '#', '#', '#', '#', '.', '#', '#', '#'],
            vec!['.', '#', '#', '#', '#', '.', '#', '#', '#', '.', '#', '.', '.', '.', '#', '#', '#', '.', '#', '.', '.', '#', '.', '#'],
            vec!['.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '#', '.', '#', '#', '.', '#', '.', '.', '#', '.', '#', '#', '.', '.'],
            vec!['#', '#', '#', '.', '#', '.', '.', '.', '#', '.', '.', '#', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '.', '.'],
            vec!['.', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.', '.', '.', '#', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '#', '.', '.', '#', '#', '#', '.', '#', '#', '#'],
            vec!['#', '#', '.', '.', '#', '#', '.', '#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '#', '.', '#', '.', '#', '.', '.'],
            vec!['#', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#'],
            vec!['#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '#', '#'],
            vec!['#', '.', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '.'],
            vec!['#', '.', '.', '#', '.', '#', '#', '.', '.', '#', '.', '.', '#', '#', '#', '.', '#', '.', '#', '#', '.', '.', '.', '.'],
            vec!['.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.', '#', '.', '#', '#', '#', '.', '.'],
            vec!['.', '.', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '.', '.', '.', '#', '#', '#', '#', '#', '.', '#'],
            vec!['#', '#', '.', '#', '.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#'],
            vec!['#', '#', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '#', '.', '.', '#'],
            vec!['#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '.', '#', '#', '#', '#', '#', '#', '.', '.', '.'],
            vec!['#', '#', '#', '.', '.', '.', '.', '#', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '#', '.', '.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '.', '#', '#', '#', '#', '#'],
        ];

        let mut expected_all_orientations = Vec::new();
        let mut oriented = expected.clone();

        for transform in [rotate, rotate, rotate, flip_horizontal, rotate, rotate, rotate].iter() {
            expected_all_orientations.push(oriented.clone());

            oriented = transform(oriented);
        }

        assert!(expected_all_orientations.contains(&puzzle.to_picture().values));
    }

    #[test]
    fn count_sea_monsters_sample() {
        let puzzle = Tiles::load("sample.txt");
        let picture = puzzle.to_picture();

        assert_eq!(2, picture.count_sea_monsters());
    }

    #[test]
    fn count_sea_monsters_picture() {
        let picture = Picture { values: vec![
            vec!['.', '#', '#', '#', '#', '.', '.', '.', '#', '#', '#', '#', '#', '.', '.', '#', '.', '.', '.', '#', '#', '#', '.', '.'],
            vec!['#', '#', '#', '#', '#', '.', '.', '#', '.', '.', '#', '.', '#', '.', '#', '#', '#', '#', '.', '.', '#', '.', '#', '.'],
            vec!['.', '#', '.', '#', '.', '.', '.', '#', '.', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '#', '#', '.', '.'],
            vec!['#', '.', '#', '.', '#', '#', '.', '#', '#', '#', '.', '#', '.', '#', '#', '.', '#', '#', '.', '#', '#', '#', '#', '#'],
            vec!['.', '.', '#', '#', '.', '#', '#', '#', '.', '#', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '.', '#', '#'],
            vec!['.', '.', '.', '#', '.', '#', '.', '.', '#', '#', '.', '#', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '#', '#'],
            vec!['#', '.', '#', '#', '.', '#', '.', '.', '#', '.', '#', '.', '.', '#', '.', '.', '#', '#', '.', '#', '.', '#', '.', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '.', '.', '.', '.', '.', '#', '.', '.', '.', '#', '#', '#', '.', '#', '.', '.', '.'],
            vec!['#', '.', '#', '#', '#', '#', '.', '#', '.', '#', '.', '.', '.', '.', '#', '#', '.', '#', '.', '.', '#', '.', '#', '.'],
            vec!['#', '#', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.', '.', '.', '#', '#', '#', '#'],
            vec!['.', '.', '#', '.', '#', '#', '.', '.', '.', '#', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '#'],
            vec!['.', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.', '#', '#', '#', '#', '#', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['.', '.', '#', '#', '.', '#', '#', '.', '#', '#', '#', '.', '.', '.', '.', '.', '#', '.', '#', '#', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '#', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.'],
            vec!['.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '.', '.', '#'],
            vec!['#', '.', '#', '#', '#', '.', '#', '.', '.', '#', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.', '.', '.'],
            vec!['#', '.', '#', '#', '#', '.', '.', '.', '#', '.', '#', '#', '.', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '.'],
            vec!['.', '#', '#', '#', '.', '#', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#', '#', '.'],
            vec!['.', '.', '#', '#', '.', '#', '.', '.', '#', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '#', '#'],
            vec!['#', '.', '#', '.', '.', '#', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '.', '.', '#', '#', '.'],
            vec!['#', '.', '#', '#', '#', '#', '#', '.', '.', '#', '.', '#', '.', '.', '.', '#', '#', '.', '.', '#', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '.', '#', '#', '.', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '.', '#', '.', '.', '#', '#', '.', '.', '.', '#', '#', '#', '.', '#', '#'],
            vec!['#', '.', '.', '#', '#', '#', '.', '.', '.', '.', '#', '#', '.', '#', '.', '.', '.', '#', '#', '.', '#', '#', '.', '#'],
        ]};

        assert!(!picture.is_sea_monster(0, 0));
        assert!(picture.is_sea_monster(2, 2));

        assert_eq!(2, picture.count_sea_monsters());
    }

    #[test]
    fn roughness_sample() {
        let puzzle = Tiles::load("sample.txt");
        let picture = puzzle.to_picture();

        assert_eq!(273, picture.roughness());
    }
}
