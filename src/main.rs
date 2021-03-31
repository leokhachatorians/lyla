mod board;
mod constants;
mod mover;

use board::{Board};
use constants::DEFAULT_FEN;

fn main() {
    let board: Board = Board::new(DEFAULT_FEN);
    for square in board.squares {
        println!("{}\n", square);
    }
}
