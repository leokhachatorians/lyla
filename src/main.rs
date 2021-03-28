mod board;

use board::{Board, Peice};


fn main() {
    let empty_board: Board = Board::new();
    for square in empty_board.board {
        println!("Peice: {:?}\nColor: {:?}\nPosition: {:?}",
            square.peice, square.color, square.position
        );
    }
    println!("Hello, world!");
}
