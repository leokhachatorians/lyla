use crate::board::Peice;
use crate::constants::{
    MAILBOX, MAILBOX64,
    CAN_SLIDE, DIRECTION_TOTALS, MOVE_DIRECTIONS,
};


pub fn check(from: i32, to: i32, peice: Peice) -> bool {

    let mut check = match out_of_bounds(from, to) {
        true => true,
        false => return false,
    };

    check = match valid_peice_move(peice, to) {
        true => true,
        false => return false,
    };

    check
}

fn out_of_bounds(from: i32, to: i32) -> bool {
    let result = (from + to) as usize;
    MAILBOX[result] > 0
}

fn valid_peice_move(peice: Peice, direction: i32) -> bool {
    if direction == 0 { return false };
    let directions = MOVE_DIRECTIONS[peice as usize];
    let check = directions.iter().any(|v| v == &direction);
    check
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cant_move_out_of_bounds() {
        // 61 is square A4
        assert_eq!(out_of_bounds(61, -1), false);
    }

    #[test]
    fn check_move_in_bounds() {
        // 61 is square A4
        assert_eq!(out_of_bounds(61, 1), true);
    }

    #[test]
    fn knight_valid_move() {
        assert_eq!(valid_peice_move(Peice::Knight, -21), true);
    }

    #[test]
    fn knight_invalid_move() {
        assert_eq!(valid_peice_move(Peice::Knight, -1), false);
    }

    #[test]
    fn bishop_valid_move() {
        assert_eq!(valid_peice_move(Peice::Bishop, -11), true);
    }

    #[test]
    fn bishop_invalid_move() {
        assert_eq!(valid_peice_move(Peice::Bishop, 0), false);
    }

    #[test]
    fn rook_valid_move() {
        assert_eq!(valid_peice_move(Peice::Rook, -10), true);
    }

    #[test]
    fn rook_invalid_move() {
        assert_eq!(valid_peice_move(Peice::Rook, 21), false);
    }

}
