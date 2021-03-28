mod board;

use board::{Board, DEFAULT};

fn main() {
    let board: Board = Board::new(DEFAULT);
    for square in board.elements {
        println!("Peice: {:?}\nColor: {:?}\n",
            square.peice, square.color,
        );
    }
    println!("Hello, world!");
}
