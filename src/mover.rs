use crate::board::Square;
use crate::constants::MAILBOX;

enum Moves {
    Up
}

pub fn check(square: &Square, to: i32) -> bool {
    let from = square.mailbox_num;
    let check = (from + to) as usize;

    println!("{} {}", from, check);
    MAILBOX[check] > 0
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn check_cant_move_out_of_bounds() {
        let square = Square::new(61);
        assert_eq!(check(&square, -1), false);
    }
}
