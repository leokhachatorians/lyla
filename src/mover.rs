use crate::constants::{CAN_SLIDE, MAILBOX, MOVE_DIRECTIONS};
use crate::board::{Board, Piece, Square};


pub fn generate(square: &Square, board: &Board) -> Vec<i32> {

    let piece_value = match square.piece {
        Piece::Knight => 0,
        Piece::Bishop => 1,
        Piece::Rook => 2,
        Piece::Queen => 3,
        Piece::King => 4,
        Piece::Pawn => 5,
        Piece::Empty => return vec![]
    };

    let move_direction = MOVE_DIRECTIONS[piece_value];
    let can_slide = CAN_SLIDE[piece_value];

    let mut moves = vec![];

    for direction in move_direction {
        let mut current = square.mailbox_num;
        while MAILBOX[current as usize] != -1 {
            current += direction;

            if current < 0 || MAILBOX[current as usize] < 0 {
                break
            }

            let current_square: &Square = &board.squares[MAILBOX[current as usize] as usize];

            if current_square.piece == Piece::Empty {
                moves.push(current);

                if !can_slide {
                    break;
                }
            }
            else if current_square.color == square.color {
                 break;
            }
            else {
                moves.push(current);
                break;
            }
        }
    }
    return moves;
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{
        BISHOP_ON_A1_EMPTY_BOARD,
        DEFAULT_FEN,
        KNIGHT_ON_A1_EMPTY_BOARD,
        KNIGHT_ON_E4_EMPTY_BOARD,
        ROOK_ON_D4_EMPTY_BOARD,
        ROOK_ON_D4_PAWN_ON_D6_BLOCK,
        ROOK_ON_D4_PAWN_ON_D6_CAPTURE
    };

    #[test]
    fn check_rook_move_default_fen() {
        let board = Board::new(DEFAULT_FEN);
        let rook = &board.squares[0];

        let valid_moves = generate(rook, &board);
        assert_eq!(valid_moves, vec!());
    }

    #[test]
    fn check_knight_on_e4_empty_board() {
        let board = Board::new(KNIGHT_ON_E4_EMPTY_BOARD);
        let knight = &board.squares[28];
        let valid_moves = generate(knight, &board);
        assert_eq!(valid_moves, [34, 36, 43, 47, 63, 67, 74, 76]);
    }

    #[test]
    fn check_knight_on_a1_empty_board() {
        let board = Board::new(KNIGHT_ON_A1_EMPTY_BOARD);
        let knight = &board.squares[0];
        let valid_moves = generate(knight, &board);
        assert_eq!(valid_moves, [33, 42]);
    }

    #[test]
    fn check_bishop_on_a1_empty_board() {
        let board = Board::new(BISHOP_ON_A1_EMPTY_BOARD);
        let bishop = &board.squares[0];
        let valid_moves = generate(bishop, &board);
        assert_eq!(valid_moves, [32, 43, 54, 65, 76, 87, 98]);
    }

    #[test]
    fn check_rook_on_d4_empty_board() {
        let board = Board::new(ROOK_ON_D4_EMPTY_BOARD);
        let rook = &board.squares[27];
        let valid_moves = generate(rook, &board);
        assert_eq!(valid_moves, [44, 34, 24, 53, 52, 51, 55, 56, 57, 58, 64, 74, 84, 94]);
    }

    #[test]
    fn check_rook_on_d4_pawn_on_d6_block() {
        let board = Board::new(ROOK_ON_D4_PAWN_ON_D6_BLOCK);
        let rook = &board.squares[27];
        let valid_moves = generate(rook, &board);
        assert_eq!(valid_moves, [44, 34, 24, 53, 52, 51, 55, 56, 57, 58, 64]);
    }

    #[test]
    fn check_rook_on_d4_pawn_on_d6_capture() {
        let board = Board::new(ROOK_ON_D4_PAWN_ON_D6_CAPTURE);
        let rook = &board.squares[27];
        let valid_moves = generate(rook, &board);
        assert_eq!(valid_moves, [44, 34, 24, 53, 52, 51, 55, 56, 57, 58, 64, 74]);
    }

}
