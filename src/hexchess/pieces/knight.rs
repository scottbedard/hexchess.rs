use crate::constants::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;

use crate::hexchess::utils::{
    get_color,
    step,
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
        let intermediate = match step(from, diagonal) {
            Some(index) => index,
            None => continue,
        };

        match knight_steps(hexchess, from, intermediate, orthogonal1, color) {
            Some(to) => result.push(to),
            None => (),
        };

        match knight_steps(hexchess, from, intermediate, orthagonal2, color) {
            Some(to) => result.push(to),
            None => (),
        };
    }

    result
}

fn knight_steps(hexchess: &Hexchess, from: u8, intermediate: u8, orthogonal: u8, color: &Color) -> Option<San> {
    match step(intermediate, orthogonal) {
        Some(to) => match hexchess.board[to as usize] {
            Some(piece) => match get_color(&piece) != *color {
                true => Some(San { from, promotion: None, to }),
                false => None,
            },
            None => Some(San { from, promotion: None, to }),
        },
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::h;
    use super::*;

    #[test]
    fn white_knight() {
        let result = Hexchess::from("1/3/5/2P1p2/9/5N5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("g8") }); // <- g8 is hostile
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("h7") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("i5") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("i4") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("h3") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("g3") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("e3") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("d3") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("c4") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("c5") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("d7") });
        // e8 is friendly
    }

    #[test]
    fn black_knight() {
        let result = Hexchess::from("1/3/5/2P1p2/9/5n5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("h7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("i5") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("i4") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("h3") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("g3") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("e3") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("d3") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("c4") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("c5") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("d7") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("e8") }); // <- e8 is hostile
    }
}
