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

    check = match valid_direction(peice, to) {
        true => true,
        false => return false,
    };

    check
}

fn out_of_bounds(from: i32, to: i32) -> bool {
    let result = (from + to) as usize;
    MAILBOX[result] > 0
}

fn valid_direction(peice: Peice, direction: i32) -> bool {
    if direction == 0 { return false };
    let directions = MOVE_DIRECTIONS[peice as usize];
    directions.contains(&direction)
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
    fn check_valid_direction() {
        assert_eq!(valid_direction(Peice::Knight, -21), true);
    }

    #[test]
    fn check_invalid_direction() {
        assert_eq!(valid_direction(Peice::Knight, -99), false);
    }

    #[test]
    fn check_zero_diretion() {
        assert_eq!(valid_direction(Peice::Knight, 0), false);
    }

}
