use crate::game::board::{Board, Position};
use crate::game::failure::Failure;
use crate::game::notation::Notation;
use crate::game::piece::{Color, Piece};
use crate::game::targets::{bishop, king, knight, pawn, queen, rook};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::fmt;
use tsify::Tsify;

/// Hexchess game state
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Hexchess {
    pub board: Board,

    pub en_passant: Option<Position>,

    pub fullmove: u16,

    pub halfmove: u8,

    pub turn: Color,
}

/// Create hexchess from fen
impl Hexchess {
    pub fn apply(&mut self, notation: Notation) -> Result<(), Failure> {
        let piece = match self.board.get(notation.from) {
            Some(val) => val,
            None => return Err(Failure::IllegalMove),
        };

        // verify it is the correct piece's turn
        if piece.color() != self.turn {
            return Err(Failure::OutOfTurn);
        }

        // update halfmove
        if piece == Piece::WhitePawn || piece == Piece::BlackPawn || self.board.get(notation.to).is_some() {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }

        // update fullmove and turn color
        if piece.color() == Color::Black {
            self.fullmove += 1;
            self.turn = Color::White;
        } else {
            self.turn = Color::Black;
        }

        // set to and from positions
        self.board.set(notation.from, None);

        self.board.set(notation.to, match notation.promotion {
            None => Some(piece),
            Some(promotion) => match piece {
                Piece::BlackPawn => match pawn::is_promotion_position(Color::Black, notation.to) {
                    false => return Err(Failure::IllegalMove),
                    true => Some(promotion.to_piece(Color::Black)),
                },
                Piece::WhitePawn => match pawn::is_promotion_position(Color::White, notation.to) {
                    false => return Err(Failure::IllegalMove),
                    true => Some(promotion.to_piece(Color::White)),
                },
                _ => return Err(Failure::IllegalMove),
            },
        });

        // set en passant
        self.en_passant = match piece {
            Piece::BlackPawn => match (notation.from, notation.to) {
                (Position::B7, Position::B5) => Some(Position::B6),
                (Position::C7, Position::C5) => Some(Position::C6),
                (Position::D7, Position::D5) => Some(Position::D6),
                (Position::E7, Position::E5) => Some(Position::E6),
                (Position::F7, Position::F5) => Some(Position::F6),
                (Position::G7, Position::G5) => Some(Position::G6),
                (Position::H7, Position::H5) => Some(Position::H6),
                (Position::I7, Position::I5) => Some(Position::I6),
                (Position::K7, Position::K5) => Some(Position::K6),
                _ => None,
            },
            Piece::WhitePawn => match (notation.from, notation.to) {
                (Position::B1, Position::B3) => Some(Position::B2),
                (Position::C2, Position::C4) => Some(Position::C3),
                (Position::D3, Position::D5) => Some(Position::D4),
                (Position::E4, Position::E6) => Some(Position::E5),
                (Position::F5, Position::F7) => Some(Position::F6),
                (Position::G4, Position::G6) => Some(Position::G5),
                (Position::H3, Position::H5) => Some(Position::H4),
                (Position::I2, Position::I4) => Some(Position::I3),
                (Position::K1, Position::K3) => Some(Position::K2),
                _ => None,
            },
            _ => None,
        };

        Ok(())
    }

    pub fn from(value: &str) -> Result<Self, Failure> {
        let mut parts = value.split_whitespace();

        let board = match parts.next() {
            Some(part) => match Board::from(part) {
                Ok(result) => result,
                Err(failure) => return Err(failure)
            },
            None => Board::new(),
        };

        let turn = match parts.next() {
            Some(part) => match Color::from(part) {
                Ok(result) => result,
                Err(failure) => return Err(failure)
            },
            None => Color::White,
        };

        let en_passant = match parts.next() {
            Some(part) => match part {
                "-" => None,
                _ => match Position::from(part) {
                    Ok(result) => Some(result),
                    Err(failure) => return Err(failure)
                },
            },
            None => None,
        };

        let halfmove = match parts.next() {
            Some(part) => match part.parse::<u8>() {
                Ok(result) => result,
                Err(_) => return Err(Failure::InvalidHalfmove),
            },
            None => 0,
        };

        let fullmove = match parts.next() {
            Some(part) => match part.parse::<u16>() {
                Ok(result) => match result >= 1 {
                    true => result,
                    false => return Err(Failure::InvalidFullmove),
                },
                Err(_) => return Err(Failure::InvalidFullmove),
            },
            None => 1,
        };

        Ok(Hexchess {
            board,
            en_passant,
            fullmove,
            halfmove,
            turn,
        })
    }

