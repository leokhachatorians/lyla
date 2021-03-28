//use std::cmp::PartialEq;
use std::fmt;


pub const DEFAULT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";


#[derive(Debug, PartialEq)]
pub enum Peice {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
    Empty,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    White,
    Black,
    Empty,
}

pub struct Square {
    pub peice: Peice,
    pub color: Color,
}

impl Square {
    pub fn new(peice: Peice, color: Color) -> Square {
        Square { peice, color }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.peice, self.color)
    }
}

pub struct Board {
    pub elements: Vec<Square>
}


impl Board {
    pub fn generate_empty_board() -> Board {
        let mut elements: Vec<Square> = Vec::new();

        for _ in 0..64 {
        elements.push(
                Square {
                    peice: Peice::Empty,
                    color: Color::Empty,
                }
            )
        }

        Board { elements }
    }

    pub fn new(fen: &str) -> Board {
        let mut file = 0;
        let mut rank = 7;
        let mut board = Board::generate_empty_board();

        // Populate via fen string 
        for c in fen.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            }
            else if c == ' ' {
                break;
            }
            else {
                if c.is_ascii_digit() {
                    file += c.to_digit(10).unwrap();
                }
                else {
                    let color = if c.is_uppercase() { Color::White } else { Color::Black };
                    let peice = match c.to_ascii_lowercase() {
                        'k' => Peice::King,
                        'q' => Peice::Queen,
                        'r' => Peice::Rook,
                        'n' => Peice::Knight,
                        'b' => Peice::Bishop,
                        'p' => Peice::Pawn,
                        _ => panic!("yeah idk what happened")
                    };

                    board.elements[(rank * 8 + file) as usize] = Square { color, peice };
                    file += 1;
                }
            }
        }

        board
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_default_positions() {
        let board = Board::new(DEFAULT);

        // Check that White owns these peices
        for index in 0..16 {
            assert_eq!(board.elements[index].color, Color::White);
        }

        // Check White has proper peices
        assert_eq!(board.elements[0].peice, Peice::Rook);
        assert_eq!(board.elements[1].peice, Peice::Knight);
        assert_eq!(board.elements[2].peice, Peice::Bishop);
        assert_eq!(board.elements[3].peice, Peice::Queen);
        assert_eq!(board.elements[4].peice, Peice::King);
        assert_eq!(board.elements[5].peice, Peice::Bishop);
        assert_eq!(board.elements[6].peice, Peice::Knight);
        assert_eq!(board.elements[7].peice, Peice::Rook);

        for index in 8..16 {
            assert_eq!(board.elements[index].peice, Peice::Pawn);
        }

        // Check that these squares are empty
        for index in 16..48 {
            assert_eq!(board.elements[index].peice, Peice::Empty);
            assert_eq!(board.elements[index].color, Color::Empty);
        }

        // Check that Black owns these peices
        for index in 48..64 {
            assert_eq!(board.elements[index].color, Color::Black);
        }

        // Check Black as proper peices
        for index in 48..56 {
            assert_eq!(board.elements[index].peice, Peice::Pawn);
        }

        assert_eq!(board.elements[56].peice, Peice::Rook);
        assert_eq!(board.elements[57].peice, Peice::Knight);
        assert_eq!(board.elements[58].peice, Peice::Bishop);
        assert_eq!(board.elements[59].peice, Peice::Queen);
        assert_eq!(board.elements[60].peice, Peice::King);
        assert_eq!(board.elements[61].peice, Peice::Bishop);
        assert_eq!(board.elements[62].peice, Peice::Knight);
        assert_eq!(board.elements[63].peice, Peice::Rook);
    }
}
