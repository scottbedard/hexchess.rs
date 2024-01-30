use crate::game::board::{Position, get_step};
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::{Color, Piece};

pub fn target(hexchess: &Hexchess, position: Position, color: Color) -> Vec<Notation> {
    let mut targets: Vec<Notation> = vec![];

    // one step diagonally (intermediate)
    // then one step orthogonally in each direction (first, second)
    let paths: [(u8, u8, u8); 6] = [
        (1, 0, 2),
        (3, 2, 4),
        (5, 4, 6),
        (7, 6, 8),
        (9, 8, 10),
        (11, 10, 0),
    ];

    for (intermediate_direction, first_direction, second_direction) in paths {
        let intermediate_position = match get_step(position, intermediate_direction) {
            Some(p) => p,
            None => continue,
        };

        match get_step(intermediate_position, first_direction) {
            Some(first_position) => match hexchess.board.get(first_position) {
                Some(piece) => if piece.color() != color {
                    targets.push(Notation { from: position, promotion: None, to: first_position });
                },
                None => targets.push(Notation { from: position, promotion: None, to: first_position }),
            },
            None => (),
        };

        match get_step(intermediate_position, second_direction) {
            Some(second_position) => match hexchess.board.get(second_position) {
                Some(piece) => if piece.color() != color {
                    targets.push(Notation { from: position, promotion: None, to: second_position });
                },
                None => targets.push(Notation { from: position, promotion: None, to: second_position }),
            },
            None => (),
        };
    }

    targets
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knight_targets() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhiteKnight));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 12);
        assert_eq!(targets[0].to_string(), "f6g8");
        assert_eq!(targets[1].to_string(), "f6h7");
        assert_eq!(targets[2].to_string(), "f6i5");
        assert_eq!(targets[3].to_string(), "f6i4");
        assert_eq!(targets[4].to_string(), "f6h3");
        assert_eq!(targets[5].to_string(), "f6g3");
        assert_eq!(targets[6].to_string(), "f6e3");
        assert_eq!(targets[7].to_string(), "f6d3");
        assert_eq!(targets[8].to_string(), "f6c4");
        assert_eq!(targets[9].to_string(), "f6c5");
        assert_eq!(targets[10].to_string(), "f6d7");
        assert_eq!(targets[11].to_string(), "f6e8");
    }

    #[test]
    fn test_knight_capture_enemy_pieces() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhiteKnight));
        hexchess.board.set(Position::I5, Some(Piece::BlackPawn));
        hexchess.board.set(Position::C5, Some(Piece::WhitePawn));
    
        let targets = hexchess.targets(Position::F6);
    
        assert_eq!(targets.len(), 11);
        assert_eq!(targets[0].to_string(), "f6g8");
        assert_eq!(targets[1].to_string(), "f6h7");
        assert_eq!(targets[2].to_string(), "f6i5"); // <- enemy piece at i5
        assert_eq!(targets[3].to_string(), "f6i4");
        assert_eq!(targets[4].to_string(), "f6h3");
        assert_eq!(targets[5].to_string(), "f6g3");
        assert_eq!(targets[6].to_string(), "f6e3");
        assert_eq!(targets[7].to_string(), "f6d3");
        assert_eq!(targets[8].to_string(), "f6c4");
        // friendly piece at c5
        assert_eq!(targets[9].to_string(), "f6d7");
        assert_eq!(targets[10].to_string(), "f6e8");
    }
}
