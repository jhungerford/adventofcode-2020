use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use crate::Winner::{NoPlayer, Player1, Player2};

#[derive(Debug, Eq, PartialEq)]
pub enum Winner {
    Player1,
    Player2,
    NoPlayer,
}

struct PlayerReader<B> where B: BufRead {
    lines: Lines<B>
}

impl Iterator for PlayerReader<BufReader<File>> {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) if line.starts_with("Player") => {},
            _ => panic!("Player section must start with player number."),
        }

        let mut cards: Vec<i32> = Vec::new();
        let mut next_line = self.lines.next().unwrap().unwrap();
        while !next_line.trim().is_empty() {
            cards.push(next_line.parse().unwrap());

            let maybe_next_line = self.lines.next();
            if maybe_next_line.is_none() {
                return Some(cards);
            }

            next_line = maybe_next_line.unwrap().unwrap();
        }

        Some(cards)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    player1: Vec<i32>,
    player2: Vec<i32>,
}

impl Game {
    /// Loads a game of combat from the given file.
    pub fn load(filename: &str) -> Game {
        // File is two sections separated by an empty line of 'Player #:', then numbers.
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut reader = PlayerReader { lines: f.lines() };

        Game {
            player1: reader.next().unwrap(),
            player2: reader.next().unwrap(),
        }
    }

    /// Plays a game of combat, and returns the winning player's score.  Modifies this game.
    pub fn play(&mut self) -> i32 {
        while !self.is_over() {
            self.play_round();
        }

        if self.player1.is_empty() {
            score(&self.player2)
        } else {
            score(&self.player1)
        }
    }

    /// Returns whether this game is complete.  Combat ends when one player has all of the cards.
    fn is_over(&self) -> bool {
        self.player1.is_empty() || self.player2.is_empty()
    }

    /// Plays one round of combat, modifying this game.
    fn play_round(&mut self) {
        let player1_card = self.player1.remove(0);
        let player2_card = self.player2.remove(0);

        if player1_card > player2_card {
            self.player1.push(player1_card);
            self.player1.push(player2_card);
        } else {
            self.player2.push(player2_card);
            self.player2.push(player1_card);
        }
    }
}

pub struct RecursiveGame {
    player1: Vec<i32>,
    player2: Vec<i32>,
    seen: HashSet<String>,
}

impl RecursiveGame {
    /// Loads a game of recursive combat from the given file.
    pub fn load(filename: &str) -> RecursiveGame {
        // File is two sections separated by an empty line of 'Player #:', then numbers.
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let mut reader = PlayerReader { lines: f.lines() };

        RecursiveGame {
            player1: reader.next().unwrap(),
            player2: reader.next().unwrap(),
            seen: HashSet::new(),
        }
    }

    /// Plays a game of recursive combat, and returns the winner.  Modifies this game.
    pub fn play(&mut self) -> Winner {
        // 1. If there was a previous round in this game that had exactly the same cards in the same
        //    order in the same player's decks, player 1 wins instantly.
        // 2. Players draw top card
        // 3. If both players have at least as many cards in their deck as the card value,
        //    winner is determined by recursive combat.
        // 4. Otherwise, winner is the player with the higher-value card.
        // Winner places the two cards at the bottom of the deck, with their card on top.
        // Recursive combat:
        // 1. Players form new deck by copying the next n-cards in their deck, where n is the
        //    card they just drew.
        // 2. Game played with the formed deck.

        let mut winner = self.winner();
        self.seen.insert(self.state());

        while winner == NoPlayer {
            let player1_card = self.player1.remove(0) as usize;
            let player2_card = self.player2.remove(0) as usize;

            // Settle the winner by recursive combat if both players have enough cards.
            let should_recurse = self.player1.len() >= player1_card && self.player2.len() >= player2_card;

            let round_winner = if should_recurse {
                self.fork(player1_card, player2_card).play()
            } else if player1_card > player2_card {
                Player1
            } else {
                Player2
            };

            match round_winner {
                Player1 => {
                    self.player1.push(player1_card as i32);
                    self.player1.push(player2_card as i32);
                }

                Player2 => {
                    self.player2.push(player2_card as i32);
                    self.player2.push(player1_card as i32);
                }

                _ => unreachable!("Round finished without a winner"),
            }

            winner = self.winner();
            self.seen.insert(self.state());
        }

        winner
    }

    /// Returns a copy of this game with player1 cards from player1's hand, and player2 cards
    /// from player2's hand.
    fn fork(&self, player1: usize, player2: usize) -> RecursiveGame {
        RecursiveGame {
            player1: self.player1[0..player1].to_vec(),
            player2: self.player2[0..player2].to_vec(),
            seen: HashSet::new(),
        }
    }

    /// Returns the winner based on this game's current state.
    fn winner(&self) -> Winner {
        if self.player2.is_empty() || self.seen.contains(&self.state()) {
            Player1
        } else if self.player1.is_empty() {
            Player2
        } else {
            NoPlayer
        }
    }

    /// Returns a String representing this game's unique state.
    fn state(&self) -> String {
        // Cards are 1 or 2 digits, so allocate enough space for card numbers, spaces, and player separator
        let mut s = String::with_capacity(self.player1.len() * 3 + self.player2.len() * 3 + 1);

        for card in &self.player1 {
            s.push_str(card.to_string().as_str());
            s.push(' ');
        }

        s.push('|');

        for card in &self.player2 {
            s.push(' ');
            s.push_str(card.to_string().as_str());
        }

        s
    }

    /// Returns the winning player's score, or player1's score if the game is still in progress.
    pub fn score(&self) -> i32 {
        if self.player1.is_empty() {
            score(&self.player2)
        } else {
            score(&self.player1)
        }
    }
}

/// Computes the score for the given hand.
fn score(hand: &Vec<i32>) -> i32 {
    let hand_len = hand.len();

    hand.iter().enumerate().fold(0, |score, (i, &card)| score + card * (hand_len - i) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_sample() {
        let game = Game::load("sample.txt");

        assert_eq!(vec![9, 2, 6, 3, 1], game.player1);
        assert_eq!(vec![5, 8, 4, 7, 10], game.player2);
    }

    #[test]
    fn play_sample() {
        let mut game = Game::load("sample.txt");
        assert_eq!(306, game.play());
    }

    #[test]
    fn play_round_sample() {
        let mut game = Game::load("sample.txt");

        let round2_game = Game {
            player1: vec![2, 6, 3, 1, 9, 5],
            player2: vec![8, 4, 7, 10],
        };

        let round3_game = Game {
            player1: vec![6, 3, 1, 9, 5],
            player2: vec![4, 7, 10, 8, 2],
        };

        let round4_game = Game {
            player1: vec![3, 1, 9, 5, 6, 4],
            player2: vec![7, 10, 8, 2],
        };

        game.play_round();
        assert_eq!(round2_game, game);

        game.play_round();
        assert_eq!(round3_game, game);

        game.play_round();
        assert_eq!(round4_game, game);
    }

    #[test]
    fn recursive_state_sample() {
        let game = RecursiveGame::load("sample.txt");

        assert_eq!("9 2 6 3 1 | 5 8 4 7 10", game.state());
    }

    #[test]
    fn play_recursive_sample() {
        let mut game = RecursiveGame::load("sample.txt");
        assert_eq!(Player2, game.play());
        assert_eq!(291, game.score());
    }

    #[test]
    fn play_recursive_infinite_sample() {
        let mut game = RecursiveGame::load("sample_infinite.txt");
        assert_eq!(Player1, game.play());
    }
}