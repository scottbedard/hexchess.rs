use crate::game::board::{Position, get_step};
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::{Color, Piece, PromotionPiece};

pub fn target(hexchess: &Hexchess, position: Position, color: Color) -> Vec<Notation> {
    let mut raw_targets: Vec<Notation> = vec![];

    let (
        advance_direction,
        portside_direction,
        starboard_direction
    ): (u8, u8, u8) = match color {
        Color::Black => (6, 4, 8),
        Color::White => (0, 10, 2),
    };

    // advance forward one position
    match advance(hexchess, position, position, advance_direction) {
        None => (),
        Some(notation) => {
            raw_targets.push(notation);

            // advance forward another position if possible
            if is_starting_position(position, color) {
                match advance(hexchess, position, notation.to, advance_direction) {
                    Some(notation) => raw_targets.push(notation),
                    None => (),
                }
            }
        },
    };

    // capture portside and starboard
    match capture(hexchess, position, color, portside_direction) {
        None => (),
        Some(notation) => raw_targets.push(notation),
    };

    match capture(hexchess, position, color, starboard_direction) {
        None => (),
        Some(notation) => raw_targets.push(notation),
    };

    // capture en_passant
    match capture_en_passant(hexchess, position, color, portside_direction, starboard_direction) {
        None => (),
        Some(notation) => raw_targets.push(notation),
    };

    // expand promotion moves
    let mut targets: Vec<Notation> = vec![];

    for notation in raw_targets {
        if is_promotion_position(color, notation.to) {
            targets.push(Notation {
                from: notation.from,
                to: notation.to,
                promotion: Some(PromotionPiece::Queen),
            });
            targets.push(Notation {
                from: notation.from,
                to: notation.to,
                promotion: Some(PromotionPiece::Knight),
            });
            targets.push(Notation {
                from: notation.from,
                to: notation.to,
                promotion: Some(PromotionPiece::Rook),
            });
            targets.push(Notation {
                from: notation.from,
                to: notation.to,
                promotion: Some(PromotionPiece::Bishop),
            });
        } else {
            targets.push(notation);
        }
    }

    targets
}

fn advance(hexchess: &Hexchess, original_position: Position, from_position: Position, advance_direction: u8) -> Option<Notation> {
    // safe to unwrap because pawns are never on the farthest rank, they would have been promoted
    let forward_position = get_step(from_position, advance_direction).unwrap();
    
    match hexchess.board.get(forward_position) {
        Some(_) => None,
        None => return Some(Notation {
            from: original_position,
            to: forward_position,
            promotion: None,
        }),
    }
}

fn capture(hexchess: &Hexchess, position: Position, color: Color, direction: u8) -> Option<Notation> {
    let capture_position = match get_step(position, direction) {
        Some(position) => position,
        None => return None,
    };

    match hexchess.board.get(capture_position) {
        Some(piece) => match piece.color() == color {
            true => None,
            false => Some(Notation {
                from: position,
                to: capture_position,
                promotion: None,
            }),
        },
        None => None,
    }
}

fn capture_en_passant(hexchess: &Hexchess, position: Position, color: Color, portside_direction: u8, starboard_direction: u8) -> Option<Notation> {
    let en_passant_position = match hexchess.en_passant {
        Some(en_passant) => en_passant,
        None => return None,
    };

    let (enemy_direction, enemy_pawn) = match color {
        Color::Black => (0, Piece::WhitePawn),
        Color::White => (6, Piece::BlackPawn),
    };

    let en_passant_sibling = match get_step(en_passant_position, enemy_direction) {
        Some(position) => position,
        None => return None,
    };

    let en_passant_piece = match hexchess.board.get(en_passant_sibling) {
        Some(position) => position,
        None => return None,
    };

    if en_passant_piece != enemy_pawn {
        return None
    }

    match get_step(position, starboard_direction) {
        Some(starboard_position) => {
            if starboard_position == en_passant_position {
                return Some(Notation {
                    from: position,
                    to: en_passant_position,
                    promotion: None,
                });
            }
        },
        None => (),
    };

    match get_step(position, portside_direction) {
        Some(portside_position) => {
            if portside_position == en_passant_position {
                return Some(Notation {
                    from: position,
                    to: en_passant_position,
                    promotion: None,
                });
            }
        },
        None => (),
    };

    None
}

pub fn is_promotion_position(color: Color, position: Position) -> bool {
    match color {
        Color::White => match position {
            Position::A6 |
            Position::B7 |
            Position::C8 |
            Position::D9 |
            Position::E10 |
            Position::F11 |
            Position::G10 |
            Position::H9 |
            Position::I8 |
            Position::K7 |
            Position::L6 => true,
            _ => false,
        },
        Color::Black => match position {
            Position::A1 |
            Position::B1 |
            Position::C1 |
            Position::D1 |
            Position::E1 |
            Position::F1 |
            Position::G1 |
            Position::H1 |
            Position::I1 |
            Position::K1 |
            Position::L1 => true,
            _ => false,
        },
    }
}

