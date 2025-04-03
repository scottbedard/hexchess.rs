use crate::constants::Color;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;
use crate::hexchess::utils::walk;

pub fn straight_line_moves_unsafe(
    hexchess: &Hexchess,
    from: &u8,
    color: &Color,
    directions: &[u8],
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    for n in directions {
        let path = walk(hexchess, *from, *n, color);

        for to in path {
            result.push(San {
                from: *from,
                promotion: None,
                to
            });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    #[test]
    fn white_queen() {
        let result = Hexchess::from("p/3/5/7/9/P4Q5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("f8") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("f9") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("f10") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("f11") }); // <- f11 is hostile
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g7") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("h8") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("h6") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("i6") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("k6") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("l6") });
        assert_eq!(result[12], San { from: hex!("f6"), promotion: None, to: hex!("h5") });
        assert_eq!(result[13], San { from: hex!("f6"), promotion: None, to: hex!("k4") });
        assert_eq!(result[14], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        assert_eq!(result[15], San { from: hex!("f6"), promotion: None, to: hex!("h4") });
        assert_eq!(result[16], San { from: hex!("f6"), promotion: None, to: hex!("i3") });
        assert_eq!(result[17], San { from: hex!("f6"), promotion: None, to: hex!("k2") });
        assert_eq!(result[18], San { from: hex!("f6"), promotion: None, to: hex!("l1") });
        assert_eq!(result[19], San { from: hex!("f6"), promotion: None, to: hex!("g4") });
        assert_eq!(result[20], San { from: hex!("f6"), promotion: None, to: hex!("h2") });
        assert_eq!(result[21], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[22], San { from: hex!("f6"), promotion: None, to: hex!("f4") });
        assert_eq!(result[23], San { from: hex!("f6"), promotion: None, to: hex!("f3") });
        assert_eq!(result[24], San { from: hex!("f6"), promotion: None, to: hex!("f2") });
        assert_eq!(result[25], San { from: hex!("f6"), promotion: None, to: hex!("f1") });
        assert_eq!(result[26], San { from: hex!("f6"), promotion: None, to: hex!("e4") });
        assert_eq!(result[27], San { from: hex!("f6"), promotion: None, to: hex!("d2") });
        assert_eq!(result[28], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[29], San { from: hex!("f6"), promotion: None, to: hex!("d4") });
        assert_eq!(result[30], San { from: hex!("f6"), promotion: None, to: hex!("c3") });
        assert_eq!(result[31], San { from: hex!("f6"), promotion: None, to: hex!("b2") });
        assert_eq!(result[32], San { from: hex!("f6"), promotion: None, to: hex!("a1") });
        assert_eq!(result[33], San { from: hex!("f6"), promotion: None, to: hex!("d5") });
        assert_eq!(result[34], San { from: hex!("f6"), promotion: None, to: hex!("b4") });
        assert_eq!(result[35], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        assert_eq!(result[36], San { from: hex!("f6"), promotion: None, to: hex!("d6") });
        assert_eq!(result[37], San { from: hex!("f6"), promotion: None, to: hex!("c6") });
        assert_eq!(result[38], San { from: hex!("f6"), promotion: None, to: hex!("b6") });
            // a6 is friendly
        assert_eq!(result[39], San { from: hex!("f6"), promotion: None, to: hex!("e7") });
        assert_eq!(result[40], San { from: hex!("f6"), promotion: None, to: hex!("d8") });
    }

    #[test]
    fn black_queen() {
        let result = Hexchess::from("p/3/5/7/9/P4q5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        // ignore black pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("f8") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("f9") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("f10") });
        // f11 is friendly
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g7") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("h8") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("h6") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("i6") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("k6") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("l6") });
        assert_eq!(result[12], San { from: hex!("f6"), promotion: None, to: hex!("h5") });
        assert_eq!(result[13], San { from: hex!("f6"), promotion: None, to: hex!("k4") });
        assert_eq!(result[14], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        assert_eq!(result[15], San { from: hex!("f6"), promotion: None, to: hex!("h4") });
        assert_eq!(result[16], San { from: hex!("f6"), promotion: None, to: hex!("i3") });
        assert_eq!(result[17], San { from: hex!("f6"), promotion: None, to: hex!("k2") });
        assert_eq!(result[18], San { from: hex!("f6"), promotion: None, to: hex!("l1") });
        assert_eq!(result[19], San { from: hex!("f6"), promotion: None, to: hex!("g4") });
        assert_eq!(result[20], San { from: hex!("f6"), promotion: None, to: hex!("h2") });
        assert_eq!(result[21], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[22], San { from: hex!("f6"), promotion: None, to: hex!("f4") });
        assert_eq!(result[23], San { from: hex!("f6"), promotion: None, to: hex!("f3") });
        assert_eq!(result[24], San { from: hex!("f6"), promotion: None, to: hex!("f2") });
        assert_eq!(result[25], San { from: hex!("f6"), promotion: None, to: hex!("f1") });
        assert_eq!(result[26], San { from: hex!("f6"), promotion: None, to: hex!("e4") });
        assert_eq!(result[27], San { from: hex!("f6"), promotion: None, to: hex!("d2") });
        assert_eq!(result[28], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[29], San { from: hex!("f6"), promotion: None, to: hex!("d4") });
        assert_eq!(result[30], San { from: hex!("f6"), promotion: None, to: hex!("c3") });
        assert_eq!(result[31], San { from: hex!("f6"), promotion: None, to: hex!("b2") });
        assert_eq!(result[32], San { from: hex!("f6"), promotion: None, to: hex!("a1") });
        assert_eq!(result[33], San { from: hex!("f6"), promotion: None, to: hex!("d5") });
        assert_eq!(result[34], San { from: hex!("f6"), promotion: None, to: hex!("b4") });
        assert_eq!(result[35], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        assert_eq!(result[36], San { from: hex!("f6"), promotion: None, to: hex!("d6") });
        assert_eq!(result[37], San { from: hex!("f6"), promotion: None, to: hex!("c6") });
        assert_eq!(result[38], San { from: hex!("f6"), promotion: None, to: hex!("b6") });
        assert_eq!(result[39], San { from: hex!("f6"), promotion: None, to: hex!("a6") }); // <- a6 is hostile
        assert_eq!(result[40], San { from: hex!("f6"), promotion: None, to: hex!("e7") });
        assert_eq!(result[41], San { from: hex!("f6"), promotion: None, to: hex!("d8") });
    }

    #[test]
    fn white_bishop() {
        let result = Hexchess::from("1/3/5/1p3P1/9/5B5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        // ignore pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("g7") });
        // h8 is friendly
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("h5") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("k4") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("g4") });
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("h2") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("e4") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("d2") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("d5") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("b4") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("e7") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("d8") }); // <- d8 is hostile
    }

    #[test]
    fn black_bishop() {
        let result = Hexchess::from("1/3/5/1p3P1/9/5b5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        // ignore pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("g7") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("h8") }); // <- h8 is hostile
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("h5") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("k4") });
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g4") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("h2") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("e4") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("d2") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("d5") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("b4") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("e7") });
        // d8 is friendly
    }

    #[test]
    fn white_rook() {
        let result = Hexchess::from("p/3/5/7/9/P4R5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("f8") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("f9") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("f10") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("f11") }); // <- f11 is hostile
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("h6") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("i6") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("k6") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("l6") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("h4") });
        assert_eq!(result[12], San { from: hex!("f6"), promotion: None, to: hex!("i3") });
        assert_eq!(result[13], San { from: hex!("f6"), promotion: None, to: hex!("k2") });
        assert_eq!(result[14], San { from: hex!("f6"), promotion: None, to: hex!("l1") });
        assert_eq!(result[15], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[16], San { from: hex!("f6"), promotion: None, to: hex!("f4") });
        assert_eq!(result[17], San { from: hex!("f6"), promotion: None, to: hex!("f3") });
        assert_eq!(result[18], San { from: hex!("f6"), promotion: None, to: hex!("f2") });
        assert_eq!(result[19], San { from: hex!("f6"), promotion: None, to: hex!("f1") });
        assert_eq!(result[20], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[21], San { from: hex!("f6"), promotion: None, to: hex!("d4") });
        assert_eq!(result[22], San { from: hex!("f6"), promotion: None, to: hex!("c3") });
        assert_eq!(result[23], San { from: hex!("f6"), promotion: None, to: hex!("b2") });
        assert_eq!(result[24], San { from: hex!("f6"), promotion: None, to: hex!("a1") });
        assert_eq!(result[25], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        assert_eq!(result[26], San { from: hex!("f6"), promotion: None, to: hex!("d6") });
        assert_eq!(result[27], San { from: hex!("f6"), promotion: None, to: hex!("c6") });
        assert_eq!(result[28], San { from: hex!("f6"), promotion: None, to: hex!("b6") });
        // a6 is friendly
    }

    #[test]
    fn black_rook() {
        let result = Hexchess::from("p/3/5/7/9/P4r5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        // ignore black pawn move at index 0
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("f8") });
        assert_eq!(result[3], San { from: hex!("f6"), promotion: None, to: hex!("f9") });
        assert_eq!(result[4], San { from: hex!("f6"), promotion: None, to: hex!("f10") });
        // f11 is friendly
        assert_eq!(result[5], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
        assert_eq!(result[6], San { from: hex!("f6"), promotion: None, to: hex!("h6") });
        assert_eq!(result[7], San { from: hex!("f6"), promotion: None, to: hex!("i6") });
        assert_eq!(result[8], San { from: hex!("f6"), promotion: None, to: hex!("k6") });
        assert_eq!(result[9], San { from: hex!("f6"), promotion: None, to: hex!("l6") });
        assert_eq!(result[10], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        assert_eq!(result[11], San { from: hex!("f6"), promotion: None, to: hex!("h4") });
        assert_eq!(result[12], San { from: hex!("f6"), promotion: None, to: hex!("i3") });
        assert_eq!(result[13], San { from: hex!("f6"), promotion: None, to: hex!("k2") });
        assert_eq!(result[14], San { from: hex!("f6"), promotion: None, to: hex!("l1") });
        assert_eq!(result[15], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[16], San { from: hex!("f6"), promotion: None, to: hex!("f4") });
        assert_eq!(result[17], San { from: hex!("f6"), promotion: None, to: hex!("f3") });
        assert_eq!(result[18], San { from: hex!("f6"), promotion: None, to: hex!("f2") });
        assert_eq!(result[19], San { from: hex!("f6"), promotion: None, to: hex!("f1") });
        assert_eq!(result[20], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[21], San { from: hex!("f6"), promotion: None, to: hex!("d4") });
        assert_eq!(result[22], San { from: hex!("f6"), promotion: None, to: hex!("c3") });
        assert_eq!(result[23], San { from: hex!("f6"), promotion: None, to: hex!("b2") });
        assert_eq!(result[24], San { from: hex!("f6"), promotion: None, to: hex!("a1") });
        assert_eq!(result[25], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        assert_eq!(result[26], San { from: hex!("f6"), promotion: None, to: hex!("d6") });
        assert_eq!(result[27], San { from: hex!("f6"), promotion: None, to: hex!("c6") });
        assert_eq!(result[28], San { from: hex!("f6"), promotion: None, to: hex!("b6") });
        assert_eq!(result[29], San { from: hex!("f6"), promotion: None, to: hex!("a6") }); // <- a6 is hostile
    }
}
