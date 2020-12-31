use day22::{Game, RecursiveGame};

fn main() {
    let mut game1 = Game::load("input.txt");
    println!("Part 1: {}", game1.play());

    let mut game2 = RecursiveGame::load("input.txt");
    game2.play();
    println!("Part 2: {}", game2.score());
}