fn is_starting_position(position: Position, color: Color) -> bool {
    match color {
        Color::Black => match position {
            Position::B7 |
            Position::C7 |
            Position::D7 |
            Position::E7 |
            Position::F7 |
            Position::G7 |
            Position::H7 |
            Position::I7 |
            Position::K7 => true,
            _ => false,
        },
        Color::White => match position {
            Position::B1 |
            Position::C2 |
            Position::D3 |
            Position::E4 |
            Position::F5 |
            Position::G4 |
            Position::H3 |
            Position::I2 |
            Position::K1 => true,
            _ => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::piece::Piece;

    #[test]
    fn test_black_pawn_advance() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F7, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F7);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[0].to_string(), "f7f6");
        assert_eq!(targets[1].to_string(), "f7f5");
    }

    #[test]
    fn test_white_pawn_advance() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F5, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F5);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[0].to_string(), "f5f6");
        assert_eq!(targets[1].to_string(), "f5f7");
    }

    #[test]
    fn test_advance_one_blocked() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
        hexchess.board.set(Position::F7, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F5);

        assert_eq!(targets.len(), 0);
    }

    #[test]
    fn test_advance_two_blocked() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F5, Some(Piece::WhitePawn));
        hexchess.board.set(Position::F7, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F5);

        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].to_string(), "f5f6");
    }

    #[test]
    fn test_pawn_advance_with_no_portside_capture_position() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::A1, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::A1);

        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].to_string(), "a1a2");
    }

    #[test]
    fn test_pawn_advance_with_no_starboard_capture_position() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::L1, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::L1);

        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].to_string(), "l1l2");
    }

    #[test]
    fn test_no_double_move_off_non_starting_position() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 1);
    }

    #[test]
    fn test_pawn_does_not_capture_friendly_piece() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
        hexchess.board.set(Position::G6, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].to_string(), "f6f7");
    }

    #[test]
    fn test_black_pawn_capture_portside() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::BlackPawn));
        hexchess.board.set(Position::G5, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[1].to_string(), "f6g5");
    }

    #[test]
    fn test_black_pawn_capture_starboard() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::BlackPawn));
        hexchess.board.set(Position::E5, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[1].to_string(), "f6e5");
    }

    #[test]
    fn test_white_pawn_capture_portside() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
        hexchess.board.set(Position::E6, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[1].to_string(), "f6e6");
    }

    #[test]
    fn test_white_pawn_capture_starboard() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
        hexchess.board.set(Position::G6, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[1].to_string(), "f6g6");
    }

    #[test]
    fn test_black_pawn_capture_en_passant_portside() {
          let mut hexchess = Hexchess::new();

          hexchess.board.set(Position::F6, Some(Piece::BlackPawn));
          hexchess.board.set(Position::G6, Some(Piece::WhitePawn));
          hexchess.turn = Color::Black;
          hexchess.en_passant = Some(Position::G5);

          let targets = hexchess.targets(Position::F6);

          assert_eq!(targets.len(), 2);
          assert_eq!(targets[1].to_string(), "f6g5");
    }

    #[test]
    fn test_black_pawn_capture_en_passant_starboard() {
          let mut hexchess = Hexchess::new();

          hexchess.board.set(Position::F6, Some(Piece::BlackPawn));
          hexchess.board.set(Position::E6, Some(Piece::WhitePawn));
          hexchess.turn = Color::Black;
          hexchess.en_passant = Some(Position::E5);

          let targets = hexchess.targets(Position::F6);

          assert_eq!(targets.len(), 2);
          assert_eq!(targets[1].to_string(), "f6e5");
    }

    #[test]
    fn test_white_pawn_capture_en_passant_portside() {
          let mut hexchess = Hexchess::new();

          hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
          hexchess.board.set(Position::E6, Some(Piece::BlackPawn));
          hexchess.turn = Color::White;
          hexchess.en_passant = Some(Position::E6);

          let targets = hexchess.targets(Position::F6);

          assert_eq!(targets.len(), 2);
          assert_eq!(targets[1].to_string(), "f6e6");
    }

    #[test]
    fn test_white_pawn_capture_en_passant_starboard() {
          let mut hexchess = Hexchess::new();

          hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
          hexchess.board.set(Position::G6, Some(Piece::BlackPawn));
          hexchess.turn = Color::White;
          hexchess.en_passant = Some(Position::E6);

          let targets = hexchess.targets(Position::F6);

          assert_eq!(targets.len(), 2);
          assert_eq!(targets[1].to_string(), "f6g6");
    }

    #[test]
    fn test_pawn_en_passant_does_not_capture_friendly_pieces() {
          let mut hexchess = Hexchess::new();

          hexchess.board.set(Position::F5, Some(Piece::WhitePawn));
          hexchess.board.set(Position::G6, Some(Piece::WhitePawn));
          hexchess.turn = Color::White;
          hexchess.en_passant = Some(Position::G5);

          let targets = hexchess.targets(Position::F5);

          assert_eq!(targets.len(), 2);
          assert_eq!(targets[0].to_string(), "f5f6");
          assert_eq!(targets[1].to_string(), "f5f7");
    }

    #[test]
    fn test_is_promotion_position() {
        assert_eq!(true, is_promotion_position(Color::White, Position::B7));
        assert_eq!(false, is_promotion_position(Color::Black, Position::B7));

        assert_eq!(true, is_promotion_position(Color::Black, Position::A1));
        assert_eq!(false, is_promotion_position(Color::White, Position::A1));
    }

    #[test]
    fn test_pawn_promotion() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F10, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F10);

        assert_eq!(targets.len(), 4);
        assert_eq!(targets[0].to_string(), "f10f11q");
        assert_eq!(targets[1].to_string(), "f10f11n");
        assert_eq!(targets[2].to_string(), "f10f11r");
        assert_eq!(targets[3].to_string(), "f10f11b");
    }
}
