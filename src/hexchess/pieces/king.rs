use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::utils::get_color;

use crate::constants::{
    HEXBOARD_GRAPH,
    Color,
    San,
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
        let result = Hexchess::from("1/3/5/7/3p1P3/5K5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([
            San { from: hex!("f6"), promotion: None, to: hex!("f7") }, // @todo: f7 is self-check
            // g7 is friendly
            San { from: hex!("f6"), promotion: None, to: hex!("g6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h5") },
            San { from: hex!("f6"), promotion: None, to: hex!("g5") },
            San { from: hex!("f6"), promotion: None, to: hex!("g4") },
            San { from: hex!("f6"), promotion: None, to: hex!("f5") },
            San { from: hex!("f6"), promotion: None, to: hex!("e4") },
            San { from: hex!("f6"), promotion: None, to: hex!("e5") },
            San { from: hex!("f6"), promotion: None, to: hex!("d5") },
            San { from: hex!("f6"), promotion: None, to: hex!("e6") },
            San { from: hex!("f6"), promotion: None, to: hex!("e7") }, // <- e7 is hostile
        ].iter()));
    }

    #[test]
    fn black_king() {
        let result = Hexchess::from("1/3/5/7/9/5k5/11/4P1p4/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([
            San { from: hex!("f6"), promotion: None, to: hex!("f7") },
            San { from: hex!("f6"), promotion: None, to: hex!("g7") },
            San { from: hex!("f6"), promotion: None, to: hex!("g6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h5") },
            San { from: hex!("f6"), promotion: None, to: hex!("g5") },
            // g4 is friendly
            San { from: hex!("f6"), promotion: None, to: hex!("f5") }, // @todo: f5 is self-check
            San { from: hex!("f6"), promotion: None, to: hex!("e4") }, // <- e4 is hostile
            San { from: hex!("f6"), promotion: None, to: hex!("e5") },
            San { from: hex!("f6"), promotion: None, to: hex!("d5") },
            San { from: hex!("f6"), promotion: None, to: hex!("e6") },
            San { from: hex!("f6"), promotion: None, to: hex!("e7") },
        ].iter()));
    }
}
