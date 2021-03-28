use std::collections::HashMap;

const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

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

pub struct Square {
    peice: Peice,
    color: Color,
    position: u8,
}

impl Square {
    pub fn new(peice: Peice, color: Color, position: u8) -> Square {
        Square { peice, position, color }
    }
}

pub struct Board {
    board: Vec<Square>
}


impl Board {
    pub fn new(fen: &str) -> Board {
        let mut board: Vec<Square> = Vec::new();
        // Generate Empty Board
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

        let mut peice_map = HashMap::new();
        peice_map.insert("k", Peice::King);
        peice_map.insert("q", Peice::Queen);
        peice_map.insert("r", Peice::Rook);
        peice_map.insert("n", Peice::Knight);
        peice_map.insert("b", Peice::Bishop);
        peice_map.insert("p", Peice::Pawn);

        let mut file = 0;
        let mut rank = 7;

        // Populate via fen string 
        for c in fen.chars() {
        }

        Board { board: board }
    }

    fn 
}
