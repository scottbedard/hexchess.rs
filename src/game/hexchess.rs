use crate::constants::SORTED_POSITIONS;
use crate::game::board::{Board, Position, get_step};
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

    #[serde(rename(deserialize = "enPassant", serialize = "enPassant"))]
    pub en_passant: Option<Position>,

    pub fullmove: u16,

    pub halfmove: u8,

    pub turn: Color,
}

/// Create hexchess from fen
impl Hexchess {
    /// Apply a legal move to the game
    pub fn apply(&mut self, notation: Notation) -> Result<(), String> {
        let piece = match self.board.get(notation.from) {
            Some(val) => val,
            None => return Err(format!("illegal move: {}", notation.to_string())),
        };

        // verify it is the correct piece's turn
        if piece.color() != self.turn {
            return Err("out of turn".to_string());
        }

        // verify the piece can move to the target position
        if !self.targets(notation.from).contains(&notation) {
            return Err(format!("illegal move: {}", notation.to_string()));
        }

        self.apply_unsafe(notation)
    }

    /// get all legal moves
    pub fn all_targets(&self) -> Vec<Notation> {
        let mut targets = vec![];

        self
            .board
            .occupied_by(self.turn)
            .iter()
            .for_each(|&p| targets.append(&mut self.targets(*p)));

        targets
    }

