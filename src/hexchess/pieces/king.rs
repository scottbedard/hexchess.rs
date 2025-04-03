use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::get_color;

use crate::constants::{
    Color,
    HEXBOARD_GRAPH,
};

pub fn king_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    let directions: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    for n in directions {
        let to = match HEXBOARD_GRAPH[from as usize][n as usize] {
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
    use crate::hex;
    use super::*;

    #[test]
    fn white_king() {
        let result = Hexchess::from("1/3/5/7/3P5/5K5/11/6p4/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        // ignore initial pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("g7") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("h5") });
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("g4") }); // <- g4 is hostile
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("e4") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("d5") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        // e7 is friendly
    }

    #[test]
    fn black_king() {
        let result = Hexchess::from("1/3/5/7/3P5/5k5/11/6p4/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("g7") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("h5") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        // g4 is friendly
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("e4") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("d5") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("e7") }); // <- e7 is hostile
    }
}
