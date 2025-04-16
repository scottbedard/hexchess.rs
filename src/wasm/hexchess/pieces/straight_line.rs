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
    use crate::h;
    use super::*;

    #[test]
    fn white_queen() {
        let result = Hexchess::parse("p/3/5/7/9/P4Q5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("f8") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("f9") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("f10") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("f11") }); // <- f11 is hostile
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("g7") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("h8") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("g6") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("h6") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("i6") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("k6") });
        assert_eq!(result[11], San { from: h!("f6"), promotion: None, to: h!("l6") });
        assert_eq!(result[12], San { from: h!("f6"), promotion: None, to: h!("h5") });
        assert_eq!(result[13], San { from: h!("f6"), promotion: None, to: h!("k4") });
        assert_eq!(result[14], San { from: h!("f6"), promotion: None, to: h!("g5") });
        assert_eq!(result[15], San { from: h!("f6"), promotion: None, to: h!("h4") });
        assert_eq!(result[16], San { from: h!("f6"), promotion: None, to: h!("i3") });
        assert_eq!(result[17], San { from: h!("f6"), promotion: None, to: h!("k2") });
        assert_eq!(result[18], San { from: h!("f6"), promotion: None, to: h!("l1") });
        assert_eq!(result[19], San { from: h!("f6"), promotion: None, to: h!("g4") });
        assert_eq!(result[20], San { from: h!("f6"), promotion: None, to: h!("h2") });
        assert_eq!(result[21], San { from: h!("f6"), promotion: None, to: h!("f5") });
        assert_eq!(result[22], San { from: h!("f6"), promotion: None, to: h!("f4") });
        assert_eq!(result[23], San { from: h!("f6"), promotion: None, to: h!("f3") });
        assert_eq!(result[24], San { from: h!("f6"), promotion: None, to: h!("f2") });
        assert_eq!(result[25], San { from: h!("f6"), promotion: None, to: h!("f1") });
        assert_eq!(result[26], San { from: h!("f6"), promotion: None, to: h!("e4") });
        assert_eq!(result[27], San { from: h!("f6"), promotion: None, to: h!("d2") });
        assert_eq!(result[28], San { from: h!("f6"), promotion: None, to: h!("e5") });
        assert_eq!(result[29], San { from: h!("f6"), promotion: None, to: h!("d4") });
        assert_eq!(result[30], San { from: h!("f6"), promotion: None, to: h!("c3") });
        assert_eq!(result[31], San { from: h!("f6"), promotion: None, to: h!("b2") });
        assert_eq!(result[32], San { from: h!("f6"), promotion: None, to: h!("a1") });
        assert_eq!(result[33], San { from: h!("f6"), promotion: None, to: h!("d5") });
        assert_eq!(result[34], San { from: h!("f6"), promotion: None, to: h!("b4") });
        assert_eq!(result[35], San { from: h!("f6"), promotion: None, to: h!("e6") });
        assert_eq!(result[36], San { from: h!("f6"), promotion: None, to: h!("d6") });
        assert_eq!(result[37], San { from: h!("f6"), promotion: None, to: h!("c6") });
        assert_eq!(result[38], San { from: h!("f6"), promotion: None, to: h!("b6") });
        // a6 is friendly
        assert_eq!(result[39], San { from: h!("f6"), promotion: None, to: h!("e7") });
        assert_eq!(result[40], San { from: h!("f6"), promotion: None, to: h!("d8") });
    }

    #[test]
    fn black_queen() {
        let result = Hexchess::parse("p/3/5/7/9/P4q5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("f8") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("f9") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("f10") });
        // f11 is friendly
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("g7") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("h8") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("g6") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("h6") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("i6") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("k6") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("l6") });
        assert_eq!(result[11], San { from: h!("f6"), promotion: None, to: h!("h5") });
        assert_eq!(result[12], San { from: h!("f6"), promotion: None, to: h!("k4") });
        assert_eq!(result[13], San { from: h!("f6"), promotion: None, to: h!("g5") });
        assert_eq!(result[14], San { from: h!("f6"), promotion: None, to: h!("h4") });
        assert_eq!(result[15], San { from: h!("f6"), promotion: None, to: h!("i3") });
        assert_eq!(result[16], San { from: h!("f6"), promotion: None, to: h!("k2") });
        assert_eq!(result[17], San { from: h!("f6"), promotion: None, to: h!("l1") });
        assert_eq!(result[18], San { from: h!("f6"), promotion: None, to: h!("g4") });
        assert_eq!(result[19], San { from: h!("f6"), promotion: None, to: h!("h2") });
        assert_eq!(result[20], San { from: h!("f6"), promotion: None, to: h!("f5") });
        assert_eq!(result[21], San { from: h!("f6"), promotion: None, to: h!("f4") });
        assert_eq!(result[22], San { from: h!("f6"), promotion: None, to: h!("f3") });
        assert_eq!(result[23], San { from: h!("f6"), promotion: None, to: h!("f2") });
        assert_eq!(result[24], San { from: h!("f6"), promotion: None, to: h!("f1") });
        assert_eq!(result[25], San { from: h!("f6"), promotion: None, to: h!("e4") });
        assert_eq!(result[26], San { from: h!("f6"), promotion: None, to: h!("d2") });
        assert_eq!(result[27], San { from: h!("f6"), promotion: None, to: h!("e5") });
        assert_eq!(result[28], San { from: h!("f6"), promotion: None, to: h!("d4") });
        assert_eq!(result[29], San { from: h!("f6"), promotion: None, to: h!("c3") });
        assert_eq!(result[30], San { from: h!("f6"), promotion: None, to: h!("b2") });
        assert_eq!(result[31], San { from: h!("f6"), promotion: None, to: h!("a1") });
        assert_eq!(result[32], San { from: h!("f6"), promotion: None, to: h!("d5") });
        assert_eq!(result[33], San { from: h!("f6"), promotion: None, to: h!("b4") });
        assert_eq!(result[34], San { from: h!("f6"), promotion: None, to: h!("e6") });
        assert_eq!(result[35], San { from: h!("f6"), promotion: None, to: h!("d6") });
        assert_eq!(result[36], San { from: h!("f6"), promotion: None, to: h!("c6") });
        assert_eq!(result[37], San { from: h!("f6"), promotion: None, to: h!("b6") });
        assert_eq!(result[38], San { from: h!("f6"), promotion: None, to: h!("a6") }); // <- a6 is hostile
        assert_eq!(result[39], San { from: h!("f6"), promotion: None, to: h!("e7") });
        assert_eq!(result[40], San { from: h!("f6"), promotion: None, to: h!("d8") });
    }

    #[test]
    fn white_bishop() {
        let result = Hexchess::parse("1/3/5/1p3P1/9/5B5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("g7") });
        // h8 is friendly
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("h5") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("k4") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("g4") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("h2") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("e4") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("d2") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("d5") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("b4") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("e7") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("d8") }); // <- d8 is hostile
    }

    #[test]
    fn black_bishop() {
        let result = Hexchess::parse("1/3/5/1p3P1/9/5b5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("g7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("h8") }); // <- h8 is hostile
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("h5") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("k4") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("g4") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("h2") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("e4") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("d2") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("d5") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("b4") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("e7") });
        // d8 is friendly
    }

    #[test]
    fn white_rook() {
        let result = Hexchess::parse("p/3/5/7/9/P4R5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("f8") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("f9") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("f10") });
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("f11") }); // <- f11 is hostile
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("g6") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("h6") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("i6") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("k6") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("l6") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("g5") });
        assert_eq!(result[11], San { from: h!("f6"), promotion: None, to: h!("h4") });
        assert_eq!(result[12], San { from: h!("f6"), promotion: None, to: h!("i3") });
        assert_eq!(result[13], San { from: h!("f6"), promotion: None, to: h!("k2") });
        assert_eq!(result[14], San { from: h!("f6"), promotion: None, to: h!("l1") });
        assert_eq!(result[15], San { from: h!("f6"), promotion: None, to: h!("f5") });
        assert_eq!(result[16], San { from: h!("f6"), promotion: None, to: h!("f4") });
        assert_eq!(result[17], San { from: h!("f6"), promotion: None, to: h!("f3") });
        assert_eq!(result[18], San { from: h!("f6"), promotion: None, to: h!("f2") });
        assert_eq!(result[19], San { from: h!("f6"), promotion: None, to: h!("f1") });
        assert_eq!(result[20], San { from: h!("f6"), promotion: None, to: h!("e5") });
        assert_eq!(result[21], San { from: h!("f6"), promotion: None, to: h!("d4") });
        assert_eq!(result[22], San { from: h!("f6"), promotion: None, to: h!("c3") });
        assert_eq!(result[23], San { from: h!("f6"), promotion: None, to: h!("b2") });
        assert_eq!(result[24], San { from: h!("f6"), promotion: None, to: h!("a1") });
        assert_eq!(result[25], San { from: h!("f6"), promotion: None, to: h!("e6") });
        assert_eq!(result[26], San { from: h!("f6"), promotion: None, to: h!("d6") });
        assert_eq!(result[27], San { from: h!("f6"), promotion: None, to: h!("c6") });
        assert_eq!(result[28], San { from: h!("f6"), promotion: None, to: h!("b6") });
        // a6 is friendly
    }

    #[test]
    fn black_rook() {
        let result = Hexchess::parse("p/3/5/7/9/P4r5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(h!("f6"));

        assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
        assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("f8") });
        assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("f9") });
        assert_eq!(result[3], San { from: h!("f6"), promotion: None, to: h!("f10") });
        // f11 is friendly
        assert_eq!(result[4], San { from: h!("f6"), promotion: None, to: h!("g6") });
        assert_eq!(result[5], San { from: h!("f6"), promotion: None, to: h!("h6") });
        assert_eq!(result[6], San { from: h!("f6"), promotion: None, to: h!("i6") });
        assert_eq!(result[7], San { from: h!("f6"), promotion: None, to: h!("k6") });
        assert_eq!(result[8], San { from: h!("f6"), promotion: None, to: h!("l6") });
        assert_eq!(result[9], San { from: h!("f6"), promotion: None, to: h!("g5") });
        assert_eq!(result[10], San { from: h!("f6"), promotion: None, to: h!("h4") });
        assert_eq!(result[11], San { from: h!("f6"), promotion: None, to: h!("i3") });
        assert_eq!(result[12], San { from: h!("f6"), promotion: None, to: h!("k2") });
        assert_eq!(result[13], San { from: h!("f6"), promotion: None, to: h!("l1") });
        assert_eq!(result[14], San { from: h!("f6"), promotion: None, to: h!("f5") });
        assert_eq!(result[15], San { from: h!("f6"), promotion: None, to: h!("f4") });
        assert_eq!(result[16], San { from: h!("f6"), promotion: None, to: h!("f3") });
        assert_eq!(result[17], San { from: h!("f6"), promotion: None, to: h!("f2") });
        assert_eq!(result[18], San { from: h!("f6"), promotion: None, to: h!("f1") });
        assert_eq!(result[19], San { from: h!("f6"), promotion: None, to: h!("e5") });
        assert_eq!(result[20], San { from: h!("f6"), promotion: None, to: h!("d4") });
        assert_eq!(result[21], San { from: h!("f6"), promotion: None, to: h!("c3") });
        assert_eq!(result[22], San { from: h!("f6"), promotion: None, to: h!("b2") });
        assert_eq!(result[23], San { from: h!("f6"), promotion: None, to: h!("a1") });
        assert_eq!(result[24], San { from: h!("f6"), promotion: None, to: h!("e6") });
        assert_eq!(result[25], San { from: h!("f6"), promotion: None, to: h!("d6") });
        assert_eq!(result[26], San { from: h!("f6"), promotion: None, to: h!("c6") });
        assert_eq!(result[27], San { from: h!("f6"), promotion: None, to: h!("b6") });
        assert_eq!(result[28], San { from: h!("f6"), promotion: None, to: h!("a6") }); // <- a6 is hostile
    }
}