    pub fn initial() -> Self {
        Hexchess {
            board: Board::initial(),
            en_passant: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    pub fn new() -> Self {
        Hexchess {
            board: Board::new(),
            en_passant: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    pub fn targets(&self, position: Position) -> Vec<Notation> {
        let piece = match self.board.get(position) {
            Some(val) => val,
            None => return vec![],
        };

        match piece {
            Piece::BlackBishop => bishop::target(&self, position, Color::Black),
            Piece::BlackKing => king::target(&self, position, Color::Black),
            Piece::BlackKnight => knight::target(&self, position, Color::Black),
            Piece::BlackPawn => pawn::target(&self, position, Color::Black),
            Piece::BlackQueen => queen::target(&self, position, Color::Black),
            Piece::BlackRook => rook::target(&self, position, Color::Black),
            Piece::WhiteBishop => bishop::target(&self, position, Color::White),
            Piece::WhiteKing => king::target(&self, position, Color::White),
            Piece::WhiteKnight => knight::target(&self, position, Color::White),
            Piece::WhitePawn => pawn::target(&self, position, Color::White),
            Piece::WhiteQueen => queen::target(&self, position, Color::White),
            Piece::WhiteRook => rook::target(&self, position, Color::White),
        }
    }

    pub fn to_json(&self) -> String {
        json!(self).to_string()
    }
}

impl fmt::Display for Hexchess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let en_passant = match self.en_passant {
            Some(value) => value.to_string(),
            None => "-".to_string(),
        };

        write!(f, "{} {} {} {} {}", self.board, self.turn, en_passant, self.halfmove, self.fullmove)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{EMPTY_HEXCHESS, INITIAL_HEXCHESS};

    #[test]
    fn test_create_hexchess_from_initial_board_fen() {
        let hexchess = match Hexchess::from(crate::constants::INITIAL_BOARD) {
            Ok(value) => value,
            Err(failure) => panic!("failure: {:?}", failure),
        };
        
        assert_eq!(Some(Piece::BlackQueen), hexchess.board.e10);
        assert_eq!(Color::White, hexchess.turn);
        assert_eq!(None, hexchess.en_passant);
        assert_eq!(0, hexchess.halfmove);
        assert_eq!(1, hexchess.fullmove);
    }

    #[test]
    fn test_create_hexchess_from_complete_fen() {
        let hexchess = match Hexchess::from("1/qbk/n1b1n/r4r1/ppppp2pp/6p4/3P1PP4/4PR1b3/5B1P3/2P2B2P2/1PRNQBK2P1 b a1 1 8") {
            Ok(value) => value,
            Err(failure) => panic!("failure: {:?}", failure),
        };
        
        assert_eq!(Some(Piece::BlackBishop), hexchess.board.h4);
        assert_eq!(Color::Black, hexchess.turn);
        assert_eq!(Some(Position::A1), hexchess.en_passant);
        assert_eq!(1, hexchess.halfmove);
        assert_eq!(8, hexchess.fullmove);
    }
    
    #[test]
    fn test_invalid_turn_color_results_in_error() {
        let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 ? - 0 1");

        assert_eq!(Err(Failure::InvalidColor), hexchess);
    }

    #[test]
    fn test_invalid_en_passant_results_in_error() {
        let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w ? 0 1");

        assert_eq!(Err(Failure::InvalidPosition), hexchess);
    }

    #[test]
    fn test_invalid_halfmove_results_in_error() {
        assert_eq!(Err(Failure::InvalidHalfmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - ? 1"));
        assert_eq!(Err(Failure::InvalidHalfmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0.5 1"));
        assert_eq!(Err(Failure::InvalidHalfmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - -6 1"));
    }

    #[test]
    fn test_invalid_fullmove_results_in_error() {
        assert_eq!(Err(Failure::InvalidFullmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 ?"));
        assert_eq!(Err(Failure::InvalidFullmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 1.5"));
        assert_eq!(Err(Failure::InvalidFullmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 0")); // <- less than 1
    }

    #[test]
    fn test_convert_hexchess_struct_to_json() {
        let hexchess = Hexchess::new();
        let expected = "{\"board\":{\"a1\":null,\"a2\":null,\"a3\":null,\"a4\":null,\"a5\":null,\"a6\":null,\"b1\":null,\"b2\":null,\"b3\":null,\"b4\":null,\"b5\":null,\"b6\":null,\"b7\":null,\"c1\":null,\"c2\":null,\"c3\":null,\"c4\":null,\"c5\":null,\"c6\":null,\"c7\":null,\"c8\":null,\"d1\":null,\"d2\":null,\"d3\":null,\"d4\":null,\"d5\":null,\"d6\":null,\"d7\":null,\"d8\":null,\"d9\":null,\"e1\":null,\"e10\":null,\"e2\":null,\"e3\":null,\"e4\":null,\"e5\":null,\"e6\":null,\"e7\":null,\"e8\":null,\"e9\":null,\"f1\":null,\"f10\":null,\"f11\":null,\"f2\":null,\"f3\":null,\"f4\":null,\"f5\":null,\"f6\":null,\"f7\":null,\"f8\":null,\"f9\":null,\"g1\":null,\"g10\":null,\"g2\":null,\"g3\":null,\"g4\":null,\"g5\":null,\"g6\":null,\"g7\":null,\"g8\":null,\"g9\":null,\"h1\":null,\"h2\":null,\"h3\":null,\"h4\":null,\"h5\":null,\"h6\":null,\"h7\":null,\"h8\":null,\"h9\":null,\"i1\":null,\"i2\":null,\"i3\":null,\"i4\":null,\"i5\":null,\"i6\":null,\"i7\":null,\"i8\":null,\"k1\":null,\"k2\":null,\"k3\":null,\"k4\":null,\"k5\":null,\"k6\":null,\"k7\":null,\"l1\":null,\"l2\":null,\"l3\":null,\"l4\":null,\"l5\":null,\"l6\":null},\"en_passant\":null,\"fullmove\":1,\"halfmove\":0,\"turn\":\"w\"}";

        assert_eq!(hexchess.to_json(), expected);
    }

    #[test]
    fn test_no_targets_returned_for_empty_position() {
        let hexchess = Hexchess::new();

        assert_eq!(hexchess.targets(Position::A1), vec![]);
    }

    #[test]
    fn test_apply_updates_game_state() {
        let mut hexchess = Hexchess::initial();
        assert_eq!(Color::White, hexchess.turn);
        assert_eq!(None, hexchess.en_passant);
        assert_eq!(0, hexchess.halfmove);
        assert_eq!(1, hexchess.fullmove);

        let _ = hexchess.apply(Notation::from("g4g5").unwrap());
        assert_eq!(Color::Black, hexchess.turn);
        assert_eq!(None, hexchess.en_passant);
        assert_eq!(0, hexchess.halfmove);
        assert_eq!(1, hexchess.fullmove);
        assert_eq!(Some(Piece::WhitePawn), hexchess.board.get(Position::G5));
        assert_eq!(None, hexchess.board.get(Position::G4));

        let _ = hexchess.apply(Notation::from("e7e6").unwrap());
        assert_eq!(Color::White, hexchess.turn);
        assert_eq!(None, hexchess.en_passant);
        assert_eq!(0, hexchess.halfmove);
        assert_eq!(2, hexchess.fullmove);
        assert_eq!(Some(Piece::BlackPawn), hexchess.board.get(Position::E6));
        assert_eq!(None, hexchess.board.get(Position::E5));

        let _ = hexchess.apply(Notation::from("f3i6").unwrap()); // <- white bishop move, no capture
        assert_eq!(1, hexchess.halfmove);
        assert_eq!(2, hexchess.fullmove);

        let _ = hexchess.apply(Notation::from("h7i6").unwrap()); // <- black pawn captures bishop
        assert_eq!(0, hexchess.halfmove);
        assert_eq!(3, hexchess.fullmove);
    }

    #[test]
    fn test_apply_from_empty_position() {
        let mut hexchess = Hexchess::new();

        let result = hexchess.apply(Notation::from("f5f6").unwrap());

        assert_eq!(Err(Failure::IllegalMove), result);
    }

    #[test]
    fn test_apply_sets_black_en_passant() {
        let mut b = Hexchess::from("1/3/5/7/p8/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = b.apply(Notation::from("b7b5").unwrap());
        assert_eq!(Some(Position::B6), b.en_passant);

        let mut c = Hexchess::from("1/3/5/7/1p7/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = c.apply(Notation::from("c7c5").unwrap());
        assert_eq!(Some(Position::C6), c.en_passant);
        
        let mut d = Hexchess::from("1/3/5/7/2p6/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = d.apply(Notation::from("d7d5").unwrap());
        assert_eq!(Some(Position::D6), d.en_passant);

        let mut e = Hexchess::from("1/3/5/7/3p5/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = e.apply(Notation::from("e7e5").unwrap());
        assert_eq!(Some(Position::E6), e.en_passant);

        let mut f = Hexchess::from("1/3/5/7/4p4/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = f.apply(Notation::from("f7f5").unwrap());
        assert_eq!(Some(Position::F6), f.en_passant);

        let mut g = Hexchess::from("1/3/5/7/5p3/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = g.apply(Notation::from("g7g5").unwrap());
        assert_eq!(Some(Position::G6), g.en_passant);

        let mut h = Hexchess::from("1/3/5/7/6p2/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = h.apply(Notation::from("h7h5").unwrap());
        assert_eq!(Some(Position::H6), h.en_passant);

        let mut i = Hexchess::from("1/3/5/7/7p1/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = i.apply(Notation::from("i7i5").unwrap());
        assert_eq!(Some(Position::I6), i.en_passant);

        let mut k = Hexchess::from("1/3/5/7/8p/11/11/11/11/11/11 b - 0 1").unwrap();
        let _ = k.apply(Notation::from("k7k5").unwrap());
        assert_eq!(Some(Position::K6), k.en_passant);
    }

    #[test]
    fn test_apply_sets_white_en_passant() {
        let mut b = Hexchess::from("1/3/5/7/9/11/11/11/11/11/1P9 w - 0 1").unwrap();
        let _ = b.apply(Notation::from("b1b3").unwrap());
        assert_eq!(Some(Position::B2), b.en_passant);

        let mut c = Hexchess::from("1/3/5/7/9/11/11/11/11/2P8/11 w - 0 1").unwrap();
        let _ = c.apply(Notation::from("c2c4").unwrap());
        assert_eq!(Some(Position::C3), c.en_passant);

        let mut d = Hexchess::from("1/3/5/7/9/11/11/11/3P7/11/11 w - 0 1").unwrap();
        let _ = d.apply(Notation::from("d3d5").unwrap());
        assert_eq!(Some(Position::D4), d.en_passant);

        let mut e = Hexchess::from("1/3/5/7/9/11/11/4P6/11/11/11 w - 0 1").unwrap();
        let _ = e.apply(Notation::from("e4e6").unwrap());
        assert_eq!(Some(Position::E5), e.en_passant);

        let mut f = Hexchess::from("1/3/5/7/9/11/5P5/11/11/11/11 w - 0 1").unwrap();
        let _ = f.apply(Notation::from("f5f7").unwrap());
        assert_eq!(Some(Position::F6), f.en_passant);

        let mut g = Hexchess::from("1/3/5/7/9/11/11/6P4/11/11/11 w - 0 1").unwrap();
        let _ = g.apply(Notation::from("g4g6").unwrap());
        assert_eq!(Some(Position::G5), g.en_passant);

        let mut h = Hexchess::from("1/3/5/7/9/11/11/11/7P3/11/11 w - 0 1").unwrap();
        let _ = h.apply(Notation::from("h3h5").unwrap());
        assert_eq!(Some(Position::H4), h.en_passant);

        let mut i = Hexchess::from("1/3/5/7/9/11/11/11/11/8P2/11 w - 0 1").unwrap();
        let _ = i.apply(Notation::from("i2i4").unwrap());
        assert_eq!(Some(Position::I3), i.en_passant);

        let mut k = Hexchess::from("1/3/5/7/9/11/11/11/11/11/9P1 w - 0 1").unwrap();
        let _ = k.apply(Notation::from("k1k3").unwrap());
        assert_eq!(Some(Position::K2), k.en_passant);
    }

    #[test]
    fn test_white_promote_pawn_to_queen() {
        let mut hexchess = Hexchess::from("1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f10f11q").unwrap());
        assert_eq!(Some(Piece::WhiteQueen), hexchess.board.get(Position::F11));
    }

    #[test]
    fn test_white_promote_pawn_to_bishop() {
        let mut hexchess = Hexchess::from("1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f10f11b").unwrap());
        assert_eq!(Some(Piece::WhiteBishop), hexchess.board.get(Position::F11));
    }

    #[test]
    fn test_white_promote_pawn_to_rook() {
        let mut hexchess = Hexchess::from("1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f10f11r").unwrap());
        assert_eq!(Some(Piece::WhiteRook), hexchess.board.get(Position::F11));
    }

    #[test]
    fn test_white_promote_pawn_to_knight() {
        let mut hexchess = Hexchess::from("1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f10f11n").unwrap());
        assert_eq!(Some(Piece::WhiteKnight), hexchess.board.get(Position::F11));
    }

    #[test]
    fn test_black_promote_pawn_to_queen() {
        let mut hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f2f1q").unwrap());
        assert_eq!(Some(Piece::BlackQueen), hexchess.board.get(Position::F1));
    }

    #[test]
    fn test_black_promote_pawn_to_bishop() {
        let mut hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f2f1b").unwrap());
        assert_eq!(Some(Piece::BlackBishop), hexchess.board.get(Position::F1));
    }

    #[test]
    fn test_black_promote_pawn_to_rook() {
        let mut hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f2f1r").unwrap());
        assert_eq!(Some(Piece::BlackRook), hexchess.board.get(Position::F1));
    }

    #[test]
    fn test_black_promote_pawn_to_knight() {
        let mut hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1").unwrap();
        let _ = hexchess.apply(Notation::from("f2f1n").unwrap());
        assert_eq!(Some(Piece::BlackKnight), hexchess.board.get(Position::F1));
    }

    #[test]
    fn test_white_cannot_promote_on_black_promotion_position() {
        let mut hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/qP9 w - 0 1").unwrap();
        let result = hexchess.apply(Notation::from("b1a1q").unwrap());
        assert_eq!(Err(Failure::IllegalMove), result);
    }

    #[test]
    fn test_black_cannot_promote_on_white_promotion_position() {
        let mut hexchess = Hexchess::from("1/3/5/7/p8/Q10/11/11/11/11/11 b - 0 1").unwrap();
        let result = hexchess.apply(Notation::from("b7a6q").unwrap());
        assert_eq!(Err(Failure::IllegalMove), result);
    }

    #[test]
    fn test_stringify_hexchess_empty() {
        let hexchess = Hexchess::new();

        assert_eq!(EMPTY_HEXCHESS, hexchess.to_string());
    }

    #[test]
    fn test_stringify_hexchess_initial() {
        let hexchess = Hexchess::initial();

        assert_eq!(INITIAL_HEXCHESS, hexchess.to_string());
    }

    #[test]
    fn test_only_correct_color_can_apply_notation() {
        let mut hexchess = Hexchess::initial();
        
        let result = hexchess.apply(Notation::from("g7g6").unwrap());

        assert_eq!(Err(Failure::OutOfTurn), result);
    }
}
