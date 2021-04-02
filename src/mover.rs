use crate::board::Square;
use crate::constants::{
    MAILBOX, MAILBOX64,
    CAN_SLIDE, DIRECTION_TOTALS, MOVE_DIRECTIONS,
};


pub fn check(from: i32, to: i32) -> bool {
    let check = (from + to) as usize;

    println!("{} {}", from, check);
    MAILBOX[check] > 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_cant_move_out_of_bounds() {
        // 61 is square A4
        assert_eq!(check(61, -1), false);
    }

    #[test]
    fn check_move_in_bounds() {
        // 61 is square A4
        assert_eq!(check(61, 1), true);
    }
}
