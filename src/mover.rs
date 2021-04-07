use crate::board::{Board, Peice};
use crate::constants::{
    MAILBOX, MAILBOX64,
    CAN_SLIDE, DIRECTION_TOTALS, MOVE_DIRECTIONS,
};

pub enum MoveResult {
    Moved,
    Captured,
    Invalid,
}


pub fn move_(from: i32, to: i32, peice: Peice, board: &Board) -> MoveResult {

    let mut check = match out_of_bounds(from, to) {
        true => MoveResult::Moved,
        false => return MoveResult::Invalid,
    };

    check = match valid_direction(peice as usize, to) {
        true => MoveResult::Moved,
        false => return MoveResult::Invalid,
    };

    check
}

fn out_of_bounds(from: i32, to: i32) -> bool {
    let result = (from + to) as usize;
    MAILBOX[result] > 0
}

fn valid_direction(peice: usize, direction: i32) -> bool {
    if direction == 0 { return false };

    let directions = MOVE_DIRECTIONS[peice];
    directions.contains(&direction)
}

fn get_range_between(from: i32, to: i32) -> Vec<i32> {
    let distance: i32 = to - from;
    let mut offset;
    let mut steps;
    let mut indexes = Vec::new();

    if distance <= 7 && distance >= -7 {
        offset = 1;
    } else if distance % 9 == 0 {
        offset = 9;
    } else if distance % 10 == 0 {
        offset = 10;
    } else if distance % 11 == 0 {
        offset = 11;
    } else {
        panic!("This distance doesnt make sense");
    }

    steps = distance / offset;

    if distance < 0 {
        offset = -offset;
        steps = -steps;
    }

    let mut start = from + offset;

    for _ in 1..steps {
        indexes.push(start);
        start += offset;
    }
    indexes
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
        assert_eq!(valid_direction(Peice::Knight as usize, -21), true);
    }

    #[test]
    fn check_invalid_direction() {
        assert_eq!(valid_direction(Peice::Knight as usize, -99), false);
    }

    #[test]
    fn check_zero_diretion() {
        assert_eq!(valid_direction(Peice::Knight as usize, 0), false);
    }

    #[test]
    fn check_between_points_horizontal() {
        // Right
        assert_eq!(get_range_between(42, 47), vec![43, 44, 45, 46]);

        // Left
        assert_eq!(get_range_between(47, 42), vec![46, 45, 44, 43]);
    }

    #[test]
    fn check_between_points_vertical() {
        // Up
        assert_eq!(get_range_between(45, 85), vec![55, 65, 75]);

        //Down
        assert_eq!(get_range_between(90, 40), vec![80, 70, 60, 50]);
    }

    #[test]
    fn check_between_points_diagonal() {
        // Up to the right
        assert_eq!(get_range_between(93, 57), vec![84, 75, 66]);

        // Down to the left
        assert_eq!(get_range_between(25, 52), vec![34, 43]);

        // Up to the left
        assert_eq!(get_range_between(45, 85), vec![55, 65, 75]);

        // Down to right
        assert_eq!(get_range_between(23, 67), vec![34, 45, 56]);
    }
}
