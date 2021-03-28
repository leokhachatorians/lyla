mod board;

use board::{Board, DEFAULT};

fn main() {
    let board: Board = Board::new(DEFAULT);
    for square in board.squares {
        println!("{}\n", square);
    }
}
