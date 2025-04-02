use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::utils::walk;

use crate::constants::{
    Color,
    Piece,
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
    use super::*;

    #[test]
    fn test_queen() {
        let result = Hexchess::from("1/3/5/7/9/5Q5/11/11/11/11/11 w - 0 1")
            .unwrap()
            .current_moves();

        // ...
    }
}
