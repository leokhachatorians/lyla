#[derive(Debug)]
enum Peice {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
    Empty,
}

#[derive(Debug)]
enum Color {
    White,
    Black,
    Empty,
}

struct Square {
    peice: Peice,
    color: Color,
    position: u8,
}

impl Square {
    pub fn new(peice: Peice, color: Color, position: u8) -> Square {
        Square { peice, position, color }
    }
}

struct Board {
    board: Vec<Square>
}

impl Board {
    pub fn new() -> Board {
        let mut board: Vec<Square> = Vec::new();
        for position in 0..64 {
            println!("{}", position);
            board.push(
                Square {
                    peice: Peice::Empty,
                    color: Color::Empty,
                    position: position
                }
            )
        }
        Board { board: board }
    }
}

fn main() {
    let empty_board: Board = Board::new();
    for square in empty_board.board {
        println!("Peice: {:?}\nColor: {:?}\nPosition: {:?}",
            square.peice, square.color, square.position
        );
    }
    println!("Hello, world!");
}