    /// Apply a move to the game, regardless of turn or legality
    pub fn apply_unsafe(&mut self, notation: Notation) -> Result<(), String> {
        let piece = match self.board.get(notation.from) {
            Some(val) => val,
            None => return Err(format!("illegal move: {}", notation.to_string())),
        };

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
                    false => return Err(format!("illegal move: {}", notation.to_string())),
                    true => Some(promotion.to_piece(Color::Black)),
                },
                Piece::WhitePawn => match pawn::is_promotion_position(Color::White, notation.to) {
                    false => return Err(format!("illegal move: {}", notation.to_string())),
                    true => Some(promotion.to_piece(Color::White)),
                },
                _ => return Err(format!("illegal move: {}", notation.to_string())),
            },
        });

        // clear en passant capture
        if Some(notation.to) == self.en_passant {
            let captured = match piece {
                Piece::BlackPawn => get_step(notation.to, 0),
                Piece::WhitePawn => get_step(notation.to, 6),
                _ => None,
            };

            match captured {
                Some(pos) => self.board.set(pos, None),
                None => {},
            };
        }

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

    /// Apply a sequence of moves
    pub fn apply_sequence(&mut self, sequence: &str) -> Result<(), String> {
        let mut clone = self.clone();
        let mut i: u16 = 0;

        for part in sequence.split_whitespace() {
            let notation = match Notation::from(part) {
                Ok(notation) => notation,
                Err(_) => {
                    return Err(format!("invalid notation at index {}: {}", i, part));
                },
            };

            if clone.apply(notation).is_err() {
                return Err(format!("Illegal move at index {}: {}", i, part));
            }

            i += 1;
        }

        self.board = clone.board;
        self.en_passant = clone.en_passant;
        self.fullmove = clone.fullmove;
        self.halfmove = clone.halfmove;
        self.turn = clone.turn;

        Ok(())
    }

    /// Create hexchess from string
    pub fn from(value: &str) -> Result<Self, String> {
        let mut parts = value.split_whitespace();

        let board = match parts.next() {
            Some(part) => match Board::from(part) {
                Ok(result) => result,
                Err(failure) => return Err(failure)
            },
            None => return Err("missing board".to_string()),
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
                    Err(_) => return Err(format!("invalid en passant: {}", part)),
                },
            },
            None => None,
        };

        let halfmove = match parts.next() {
            Some(part) => match part.parse::<u8>() {
                Ok(result) => result,
                Err(_) => return Err(format!("invalid halfmove: {}", part)),
            },
            None => 0,
        };

        let fullmove = match parts.next() {
            Some(part) => match part.parse::<u16>() {
                Ok(result) => match result >= 1 {
                    true => result,
                    false => return Err(format!("invalid fullmove: {}", result)),
                },
                Err(_) => return Err(format!("invalid fullmove: {}", part)),
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

    /// Get the color of a position
    pub fn color(&self, position: Position) -> Option<Color> {
        match self.board.get(position) {
            None => None,
            Some(piece) => Some(piece.color()),
        }
    }

    /// Find the king of a given color
    pub fn find_king(&self, color: Color) -> Option<Position> {
        let king = match color {
            Color::White => Some(Piece::WhiteKing),
            Color::Black => Some(Piece::BlackKing),
        };

        for p in SORTED_POSITIONS.iter() {
            if self.board.get(*p) == king {
                return Some(*p);
            }
        }

        return None
    }

    /// Create hexchess with initial board state
    pub fn initial() -> Self {
        Hexchess {
            board: Board::initial(),
            en_passant: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    /// Test if the board is currently in checkmate
    pub fn is_checkmate(&self) -> bool {
        let king_position = match self.find_king(self.turn) {
            Some(p) => p,
            None => return false,
        };

        if self.is_threatened(king_position) {
            for p in SORTED_POSITIONS.iter() {
                let piece_color = match self.color(*p) {
                    Some(c) => c,
                    None => continue,
                };

                if piece_color != self.turn {
                    continue;
                }

                if self.targets(*p).len() > 0 {
                    return false
                }
            }

            return true
        }

        false
    }

    /// Test if a position is threatened by an enemy piece
    pub fn is_threatened(&self, position: Position) -> bool {
        let piece = match self.board.get(position) {
            Some(val) => val,
            None => return false,
        };

        let enemy_color = match piece.color() {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }; 

        for p in SORTED_POSITIONS.iter() {
            let piece: Piece = match self.board.get(*p) {
                Some(val) => val,
                None => continue,
            };

            if piece.color() != enemy_color {
                continue;
            }
            
            for target in self.targets_unsafe(*p).iter() {
                if target.to == position {
                    return true;
                }
            }
        }

        false
    }

    /// Create an empty hexchess
    pub fn new() -> Self {
        Hexchess {
            board: Board::new(),
            en_passant: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    /// Get the legal targets from a position
    pub fn targets(&self, position: Position) -> Vec<Notation> {
        let color = match self.color(position) {
            Some(val) => val,
            None => return vec![],
        };

        self
            .targets_unsafe(position)
            .into_iter()
            .filter(|&notation| {
                let mut hexchess = self.clone();
                let _ = hexchess.apply_unsafe(notation);

                match hexchess.find_king(color) {
                    Some(p) => !hexchess.is_threatened(p),
                    None => true,
                }
            })
            .collect()
    }

    /// Get all targets from a position, including potential self-checks
    pub fn targets_unsafe(&self, position: Position) -> Vec<Notation> {
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

    /// Stringify a hexchess to JSON
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

        assert_eq!(Err("invalid color: ?".to_string()), hexchess);
    }

    #[test]
    fn test_invalid_en_passant_results_in_error() {
        let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w ? 0 1");

        assert_eq!(Err("invalid en passant: ?".to_string()), hexchess);
    }

    #[test]
    fn test_invalid_halfmove_results_in_error() {
        assert_eq!(Err("invalid halfmove: ?".to_string()), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - ? 1"));
        assert_eq!(Err("invalid halfmove: 0.5".to_string()), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0.5 1"));
        assert_eq!(Err("invalid halfmove: -6".to_string()), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - -6 1"));
    }

    #[test]
    fn test_invalid_fullmove_results_in_error() {
        assert_eq!(Err("invalid fullmove: ?".to_string()), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 ?"));
        assert_eq!(Err("invalid fullmove: 1.5".to_string()), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 1.5"));
        assert_eq!(Err("invalid fullmove: 0".to_string()), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 0")); // <- less than 1
    }

    #[test]
    fn test_convert_hexchess_struct_to_json() {
        let hexchess = Hexchess::new();
        let expected = "{\"board\":{\"a1\":null,\"a2\":null,\"a3\":null,\"a4\":null,\"a5\":null,\"a6\":null,\"b1\":null,\"b2\":null,\"b3\":null,\"b4\":null,\"b5\":null,\"b6\":null,\"b7\":null,\"c1\":null,\"c2\":null,\"c3\":null,\"c4\":null,\"c5\":null,\"c6\":null,\"c7\":null,\"c8\":null,\"d1\":null,\"d2\":null,\"d3\":null,\"d4\":null,\"d5\":null,\"d6\":null,\"d7\":null,\"d8\":null,\"d9\":null,\"e1\":null,\"e10\":null,\"e2\":null,\"e3\":null,\"e4\":null,\"e5\":null,\"e6\":null,\"e7\":null,\"e8\":null,\"e9\":null,\"f1\":null,\"f10\":null,\"f11\":null,\"f2\":null,\"f3\":null,\"f4\":null,\"f5\":null,\"f6\":null,\"f7\":null,\"f8\":null,\"f9\":null,\"g1\":null,\"g10\":null,\"g2\":null,\"g3\":null,\"g4\":null,\"g5\":null,\"g6\":null,\"g7\":null,\"g8\":null,\"g9\":null,\"h1\":null,\"h2\":null,\"h3\":null,\"h4\":null,\"h5\":null,\"h6\":null,\"h7\":null,\"h8\":null,\"h9\":null,\"i1\":null,\"i2\":null,\"i3\":null,\"i4\":null,\"i5\":null,\"i6\":null,\"i7\":null,\"i8\":null,\"k1\":null,\"k2\":null,\"k3\":null,\"k4\":null,\"k5\":null,\"k6\":null,\"k7\":null,\"l1\":null,\"l2\":null,\"l3\":null,\"l4\":null,\"l5\":null,\"l6\":null},\"enPassant\":null,\"fullmove\":1,\"halfmove\":0,\"turn\":\"w\"}";

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

        assert_eq!(Err("illegal move: f5f6".to_string()), result);
    }

    #[test]
    fn test_apply_illegal_move() {
        let mut hexchess = Hexchess::new();

        let result = hexchess.apply(Notation::from("f5a1").unwrap());

        assert_eq!(Err("illegal move: f5a1".to_string()), result);
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
    fn test_apply_clears_white_starboard_en_passant() {
          let mut hexchess = Hexchess::new();
          hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
          hexchess.board.set(Position::G5, Some(Piece::BlackPawn));
          hexchess.turn = Color::White;
          hexchess.en_passant = Some(Position::G6);

          assert_eq!(Some(Piece::BlackPawn), hexchess.board.get(Position::G5));
          
          let _ = hexchess.apply(Notation::from("f6g6").unwrap());

          assert_eq!(None, hexchess.board.get(Position::G5));
    }

    #[test]
    fn test_apply_clears_white_portside_en_passant() {
          let mut hexchess = Hexchess::new();
          hexchess.board.set(Position::E5, Some(Piece::BlackPawn));
          hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
          hexchess.turn = Color::White;
          hexchess.en_passant = Some(Position::E6);

          assert_eq!(Some(Piece::BlackPawn), hexchess.board.get(Position::E5));
          
          let _ = hexchess.apply(Notation::from("f6e6").unwrap());

          assert_eq!(None, hexchess.board.get(Position::E5));
    }

    #[test]
    fn test_apply_clears_black_starboard_en_passant() {
          let mut hexchess = Hexchess::new();
          hexchess.board.set(Position::F6, Some(Piece::BlackPawn));
          hexchess.board.set(Position::E6, Some(Piece::WhitePawn));
          hexchess.turn = Color::Black;
          hexchess.en_passant = Some(Position::E5);

          assert_eq!(Some(Piece::WhitePawn), hexchess.board.get(Position::E6));
          
          let _ = hexchess.apply(Notation::from("f6e5").unwrap());

          assert_eq!(None, hexchess.board.get(Position::E6));
    }

    #[test]
    fn test_apply_clears_black_portside_en_passant() {
          let mut hexchess = Hexchess::new();
          hexchess.board.set(Position::F6, Some(Piece::BlackPawn));
          hexchess.board.set(Position::G6, Some(Piece::WhitePawn));
          hexchess.turn = Color::Black;
          hexchess.en_passant = Some(Position::G5);

          assert_eq!(Some(Piece::WhitePawn), hexchess.board.get(Position::G6));
          
          let _ = hexchess.apply(Notation::from("f6g5").unwrap());

          assert_eq!(None, hexchess.board.get(Position::G6));
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
        assert_eq!(Err("illegal move: b1a1q".to_string()), result);
    }

    #[test]
    fn test_black_cannot_promote_on_white_promotion_position() {
        let mut hexchess = Hexchess::from("1/3/5/7/p8/Q10/11/11/11/11/11 b - 0 1").unwrap();
        let result = hexchess.apply(Notation::from("b7a6q").unwrap());
        assert_eq!(Err("illegal move: b7a6q".to_string()), result);
    }

    #[test]
    fn test_stringify_hexchess_empty() {
        let hexchess = Hexchess::new();

        assert_eq!(crate::constants::EMPTY_HEXCHESS, hexchess.to_string());
    }

    #[test]
    fn test_stringify_hexchess_initial() {
        let hexchess = Hexchess::initial();

        assert_eq!(crate::constants::INITIAL_HEXCHESS, hexchess.to_string());
    }

    #[test]
    fn test_stringify_with_en_passant() {
        let mut hexchess = Hexchess::new();

        hexchess.en_passant = Some(Position::F6);

        assert_eq!("1/3/5/7/9/11/11/11/11/11/11 w f6 0 1", hexchess.to_string());
    }

    #[test]
    fn test_only_correct_color_can_apply_notation() {
        let mut hexchess = Hexchess::initial();
        
        let result = hexchess.apply(Notation::from("g7g6").unwrap());

        assert_eq!(Err("out of turn".to_string()), result);
    }

    #[test]
    fn test_only_pawns_can_be_promoted() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F10, Some(Piece::WhiteRook));
        
        let result = hexchess.apply(Notation::from("f10f11q").unwrap());

        assert_eq!(Err("illegal move: f10f11q".to_string()), result);
    }

    #[test]
    fn test_creating_hexchess_with_invalid_board_returns_failure() {
        let hexchess = Hexchess::from("whoops");

        assert_eq!(Err("invalid board: whoops".to_string()), hexchess);
    }

    #[test]
    fn test_creating_hexchess_with_empty_string_returns_failure() {
        let hexchess = Hexchess::from("");

        assert_eq!(Err("missing board".to_string()), hexchess);
    }

    #[test]
    fn test_color() {
        let hexchess = Hexchess::initial();

        assert_eq!(None, hexchess.color(Position::A1));

        assert_eq!(Some(Color::White), hexchess.color(Position::B1)); // P
        assert_eq!(Some(Color::White), hexchess.color(Position::C1)); // R
        assert_eq!(Some(Color::White), hexchess.color(Position::D1)); // N
        assert_eq!(Some(Color::White), hexchess.color(Position::E1)); // Q
        assert_eq!(Some(Color::White), hexchess.color(Position::F1)); // B
        assert_eq!(Some(Color::White), hexchess.color(Position::G1)); // K

        assert_eq!(Some(Color::Black), hexchess.color(Position::B7)); // p
        assert_eq!(Some(Color::Black), hexchess.color(Position::C8)); // r
        assert_eq!(Some(Color::Black), hexchess.color(Position::D9)); // n
        assert_eq!(Some(Color::Black), hexchess.color(Position::E10)); // q
        assert_eq!(Some(Color::Black), hexchess.color(Position::F11)); // b
        assert_eq!(Some(Color::Black), hexchess.color(Position::G10)); // k
    }

    #[test]
    fn test_position_is_threatened_by_enemy_piece() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::G10, Some(Piece::WhiteKing));
        hexchess.board.set(Position::G1, Some(Piece::BlackRook));

        assert_eq!(true, hexchess.is_threatened(Position::G10));
    }

    #[test]
    fn test_position_is_not_threatened_by_friendly_piece() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhiteQueen));
        hexchess.board.set(Position::F7, Some(Piece::WhiteKing));

        assert_eq!(false, hexchess.is_threatened(Position::F7));
    }

    #[test]
    fn test_unattacked_position_is_not_threatened() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::G10, Some(Piece::WhiteKing));

        assert_eq!(false, hexchess.is_threatened(Position::G10));
    }

    #[test]
    fn test_position_is_threatened_while_attacking_or_defending() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F6, Some(Piece::WhiteKing));
        hexchess.board.set(Position::F7, Some(Piece::BlackQueen));

        hexchess.turn = Color::Black;
        assert_eq!(true, hexchess.is_threatened(Position::F6));

        hexchess.turn = Color::White;
        assert_eq!(true, hexchess.is_threatened(Position::F6));
    }

    #[test]
    fn test_cannot_step_out_of_a_pin() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F7, Some(Piece::WhiteKing));
        hexchess.board.set(Position::F6, Some(Piece::WhiteRook));
        hexchess.board.set(Position::F5, Some(Piece::BlackQueen));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(1, targets.len());
        assert_eq!(Position::F5, targets[0].to);
    }

    #[test]
    fn test_cannot_self_check_on_opponents_turn() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F7, Some(Piece::WhiteKing));
        hexchess.board.set(Position::F6, Some(Piece::WhiteRook));
        hexchess.board.set(Position::F5, Some(Piece::BlackQueen));
        hexchess.turn = Color::Black;

        let targets = hexchess.targets(Position::F6);

        assert_eq!(1, targets.len());
        assert_eq!(Position::F5, targets[0].to);
    }

    #[test]
    fn test_king_cannot_step_into_check() {
        let mut hexchess = Hexchess::new();
        hexchess.board.set(Position::F11, Some(Piece::WhiteKing));
        hexchess.board.set(Position::F9, Some(Piece::BlackQueen));

        let targets = hexchess.targets(Position::F11);

        assert!(targets.is_empty());
    }

    #[test]
    fn test_targets_of_unoccupied_position() {
        assert!(Hexchess::new().targets(Position::A1).is_empty());
        assert!(Hexchess::new().targets_unsafe(Position::A1).is_empty());
    }

    #[test]
    fn test_move_to_en_passant_position_with_non_pawn_doesnt_remove_enemy_piece()
    {
        let mut hexchess = Hexchess::initial();
        let _ = hexchess.apply(Notation::from("b1b3").unwrap());
        let _ = hexchess.apply(Notation::from("f10b2").unwrap());

        assert_eq!(Some(Piece::WhitePawn), hexchess.board.get(Position::B3));
        assert_eq!(Some(Piece::BlackBishop), hexchess.board.get(Position::B2));
    }

    #[test]
    fn test_is_checkmate() {
        let mut hexchess = Hexchess::from("K/3/2q2/1q5/9/11/11/11/11/11/11 b - 0 1").unwrap();
        
        assert_eq!(false, hexchess.is_checkmate());

        let _ = hexchess.apply(Notation::from("d8f10").unwrap());

        assert_eq!(true, hexchess.is_checkmate());
    }

    #[test]
    fn test_not_checkmate() {
        let mut hexchess = Hexchess::from("b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1").unwrap();
        hexchess.turn = Color::Black;

        assert_eq!(false, hexchess.is_checkmate());
    }

    #[test]
    fn test_find_king() {
        let hexchess = Hexchess::initial();
        let blank = Hexchess::new();

        assert_eq!(Some(Position::G1), hexchess.find_king(Color::White));
        assert_eq!(Some(Position::G10), hexchess.find_king(Color::Black));
        assert_eq!(None, blank.find_king(Color::White));
        assert_eq!(None, blank.find_king(Color::Black));
    }

    #[test]
    fn test_applying_a_sequence_of_moves() {
        let mut hexchess = Hexchess::initial();
        let _ = hexchess.apply_sequence("g4g6 f7g6 f5f7 g6f6");

        assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3");
    }

    #[test]
    fn test_apply_sequence_with_invalid_move() {
        let mut hexchess = Hexchess::initial();
        let result = hexchess.apply_sequence("g4g5 whoops");

        assert_eq!(hexchess.to_string(), crate::constants::INITIAL_HEXCHESS); // <- the board has not changed
        assert_eq!(Err(String::from("invalid notation at index 1: whoops")), result);
    }

    #[test]
    fn test_apply_sequence_with_illegal_move() {
        let mut hexchess = Hexchess::initial();
        let result = hexchess.apply_sequence("g4g5 b7a1"); // <- b7 is a black pawn, it cannot move to a1

        assert_eq!(hexchess.to_string(), crate::constants::INITIAL_HEXCHESS); // <- the board has not changed
        assert_eq!(Err(String::from("Illegal move at index 1: b7a1")), result);
    }

    #[test]
    fn test_apply_sequence_that_attempts_a_self_check() {
        let mut hexchess = Hexchess::initial();
        let result = hexchess.apply_sequence("f2d4 g10g9");

        assert_eq!(hexchess.to_string(), crate::constants::INITIAL_HEXCHESS);
        assert_eq!(Err(String::from("Illegal move at index 1: g10g9")), result);
    }
}
