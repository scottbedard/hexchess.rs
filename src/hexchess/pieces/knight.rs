use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::get_color;

use crate::constants::{
    Color,
    HEXBOARD_GRAPH,
};

pub fn knight_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    // diagonal direction, first orthogonal direction, second orthogonal direction
    let targets: [(u8, u8, u8); 6] = [
        (1, 0, 2),
        (3, 2, 4),
        (5, 4, 6),
        (7, 6, 8),
        (9, 8, 10),
        (11, 10, 0),
    ];

    for (diagonal, orthogonal1, orthagonal2) in targets {
        let intermediate = match HEXBOARD_GRAPH[from as usize][diagonal as usize] {
            Some(index) => index,
            None => continue,
        };
    
        match HEXBOARD_GRAPH[intermediate as usize][orthogonal1 as usize] {
            Some(to) => match hexchess.board[to as usize] {
                Some(piece) => {
                    if get_color(&piece) != *color {
                        result.push(San { from, promotion: None, to }); // <- occupied by enemy
                    }
                }
                None => result.push(San { from, promotion: None, to }), // <- unoccupied
            },
            None => (), // <- no position
        };

        match HEXBOARD_GRAPH[intermediate as usize][orthagonal2 as usize] {
            Some(to) => match hexchess.board[to as usize] {
                Some(piece) => {
                    if get_color(&piece) != *color {
                        result.push(San { from, promotion: None, to });
                    }
                }
                None => result.push(San { from, promotion: None, to }),
            },
            None => (),
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    #[test]
    fn white_knight() {
        let result = Hexchess::from("1/3/5/2P1p2/9/5N5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        // ignore white pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("g8") }); // <- g8 is hostile
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("h7") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("i5") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("i4") });
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("h3") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("g3") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("e3") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("d3") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("c4") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("c5") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("d7") });
        // e8 is friendly
    }
    

    #[test]
    fn black_knight() {
        let result = Hexchess::from("1/3/5/2P1p2/9/5n5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        // ignore black pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("h7") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("i5") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("i4") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("h3") });
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g3") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("e3") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("d3") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("c4") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("c5") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("d7") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("e8") }); // <- e8 is hostile
    }
}
