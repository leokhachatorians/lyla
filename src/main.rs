mod board;

use board::{Board};

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


fn main() {
    let empty_board: Board = Board::new(START);
    for square in empty_board.board {
        println!("Peice: {:?}\nColor: {:?}\n",
            square.peice, square.color,
        );
    }
    println!("Hello, world!");
}
