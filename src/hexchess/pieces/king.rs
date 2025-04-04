use crate::constants::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;

use crate::hexchess::utils::{
    get_color,
    step,
};


pub fn king_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    let directions: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    for n in directions {
        let to = match step(from, n) {
            Some(to) => to,
            None => continue,
        };

        match hexchess.board[to as usize] {
            Some(piece) => {
                if get_color(&piece) != *color {
                    result.push(San { from, promotion: None, to });
                }
            },
            None => {
                result.push(San { from, promotion: None, to });
            }
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::h;
    use super::*;

    #[test]
    fn white_king() {
        let result = Hexchess::from("1/3/5/7/3P5/5K5/11/6p4/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("g7") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("g6") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("h5") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("g5") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("g4") }); // <- g4 is hostile
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("f5") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("e4") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("e5") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("d5") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("e6") });
        // e7 is friendly
    }

    #[test]
    fn black_king() {
        let result = Hexchess::from("1/3/5/7/3P5/5k5/11/6p4/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("g7") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("g6") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("h5") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("g5") });
        // g4 is friendly
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("f5") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("e4") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("e5") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("d5") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("e6") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("e7") }); // <- e7 is hostile
    }
}
