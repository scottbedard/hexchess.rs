use crate::game::board::{Position, get_step};
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::Color;

pub fn target(hexchess: &Hexchess, position: Position, color: Color) -> Vec<Notation> {
    let mut targets: Vec<Notation> = vec![];

    for n in 0..12 {
        let to = match get_step(position, n) {
            None => continue,
            Some(p) => p,
        };

        let is_target = match hexchess.board.get(to) {
            None => true,
            Some(piece) => piece.color() != color,
        };

        if is_target {
            targets.push(Notation {
                from: position,
                promotion: None,
                to,
            });
        }
    }

    targets
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::piece::Piece;
    #[test]
    fn test_king_targets() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F6, Some(Piece::WhiteKing));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 12);

        assert_eq!(targets[0].to_string(), "f6f7");
        assert_eq!(targets[1].to_string(), "f6g7");
        assert_eq!(targets[2].to_string(), "f6g6");
        assert_eq!(targets[3].to_string(), "f6h5");
        assert_eq!(targets[4].to_string(), "f6g5");
        assert_eq!(targets[5].to_string(), "f6g4");
        assert_eq!(targets[6].to_string(), "f6f5");
        assert_eq!(targets[7].to_string(), "f6e4");
        assert_eq!(targets[8].to_string(), "f6e5");
        assert_eq!(targets[9].to_string(), "f6d5");
        assert_eq!(targets[10].to_string(), "f6e6");
        assert_eq!(targets[11].to_string(), "f6e7");
    }

    #[test]
    fn test_king_only_captures_enemy_piece() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F6, Some(Piece::WhiteKing));
        hexchess.board.set(Position::E7, Some(Piece::WhitePawn));
        hexchess.board.set(Position::E6, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 11);
        assert_eq!(targets[0].to_string(), "f6f7");
        assert_eq!(targets[1].to_string(), "f6g7");
        assert_eq!(targets[2].to_string(), "f6g6");
        assert_eq!(targets[3].to_string(), "f6h5");
        assert_eq!(targets[4].to_string(), "f6g5");
        assert_eq!(targets[5].to_string(), "f6g4");
        assert_eq!(targets[6].to_string(), "f6f5");
        assert_eq!(targets[7].to_string(), "f6e4");
        assert_eq!(targets[8].to_string(), "f6e5");
        assert_eq!(targets[9].to_string(), "f6d5");
        assert_eq!(targets[10].to_string(), "f6e6");
    }
}
