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
    pub valid: bool,
}

impl Square {
    pub fn new(valid: bool) -> Square {
        Square {
            peice: Peice::Empty,
            color: Color::Empty,
            valid: valid
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.peice, self.color, self.valid)
    }
}

pub struct Board {
    pub squares: Vec<Square>
}


impl Board {
    pub fn generate_empty_board() -> Board {
        let mut squares: Vec<Square> = Vec::new();
        let invalid_indices = vec!(
            29, 30, 39, 40, 49, 50, 59, 60, 69, 70,
            79, 80, 89, 90
        );

        for i in 0..120 {
            if 20 < i && i < 99 && ! invalid_indices.contains(&i) {
                squares.push(Square::new(true))
            }
            else {
                squares.push(Square::new(false))
            }
        }

        Board { squares }
    }

    pub fn new(fen: &str) -> Board {
        // Top of the 10x12

        let mut file = 1; // offset by 1
        let mut rank = 9; // offset by 2
        let mut board = Board::generate_empty_board();

        // Populate via fen string 
        for c in fen.chars() {
            if c == '/' {
                rank -= 1;
                file = 1;
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


                    board.squares[(rank * 10 + file) as usize] = Square {
                        color, peice, valid: true
                    };
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
    fn check_default_white_peices() {
        let board = Board::new(DEFAULT);

        // Back rank
        for index in 21..28 {
            assert_eq!(board.squares[index].color, Color::White);
            assert_eq!(board.squares[index].valid, true);
        }
        assert_eq!(board.squares[21].peice, Peice::Rook);
        assert_eq!(board.squares[22].peice, Peice::Knight);
        assert_eq!(board.squares[23].peice, Peice::Bishop);
        assert_eq!(board.squares[24].peice, Peice::Queen);
        assert_eq!(board.squares[25].peice, Peice::King);
        assert_eq!(board.squares[26].peice, Peice::Bishop);
        assert_eq!(board.squares[27].peice, Peice::Knight);
        assert_eq!(board.squares[28].peice, Peice::Rook);

        // Pawns
        for index in 31..38 {
            assert_eq!(board.squares[index].color, Color::White);
            assert_eq!(board.squares[index].peice, Peice::Pawn);
            assert_eq!(board.squares[index].valid, true);
        }
    }

    #[test]
    fn check_default_black_peices() {
        let board = Board::new(DEFAULT);

        // Back rank
        for index in 91..98 {
            assert_eq!(board.squares[index].color, Color::Black);
            assert_eq!(board.squares[index].valid, true);
        }
        assert_eq!(board.squares[91].peice, Peice::Rook);
        assert_eq!(board.squares[92].peice, Peice::Knight);
        assert_eq!(board.squares[93].peice, Peice::Bishop);
        assert_eq!(board.squares[94].peice, Peice::Queen);
        assert_eq!(board.squares[95].peice, Peice::King);
        assert_eq!(board.squares[96].peice, Peice::Bishop);
        assert_eq!(board.squares[97].peice, Peice::Knight);
        assert_eq!(board.squares[98].peice, Peice::Rook);

        // Pawns
        for index in 81..88 {
            assert_eq!(board.squares[index].color, Color::Black);
            assert_eq!(board.squares[index].peice, Peice::Pawn);
            assert_eq!(board.squares[index].valid, true);
        }
    }

    #[test]
    fn check_default_empty_spots() {
        let board = Board::new(DEFAULT);

        for index in 41..48 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, true);
        }
        for index in 51..58 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, true);
        }
        for index in 61..68 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, true);
        }
        for index in 71..78 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, true);
        }
    }

    #[test]
    fn check_invalid_positions() {
        let board = Board::new(DEFAULT);

        for index in 0..21 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, false);
        }

        for index in 99..120 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, false);
        }

        let indices = vec!(
            29, 30, 39, 40, 49, 50, 59, 60, 69, 70,
            79, 80, 89, 90
        );

        for index in indices {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            assert_eq!(board.squares[index].valid, false);
        }

    }
}
