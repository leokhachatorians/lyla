use std::fmt;

use crate::constants::DEFAULT_FEN;


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
    pub mailbox_num: i32,
    //pub valid: bool,
}

impl Square {
    pub fn new(mailbox_num: i32) -> Square {
        Square {
            peice: Peice::Empty,
            color: Color::Empty,
            mailbox_num
            //valid: valid
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.peice, self.color)
        //write!(f, "({:?}, {:?}, {:?})", self.peice, self.color, self.valid)
    }
}

pub struct Board {
    pub squares: Vec<Square>
}

impl Board {
    pub fn generate_empty_board() -> Board {
        let mut squares: Vec<Square> = Vec::new();
        let mut mailbox_num = 21;
        let mut running_count = 0;

        for _ in 0..64 {
            running_count += 1;
            squares.push(Square::new(mailbox_num));

            if running_count == 8 {
                mailbox_num += 3;
                running_count = 0;
            } else {
                mailbox_num += 1;
            }
        }

        Board { squares }
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


                    board.squares[(rank * 8 + file) as usize].peice = peice;
                    board.squares[(rank * 8 + file) as usize].color = color;
                   // = Square {
                   //     color, peice
                   //     //color, peice, valid: true
                   // };
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
        let board = Board::new(DEFAULT_FEN);

        // Back rank
        for index in 0..7 {
            assert_eq!(board.squares[index].color, Color::White);
            //assert_eq!(board.squares[index].valid, true);
        }
        assert_eq!(board.squares[0].peice, Peice::Rook);
        assert_eq!(board.squares[1].peice, Peice::Knight);
        assert_eq!(board.squares[2].peice, Peice::Bishop);
        assert_eq!(board.squares[3].peice, Peice::Queen);
        assert_eq!(board.squares[4].peice, Peice::King);
        assert_eq!(board.squares[5].peice, Peice::Bishop);
        assert_eq!(board.squares[6].peice, Peice::Knight);
        assert_eq!(board.squares[7].peice, Peice::Rook);

        // Pawns
        for index in 8..15 {
            assert_eq!(board.squares[index].color, Color::White);
            assert_eq!(board.squares[index].peice, Peice::Pawn);
            //assert_eq!(board.squares[index].valid, true);
        }
    }

    #[test]
    fn check_default_black_peices() {
        let board = Board::new(DEFAULT_FEN);

        // Back rank
        for index in 48..64 {
            assert_eq!(board.squares[index].color, Color::Black);
            //assert_eq!(board.squares[index].valid, true);
        }
        assert_eq!(board.squares[56].peice, Peice::Rook);
        assert_eq!(board.squares[57].peice, Peice::Knight);
        assert_eq!(board.squares[58].peice, Peice::Bishop);
        assert_eq!(board.squares[59].peice, Peice::Queen);
        assert_eq!(board.squares[60].peice, Peice::King);
        assert_eq!(board.squares[61].peice, Peice::Bishop);
        assert_eq!(board.squares[62].peice, Peice::Knight);
        assert_eq!(board.squares[63].peice, Peice::Rook);

        // Pawns
        for index in 48..56 {
            assert_eq!(board.squares[index].color, Color::Black);
            assert_eq!(board.squares[index].peice, Peice::Pawn);
            //assert_eq!(board.squares[index].valid, true);
        }
    }

    #[test]
    fn check_default_empty_spots() {
        let board = Board::new(DEFAULT_FEN);

        for index in 16..48 {
            assert_eq!(board.squares[index].peice, Peice::Empty);
            assert_eq!(board.squares[index].color, Color::Empty);
            //assert_eq!(board.squares[index].valid, true);
        }
        //for index in 51..58 {
        //    assert_eq!(board.squares[index].peice, Peice::Empty);
        //    assert_eq!(board.squares[index].color, Color::Empty);
        //    //assert_eq!(board.squares[index].valid, true);
        //}
        //for index in 61..68 {
        //    assert_eq!(board.squares[index].peice, Peice::Empty);
        //    assert_eq!(board.squares[index].color, Color::Empty);
        //    //assert_eq!(board.squares[index].valid, true);
        //}
        //for index in 71..78 {
        //    assert_eq!(board.squares[index].peice, Peice::Empty);
        //    assert_eq!(board.squares[index].color, Color::Empty);
        //    //assert_eq!(board.squares[index].valid, true);
        //}
    }

//    #[test]
//    fn check_invalid_positions() {
//        let board = Board::new(DEFAULT_FEN);
//
//        for index in 0..21 {
//            assert_eq!(board.squares[index].peice, Peice::Empty);
//            assert_eq!(board.squares[index].color, Color::Empty);
//            assert_eq!(board.squares[index].valid, false);
//        }
//
//        for index in 99..120 {
//            assert_eq!(board.squares[index].peice, Peice::Empty);
//            assert_eq!(board.squares[index].color, Color::Empty);
//            assert_eq!(board.squares[index].valid, false);
//        }
//
//        let indices = vec!(
//            29, 30, 39, 40, 49, 50, 59, 60, 69, 70,
//            79, 80, 89, 90
//        );
//
//        for index in indices {
//            assert_eq!(board.squares[index].peice, Peice::Empty);
//            assert_eq!(board.squares[index].color, Color::Empty);
//            assert_eq!(board.squares[index].valid, false);
//        }
//
//    }
}
