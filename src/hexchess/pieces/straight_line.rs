use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::utils::walk;

use crate::constants::{
    Color,
    San,
};

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

        assert!(result.iter().eq([ 
            San { from: hex!("f6"), promotion: None, to: hex!("f7") },
            San { from: hex!("f6"), promotion: None, to: hex!("f8") },
            San { from: hex!("f6"), promotion: None, to: hex!("f9") },
            San { from: hex!("f6"), promotion: None, to: hex!("f10") },
            San { from: hex!("f6"), promotion: None, to: hex!("f11") }, // <- f11 is hostile
            San { from: hex!("f6"), promotion: None, to: hex!("g7") },
            San { from: hex!("f6"), promotion: None, to: hex!("h8") },
            San { from: hex!("f6"), promotion: None, to: hex!("g6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h6") },
            San { from: hex!("f6"), promotion: None, to: hex!("i6") },
            San { from: hex!("f6"), promotion: None, to: hex!("k6") },
            San { from: hex!("f6"), promotion: None, to: hex!("l6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h5") },
            San { from: hex!("f6"), promotion: None, to: hex!("k4") },
            San { from: hex!("f6"), promotion: None, to: hex!("g5") },
            San { from: hex!("f6"), promotion: None, to: hex!("h4") },
            San { from: hex!("f6"), promotion: None, to: hex!("i3") },
            San { from: hex!("f6"), promotion: None, to: hex!("k2") },
            San { from: hex!("f6"), promotion: None, to: hex!("l1") },
            San { from: hex!("f6"), promotion: None, to: hex!("g4") },
            San { from: hex!("f6"), promotion: None, to: hex!("h2") },
            San { from: hex!("f6"), promotion: None, to: hex!("f5") },
            San { from: hex!("f6"), promotion: None, to: hex!("f4") },
            San { from: hex!("f6"), promotion: None, to: hex!("f3") },
            San { from: hex!("f6"), promotion: None, to: hex!("f2") },
            San { from: hex!("f6"), promotion: None, to: hex!("f1") },
            San { from: hex!("f6"), promotion: None, to: hex!("e4") },
            San { from: hex!("f6"), promotion: None, to: hex!("d2") },
            San { from: hex!("f6"), promotion: None, to: hex!("e5") },
            San { from: hex!("f6"), promotion: None, to: hex!("d4") },
            San { from: hex!("f6"), promotion: None, to: hex!("c3") },
            San { from: hex!("f6"), promotion: None, to: hex!("b2") },
            San { from: hex!("f6"), promotion: None, to: hex!("a1") },
            San { from: hex!("f6"), promotion: None, to: hex!("d5") },
            San { from: hex!("f6"), promotion: None, to: hex!("b4") },
            San { from: hex!("f6"), promotion: None, to: hex!("e6") },
            San { from: hex!("f6"), promotion: None, to: hex!("d6") },
            San { from: hex!("f6"), promotion: None, to: hex!("c6") },
            San { from: hex!("f6"), promotion: None, to: hex!("b6") },
            // a6 is friendly
            San { from: hex!("f6"), promotion: None, to: hex!("e7") },
            San { from: hex!("f6"), promotion: None, to: hex!("d8") },
        ].iter()));
    }

    #[test]
    fn black_queen() {
        let result = Hexchess::from("p/3/5/7/9/P4q5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([ 
            San { from: hex!("f6"), promotion: None, to: hex!("f7") },
            San { from: hex!("f6"), promotion: None, to: hex!("f8") },
            San { from: hex!("f6"), promotion: None, to: hex!("f9") },
            San { from: hex!("f6"), promotion: None, to: hex!("f10") },
            // f11 is friendly
            San { from: hex!("f6"), promotion: None, to: hex!("g7") },
            San { from: hex!("f6"), promotion: None, to: hex!("h8") },
            San { from: hex!("f6"), promotion: None, to: hex!("g6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h6") },
            San { from: hex!("f6"), promotion: None, to: hex!("i6") },
            San { from: hex!("f6"), promotion: None, to: hex!("k6") },
            San { from: hex!("f6"), promotion: None, to: hex!("l6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h5") },
            San { from: hex!("f6"), promotion: None, to: hex!("k4") },
            San { from: hex!("f6"), promotion: None, to: hex!("g5") },
            San { from: hex!("f6"), promotion: None, to: hex!("h4") },
            San { from: hex!("f6"), promotion: None, to: hex!("i3") },
            San { from: hex!("f6"), promotion: None, to: hex!("k2") },
            San { from: hex!("f6"), promotion: None, to: hex!("l1") },
            San { from: hex!("f6"), promotion: None, to: hex!("g4") },
            San { from: hex!("f6"), promotion: None, to: hex!("h2") },
            San { from: hex!("f6"), promotion: None, to: hex!("f5") },
            San { from: hex!("f6"), promotion: None, to: hex!("f4") },
            San { from: hex!("f6"), promotion: None, to: hex!("f3") },
            San { from: hex!("f6"), promotion: None, to: hex!("f2") },
            San { from: hex!("f6"), promotion: None, to: hex!("f1") },
            San { from: hex!("f6"), promotion: None, to: hex!("e4") },
            San { from: hex!("f6"), promotion: None, to: hex!("d2") },
            San { from: hex!("f6"), promotion: None, to: hex!("e5") },
            San { from: hex!("f6"), promotion: None, to: hex!("d4") },
            San { from: hex!("f6"), promotion: None, to: hex!("c3") },
            San { from: hex!("f6"), promotion: None, to: hex!("b2") },
            San { from: hex!("f6"), promotion: None, to: hex!("a1") },
            San { from: hex!("f6"), promotion: None, to: hex!("d5") },
            San { from: hex!("f6"), promotion: None, to: hex!("b4") },
            San { from: hex!("f6"), promotion: None, to: hex!("e6") },
            San { from: hex!("f6"), promotion: None, to: hex!("d6") },
            San { from: hex!("f6"), promotion: None, to: hex!("c6") },
            San { from: hex!("f6"), promotion: None, to: hex!("b6") },
            San { from: hex!("f6"), promotion: None, to: hex!("a6") }, // <- a6 is hostile
            San { from: hex!("f6"), promotion: None, to: hex!("e7") },
            San { from: hex!("f6"), promotion: None, to: hex!("d8") },
        ].iter()));
    }

    #[test]
    fn white_bishop() {
        let result = Hexchess::from("1/3/5/1p3P1/9/5B5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([ 
            San { from: hex!("f6"), promotion: None, to: hex!("g7") },
            // h8 is friendly
            San { from: hex!("f6"), promotion: None, to: hex!("h5") },
            San { from: hex!("f6"), promotion: None, to: hex!("k4") },
            San { from: hex!("f6"), promotion: None, to: hex!("g4") },
            San { from: hex!("f6"), promotion: None, to: hex!("h2") },
            San { from: hex!("f6"), promotion: None, to: hex!("e4") },
            San { from: hex!("f6"), promotion: None, to: hex!("d2") },
            San { from: hex!("f6"), promotion: None, to: hex!("d5") },
            San { from: hex!("f6"), promotion: None, to: hex!("b4") },
            San { from: hex!("f6"), promotion: None, to: hex!("e7") },
            San { from: hex!("f6"), promotion: None, to: hex!("d8") }, // <- d8 is hostile
        ].iter()));
    }

    #[test]
    fn black_bishop() {
        let result = Hexchess::from("1/3/5/1p3P1/9/5b5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([
            San { from: hex!("f6"), promotion: None, to: hex!("g7") },
            San { from: hex!("f6"), promotion: None, to: hex!("h8") }, // <- h8 is hostile
            San { from: hex!("f6"), promotion: None, to: hex!("h5") },
            San { from: hex!("f6"), promotion: None, to: hex!("k4") },
            San { from: hex!("f6"), promotion: None, to: hex!("g4") },
            San { from: hex!("f6"), promotion: None, to: hex!("h2") },
            San { from: hex!("f6"), promotion: None, to: hex!("e4") },
            San { from: hex!("f6"), promotion: None, to: hex!("d2") },
            San { from: hex!("f6"), promotion: None, to: hex!("d5") },
            San { from: hex!("f6"), promotion: None, to: hex!("b4") },
            San { from: hex!("f6"), promotion: None, to: hex!("e7") },
            // d8 is friendly
        ].iter()));
    }

    #[test]
    fn white_rook() {
        let result = Hexchess::from("p/3/5/7/9/P4R5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([ 
            San { from: hex!("f6"), promotion: None, to: hex!("f7") },
            San { from: hex!("f6"), promotion: None, to: hex!("f8") },
            San { from: hex!("f6"), promotion: None, to: hex!("f9") },
            San { from: hex!("f6"), promotion: None, to: hex!("f10") },
            San { from: hex!("f6"), promotion: None, to: hex!("f11") }, // <- f11 is hostile
            San { from: hex!("f6"), promotion: None, to: hex!("g6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h6") },
            San { from: hex!("f6"), promotion: None, to: hex!("i6") },
            San { from: hex!("f6"), promotion: None, to: hex!("k6") },
            San { from: hex!("f6"), promotion: None, to: hex!("l6") },
            San { from: hex!("f6"), promotion: None, to: hex!("g5") },
            San { from: hex!("f6"), promotion: None, to: hex!("h4") },
            San { from: hex!("f6"), promotion: None, to: hex!("i3") },
            San { from: hex!("f6"), promotion: None, to: hex!("k2") },
            San { from: hex!("f6"), promotion: None, to: hex!("l1") },
            San { from: hex!("f6"), promotion: None, to: hex!("f5") },
            San { from: hex!("f6"), promotion: None, to: hex!("f4") },
            San { from: hex!("f6"), promotion: None, to: hex!("f3") },
            San { from: hex!("f6"), promotion: None, to: hex!("f2") },
            San { from: hex!("f6"), promotion: None, to: hex!("f1") },
            San { from: hex!("f6"), promotion: None, to: hex!("e5") },
            San { from: hex!("f6"), promotion: None, to: hex!("d4") },
            San { from: hex!("f6"), promotion: None, to: hex!("c3") },
            San { from: hex!("f6"), promotion: None, to: hex!("b2") },
            San { from: hex!("f6"), promotion: None, to: hex!("a1") },
            San { from: hex!("f6"), promotion: None, to: hex!("e6") },
            San { from: hex!("f6"), promotion: None, to: hex!("d6") },
            San { from: hex!("f6"), promotion: None, to: hex!("c6") },
            San { from: hex!("f6"), promotion: None, to: hex!("b6") },
            // a6 is friendly        
        ].iter()));
    }

    #[test]
    fn black_rook() {
        let result = Hexchess::from("p/3/5/7/9/P4r5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .current_moves();

        assert!(result.iter().eq([ 
            San { from: hex!("f6"), promotion: None, to: hex!("f7") },
            San { from: hex!("f6"), promotion: None, to: hex!("f8") },
            San { from: hex!("f6"), promotion: None, to: hex!("f9") },
            San { from: hex!("f6"), promotion: None, to: hex!("f10") },
            // f11 is friendly
            San { from: hex!("f6"), promotion: None, to: hex!("g6") },
            San { from: hex!("f6"), promotion: None, to: hex!("h6") },
            San { from: hex!("f6"), promotion: None, to: hex!("i6") },
            San { from: hex!("f6"), promotion: None, to: hex!("k6") },
            San { from: hex!("f6"), promotion: None, to: hex!("l6") },
            San { from: hex!("f6"), promotion: None, to: hex!("g5") },
            San { from: hex!("f6"), promotion: None, to: hex!("h4") },
            San { from: hex!("f6"), promotion: None, to: hex!("i3") },
            San { from: hex!("f6"), promotion: None, to: hex!("k2") },
            San { from: hex!("f6"), promotion: None, to: hex!("l1") },
            San { from: hex!("f6"), promotion: None, to: hex!("f5") },
            San { from: hex!("f6"), promotion: None, to: hex!("f4") },
            San { from: hex!("f6"), promotion: None, to: hex!("f3") },
            San { from: hex!("f6"), promotion: None, to: hex!("f2") },
            San { from: hex!("f6"), promotion: None, to: hex!("f1") },
            San { from: hex!("f6"), promotion: None, to: hex!("e5") },
            San { from: hex!("f6"), promotion: None, to: hex!("d4") },
            San { from: hex!("f6"), promotion: None, to: hex!("c3") },
            San { from: hex!("f6"), promotion: None, to: hex!("b2") },
            San { from: hex!("f6"), promotion: None, to: hex!("a1") },
            San { from: hex!("f6"), promotion: None, to: hex!("e6") },
            San { from: hex!("f6"), promotion: None, to: hex!("d6") },
            San { from: hex!("f6"), promotion: None, to: hex!("c6") },
            San { from: hex!("f6"), promotion: None, to: hex!("b6") },
            San { from: hex!("f6"), promotion: None, to: hex!("a6") }, // <- a6 is hostile        
        ].iter()));
    }
}
