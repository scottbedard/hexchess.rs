use crate::h;
use crate::hexchess::pieces::king::king_moves_unsafe;
use crate::hexchess::pieces::knight::knight_moves_unsafe;
use crate::hexchess::pieces::pawn::pawn_moves_unsafe;
use crate::hexchess::pieces::straight_line::straight_line_moves_unsafe;
use crate::hexchess::san::San;
use serde_with::serde_as;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

use crate::constants::{
    Color,
    INITIAL_POSITION,
    Piece,
    PromotionPiece,
};

use crate::hexchess::utils::{
    get_color,
    is_legal_en_passant,
    step,
    index,
    to_position,
};

/// Hexchess game state
#[serde_as]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi, type_suffix = "Struct")]
pub struct Hexchess {
    #[tsify(type = "Board")]
    #[serde_as(as = "[_; 91]")]
    pub board: [Option<Piece>; 91],

    pub ep: Option<u8>,

    pub fullmove: u16,

    pub halfmove: u8,

    #[tsify(type = "Color")]
    pub turn: Color,
}

impl Hexchess {
    /// apply a whitespace separated sequence of moves
    pub fn apply(&mut self, sequence: &str) -> Result<(), String> {
        let mut clone = self.clone();
        let mut i: u32 = 0;

        for part in sequence.split_whitespace() {
            let san = match San::from(&part.to_string()) {
                Ok(san) => san,
                Err(_) => {
                    return Err(format!("invalid san at index {}: {}", i, part));
                },
            };

            if clone.apply_move(&san).is_err() {
                return Err(format!("illegal move at index {}: {}", i, part));
            }

            i += 1;
        }

        self.board = clone.board;
        self.turn = clone.turn;
        self.ep = clone.ep;
        self.fullmove = clone.fullmove;
        self.halfmove = clone.halfmove;

        Ok(())
    }

    /// apply legal move
    pub fn apply_move(&mut self, san: &San) -> Result<(), String> {
        if !self.is_legal(san) {
            return Err(format!("illegal move: {:?}", san));
        }

        self.apply_move_unsafe(san);

        Ok(())
    }

    /// apply move, regardless of turn or legality
    pub fn apply_move_unsafe(&mut self, san: &San) -> &Self {
        let piece = match self.board[san.from as usize] {
            Some(piece) => piece,
            None => panic!("cannot apply move from empty position: {}", san.from),
        };

        // update halfmove
        if self.board[san.to as usize].is_some() || (
            piece == Piece::BlackPawn ||
            piece == Piece::WhitePawn
        ) {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }

        let color = get_color(&piece);

        // update fullmove and turn color
        if color == Color::Black {
            self.fullmove += 1;
            self.turn = Color::White;
        } else {
            self.turn = Color::Black;
        }

        // set from positions
        self.board[san.from as usize] = None;

        // set to position
        self.board[san.to as usize] = Some(
            match san.promotion {
                None => piece,
                Some(piece) => match color {
                    Color::Black => match piece {
                        PromotionPiece::Bishop => Piece::BlackBishop,
                        PromotionPiece::Knight => Piece::BlackKnight,
                        PromotionPiece::Queen => Piece::BlackQueen,
                        PromotionPiece::Rook => Piece::BlackRook,
                    },
                    Color::White => match piece {
                        PromotionPiece::Bishop => Piece::WhiteBishop,
                        PromotionPiece::Knight => Piece::WhiteKnight,
                        PromotionPiece::Queen => Piece::WhiteQueen,
                        PromotionPiece::Rook => Piece::WhiteRook,
                    },
                },
            }
        );

        // clear captured en passant
        if Some(san.to) == self.ep {
            let captured = match piece {
                Piece::BlackPawn => step(san.to, 0),
                Piece::WhitePawn => step(san.to, 6),
                _ => None,
            };

            match captured {
                Some(position) => self.board[position as usize] = None,
                None => {},
            };
        }

        // set en passsant
        self.ep = match piece {
            Piece::BlackPawn => match (san.from, san.to) {
                (h!("c7"), h!("c5")) => Some(h!("c6")),
                (h!("d7"), h!("d5")) => Some(h!("d6")),
                (h!("e7"), h!("e5")) => Some(h!("e6")),
                (h!("f7"), h!("f5")) => Some(h!("f6")),
                (h!("g7"), h!("g5")) => Some(h!("g6")),
                (h!("h7"), h!("h5")) => Some(h!("h6")),
                (h!("i7"), h!("i5")) => Some(h!("i6")),
                (h!("k7"), h!("k5")) => Some(h!("k6")),
                _ => None,
            },
            Piece::WhitePawn => match (san.from, san.to) {
                (h!("c2"), h!("c4")) => Some(h!("c3")),
                (h!("d3"), h!("d5")) => Some(h!("d4")),
                (h!("e4"), h!("e6")) => Some(h!("e5")),
                (h!("f5"), h!("f7")) => Some(h!("f6")),
                (h!("g4"), h!("g6")) => Some(h!("g5")),
                (h!("h3"), h!("h5")) => Some(h!("h4")),
                (h!("i2"), h!("i4")) => Some(h!("i3")),
                (h!("k1"), h!("k3")) => Some(h!("k2")),
                _ => None,
            },
            _ => None,
        };

        self
    }

    /// get legal moves for current turn
    pub fn current_moves(&self) -> Vec<San> {
        let mut result: Vec<San> = vec![];

        for n in self.get_color(self.turn) {
            result.extend(self.moves_from(n));
        }

        result
    }

    /// get piece at position
    pub fn get(&self, position: &str) -> Option<Piece> {
        match index(position) {
            Ok(index) => self.board[index as usize],
            Err(_) => None,
        }
    }

    /// get positions occupied by a color
    pub fn get_color(&self, color: Color) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        for (index, piece) in self.board.iter().enumerate() {
            match piece {
                Some(piece) => match get_color(piece) == color {
                    true => result.push(index as u8),
                    false => continue,
                },
                None => continue,
            };
        }

        result
    }

    /// get legal moves a position
    pub fn moves_from(&self, from: u8) -> Vec<San> {
        let piece = match self.board[from as usize] {
            Some(piece) => piece,
            None => return vec![],
        };

        let color = get_color(&piece);

        self.moves_from_unsafe(from)
            .into_iter()
            .filter(|san| {
                // prevent self check
                let mut clone = self.clone();

                clone.apply_move_unsafe(san);
                
                match clone.find_king(color) {
                    Some(king) => !clone.is_threatened(king),
                    None => true,
                }
            })
            .collect()
    }

    /// get moves from a position, regardless of turn or legality
    pub fn moves_from_unsafe(&self, from: u8) -> Vec<San> {
        let mut result: Vec<San> = vec![];

        let piece = match self.board[from as usize] {
            Some(piece) => piece,
            None => return result,
        };
        
        let color = get_color(&piece);

        result.extend(match piece {
            Piece::BlackKing | Piece::WhiteKing => {
                king_moves_unsafe(&self, from, &color)
            },
            Piece::BlackKnight | Piece::WhiteKnight => {
                knight_moves_unsafe(&self, from, &color)
            },
            Piece::BlackPawn | Piece::WhitePawn => {
                pawn_moves_unsafe(&self, from, &color)
            },
            Piece::BlackBishop | Piece::WhiteBishop => {
                straight_line_moves_unsafe(&self, &from, &color, &[1, 3, 5, 7, 9, 11])
            },
            Piece::BlackRook | Piece::WhiteRook => {
                straight_line_moves_unsafe(&self, &from, &color, &[0, 2, 4, 6, 8, 10])
            },
            Piece::BlackQueen | Piece::WhiteQueen => {
                straight_line_moves_unsafe(&self, &from, &color, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
            }
        });
        
        result
    }

    /// create a new hexchess instance
    pub fn new() -> Self {
        Self {
            board: [None; 91],
            ep: None,
            fullmove: 1,
            halfmove: 0,
            turn: Color::White,
        }
    }

    /// find king by color
    pub fn find_king(&self, color: Color) -> Option<u8> {
        let king = match color {
            Color::Black => Piece::BlackKing,
            Color::White => Piece::WhiteKing,
        };

        for (index, piece) in self.board.iter().enumerate() {
            if piece == &Some(king) {
                return Some(index as u8);
            }
        }

        None
    }

    /// initialize a hexchess instance to the starting position
    pub fn init() -> Self {
        Self::parse(INITIAL_POSITION).unwrap()
    }

    /// test if the board is in check
    pub fn is_check(&self) -> bool {
        let king = match self.find_king(self.turn) {
            Some(king) => king,
            None => return false
        };

        let opposite_turn = match self.turn {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };

        for n in self.get_color(opposite_turn) {
            for san in self.moves_from_unsafe(n) {
                if san.to == king {
                    return true
                }
            }
        }
        
        false
    }

    /// test if the board is in checkmate
    pub fn is_checkmate(&self) -> bool {
        self.is_check() && self.current_moves().len() == 0
    }

    /// test if move is legal
    pub fn is_legal(&self, san: &San) -> bool {
        let piece = match self.board[san.from as usize] {
            Some(piece) => piece,
            None => return false,
        };
        
        if get_color(&piece) != self.turn {
            return false;
        }

        self.moves_from(san.from)
            .iter()
            .any(|move_san| move_san == san)
    }

    /// test if the board is in stalemate
    pub fn is_stalemate(&self) -> bool {
        !self.is_check() && self.current_moves().len() == 0
    }

    /// test if position is threatened
    pub fn is_threatened(&self, position: u8) -> bool {
        let threatened_piece = match self.board[position as usize] {
            Some(piece) => piece,
            None => return false,
        };

        let color = get_color(&threatened_piece);

        for n in 0u8..91u8 {
            match self.board[n as usize] {
                Some(piece) => match color == get_color(&piece) {
                    true => continue,
                    false => {
                        for san in self.moves_from_unsafe(n) {
                            if san.to == position {
                                return true
                            }
                        }
                    }
                },
                None => continue,
            };
        }

        false
    }

    /// create hexchess instance from fen
    pub fn parse(source: &str) -> Result<Self, String> {
        let mut parts = source.split_whitespace();

        let board = match parts.next() {
            Some(part) => match parse_board(&part.to_string()) {
                Ok(result) => result,
                Err(failure) => return Err(failure),
            }
            _ => return Err("board not found".to_string()),
        };

        let turn = match parts.next() {
            Some(part) => match part {
                "b" => Color::Black,
                "w" => Color::White,
                _ => return Err(format!("invalid turn color: {}", part)),
            },
            None => Color::White,
        };

        let ep = match parts.next() {
            Some(part) => match part {
                "-" => None,
                _ => match index(&part) {
                    Ok(result) => match is_legal_en_passant(&result) {
                        true => Some(result),
                        false => return Err(format!("illegal en passant position: {}", part)),
                    },
                    Err(_) => return Err(format!("invalid en passant position: {}", part)),
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
                    false => return Err(format!("invalid fullmove: {}", part)),
                },
                Err(_) => return Err(format!("invalid fullmove: {}", part)),
            },
            None => 1,
        };

        Ok(Self {
            board,
            ep,
            fullmove,
            halfmove,
            turn,
        })
    }

    /// format as fen string
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            stringify_board(&self.board),
            match self.turn {
                Color::Black => 'b',
                Color::White => 'w',
            },
            match self.ep {
                Some(ep) => to_position(&ep),
                None => "-",
            },
            self.halfmove,
            self.fullmove,
        )
    }
}

/// parse the board segment of fen
fn parse_board(source: &String) -> Result<[Option<Piece>; 91], String> {
    let mut arr: [Option<Piece>; 91] = [None; 91];
    let mut black = false;
    let mut white = false;
    let mut fen_index: u8 = 0;

    for (index, current) in source.chars().enumerate() {
        match current {
            '/' => continue,
            '0' => continue,
            '1' => match source.chars().nth(index as usize + 1) {
                Some('0') | Some('1') => fen_index += 10,
                _ => fen_index += 1,
            },
            '2' => fen_index += 2,
            '3' => fen_index += 3,
            '4' => fen_index += 4,
            '5' => fen_index += 5,
            '6' => fen_index += 6,
            '7' => fen_index += 7,
            '8' => fen_index += 8,
            '9' => fen_index += 9,
            'b' | 'B' | 'n' | 'N' | 'p' | 'P' | 'Q' | 'q' | 'r' | 'R' => {
                // // it's safe to unwrap current because our match already checks for it
                arr[fen_index as usize] = Some(to_piece(current).unwrap());

                fen_index += 1;
            }
            'k' => {
                if black {
                    return Err("multiple black kings".to_string());
                }

                arr[fen_index as usize] = Some(Piece::BlackKing);
                black = true;
                fen_index += 1;
            }
            'K' => {
                if white {
                    return Err("multiple white kings".to_string());
                }

                arr[fen_index as usize] = Some(Piece::WhiteKing);
                white = true;
                fen_index += 1;
            },
            _ => return Err(format!("invalid character at index {}: {}", index, current)),
        }
    }

    if fen_index != 91 {
        return Err("board overflow".to_string());
    }

    Ok(arr)
}

/// format the board section of a fen
fn stringify_board(board: &[Option<Piece>; 91]) -> String {
    let mut blank: u8 = 0;
    let mut index: u8 = 0;
    let mut result = String::new();

    for val in board.iter() {
        match val {
            None => {
                blank += 1;
            },
            Some(piece) => {
                if blank > 0 {
                    result.push_str(&blank.to_string());
                    blank = 0;
                }

                result.push(match piece {
                    Piece::BlackBishop => 'b',
                    Piece::BlackKing => 'k',
                    Piece::BlackKnight => 'n',
                    Piece::BlackPawn => 'p',
                    Piece::BlackQueen => 'q',
                    Piece::BlackRook => 'r',
                    Piece::WhiteBishop => 'B',
                    Piece::WhiteKing => 'K',
                    Piece::WhiteKnight => 'N',
                    Piece::WhitePawn => 'P',
                    Piece::WhiteQueen => 'Q',
                    Piece::WhiteRook => 'R',
                });
            },
        };

        match index {
            0 | 3 | 8 | 15 | 24 | 35 | 46 | 57 | 68 | 79 => {
                if blank > 0 {
                    result.push_str(&blank.to_string());
                }

                result.push('/');
                blank = 0;
            },
            _ => {}
        };

        index += 1;
    }

    if blank > 0 {
        result.push_str(&blank.to_string());
    }

    result
}

/// convert character to piece
fn to_piece(source: char) -> Result<Piece, &'static str> {
    match source {
        'p' => Ok(Piece::BlackPawn),
        'n' => Ok(Piece::BlackKnight),
        'b' => Ok(Piece::BlackBishop),
        'r' => Ok(Piece::BlackRook),
        'q' => Ok(Piece::BlackQueen),
        'k' => Ok(Piece::BlackKing),
        'P' => Ok(Piece::WhitePawn),
        'N' => Ok(Piece::WhiteKnight),
        'B' => Ok(Piece::WhiteBishop),
        'R' => Ok(Piece::WhiteRook),
        'Q' => Ok(Piece::WhiteQueen),
        'K' => Ok(Piece::WhiteKing),
        _ => Err("invalid_piece_character")
    }
}

#[cfg(test)]
mod tests {
    use crate::{h, s};
    use super::*;

    mod apply {
        use super::*;

        #[test]
        fn test_applying_a_sequence_of_moves() {
            let mut hexchess = Hexchess::init();
            let _ = hexchess.apply("g4g6 f7g6 f5f7 g6f6");

            assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3");
        }

        #[test]
        fn test_apply_with_invalid_san() {
            let mut hexchess = Hexchess::init();
            let result = hexchess.apply("whoops");

            assert_eq!(result, Err("invalid san at index 0: whoops".to_string()));
        }

        #[test]
        fn test_apply_with_illegal_move() {
            let mut hexchess = Hexchess::init();
            let result = hexchess.apply("g4g5 a6a5");

            assert_eq!(result, Err("illegal move at index 1: a6a5".to_string()));
        }
    }

    mod apply_move {
        use super::*;

        #[test]
        fn sets_to_and_from_positions() {
            let mut hexchess = Hexchess::init();
            let _ = hexchess.apply_move(&s!("g4g5"));
            let _ = hexchess.apply_move(&s!("e7e6"));
            assert_eq!(hexchess.board[h!("g5")], Some(Piece::WhitePawn));
            assert_eq!(hexchess.board[h!("g4")], None);
            assert_eq!(hexchess.board[h!("e6")], Some(Piece::BlackPawn));
            assert_eq!(hexchess.board[h!("e7")], None);
        }

        #[test]
        fn clears_en_pasant_capture_white() {
            let mut hexchess = Hexchess::parse("b/qbk/n1b1n/r5r/ppppp1ppp/5P5/6p4/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w g6 0 2").unwrap();
            let _ = hexchess.apply_move(&s!("f6g6"));

            assert_eq!(hexchess.get("g5"), None);
        }

        #[test]
        fn clears_en_passant_capture_black() {
            let mut hexchess = Hexchess::parse("b/qbk/n1b1n/r5r/pppp1pppp/5pP4/4PP5/11/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 2").unwrap();
            let _ = hexchess.apply_move(&s!("f6g5"));

            assert_eq!(hexchess.get("g6"), None);
        }

        #[test]
        fn only_pawns_capture_en_passant() {
            let mut hexchess = Hexchess::parse("b/qbk/n1b1n/r5r/ppppp1ppp/11/5Pp4/4P1PB3/3P1B1P3/2P5P2/1PRNQBKNRP1 w g6 0 2").unwrap();
            let _ = hexchess.apply_move(&s!("h4g6")); // <- bishop to en passant
            
            assert_eq!(hexchess.get("g5"), Some(Piece::BlackPawn));
        }

        #[test]
        fn alternate_color_back_and_forth() {
            let mut hexchess = Hexchess::init();

            assert_eq!(hexchess.turn, Color::White);
            let _ = hexchess.apply_move(&s!("g4g5"));
            assert_eq!(hexchess.turn, Color::Black);
            let _ = hexchess.apply_move(&s!("e7e6"));
            assert_eq!(hexchess.turn, Color::White);
            let _ = hexchess.apply_move(&s!("f5f6"));
            assert_eq!(hexchess.turn, Color::Black);
        }

        #[test]
        fn sets_and_unsets_en_passant() {
            let mut hexchess = Hexchess::init();

            let _ = hexchess.apply_move(&s!("g4g6"));
            assert_eq!(hexchess.ep, Some(h!("g5")));

            let _ = hexchess.apply_move(&s!("e7e5"));
            assert_eq!(hexchess.ep, Some(h!("e6")));

            let _ = hexchess.apply_move(&s!("b1b2"));
            assert_eq!(hexchess.ep, None);
        }

        #[test]
        fn sets_halfmove_and_fullmove() {
            let mut hexchess = Hexchess::init();

            assert_eq!(hexchess.halfmove, 0);
            assert_eq!(hexchess.fullmove, 1);

            let _ = hexchess.apply_move(&s!("e4e5"));
            assert_eq!(hexchess.halfmove, 0);
            assert_eq!(hexchess.fullmove, 1);

            let _ = hexchess.apply_move(&s!("f7f6"));
            assert_eq!(hexchess.halfmove, 0);
            assert_eq!(hexchess.fullmove, 2);

            let _ = hexchess.apply_move(&s!("f3c6"));
            assert_eq!(hexchess.halfmove, 1);
            assert_eq!(hexchess.fullmove, 2);

            let _ = hexchess.apply_move(&s!("i8h8"));
            assert_eq!(hexchess.halfmove, 2);
            assert_eq!(hexchess.fullmove, 3);

            let _ = hexchess.apply_move(&s!("c6e10"));
            assert_eq!(hexchess.halfmove, 0);
            assert_eq!(hexchess.fullmove, 3);
        }

        // promote white pieces
        #[test]
        fn white_and_black_promotions() {
            let mut hexchess = Hexchess::parse("1/3/1P1P1/7/1P5P1/11/11/11/11/2p1p1p1p2/11 w - 0 1").unwrap();

            let _ = hexchess.apply_move(&s!("c7c8r"));
            assert_eq!(hexchess.board[h!("c8")], Some(Piece::WhiteRook));

            let _ = hexchess.apply_move(&s!("c2c1r"));
            assert_eq!(hexchess.board[h!("c1")], Some(Piece::BlackRook));

            let _ = hexchess.apply_move(&s!("e9e10b"));
            assert_eq!(hexchess.board[h!("e10")], Some(Piece::WhiteBishop));

            let _ = hexchess.apply_move(&s!("e2e1b"));
            assert_eq!(hexchess.board[h!("e1")], Some(Piece::BlackBishop));

            let _ = hexchess.apply_move(&s!("g9g10q"));
            assert_eq!(hexchess.board[h!("g10")], Some(Piece::WhiteQueen));

            let _ = hexchess.apply_move(&s!("g2g1q"));
            assert_eq!(hexchess.board[h!("g1")], Some(Piece::BlackQueen));

            let _ = hexchess.apply_move(&s!("i7i8n"));
            assert_eq!(hexchess.board[h!("i8")], Some(Piece::WhiteKnight));

            let _ = hexchess.apply_move(&s!("i2i1n"));
            assert_eq!(hexchess.board[h!("i1")], Some(Piece::BlackKnight));
        }
    
        #[test]
        fn errors_on_illegal_move() {
            let mut hexchess = Hexchess::init();

            assert_eq!(hexchess.apply_move(&s!("a4a5")).is_err(), true);
        }

        #[test]
        #[should_panic]
        fn apply_move_unsafe_panics_on_empty_positions() {
            let mut hexchess = Hexchess::init();

            hexchess.apply_move_unsafe(&s!("a4a5"));
        }
    }

    #[test]
    fn test_clone() {
        let hexchess = Hexchess::init();
        let clone = hexchess.clone();

        assert_eq!(clone.board, hexchess.board);
        assert_eq!(clone.ep, hexchess.ep);
        assert_eq!(clone.turn, hexchess.turn);
        assert_eq!(clone.halfmove, hexchess.halfmove);  
        assert_eq!(clone.fullmove, hexchess.fullmove);
    }

    #[test]
    fn test_apply_move_unsafe() {
        let mut hexchess = Hexchess::init();

        hexchess.apply_move_unsafe(&s!("b1b6")); // <- illegal pawn move

        assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/1P9/5P5/4P1P4/3P1B1P3/2P2B2P2/2RNQBKNRP1 b - 0 1");
    }

    #[test]
    fn test_current_moves() {
        let hexchess = Hexchess::init();
        let result = hexchess.current_moves().iter().map(|s| s.to_string()).collect::<Vec<String>>();

        assert_eq!(result.len(), 51);
        assert_eq!(result[0], "f5f6");
        assert_eq!(result[1], "e4e5");
        assert_eq!(result[2], "e4e6");
        assert_eq!(result[3], "g4g5");
        assert_eq!(result[4], "g4g6");
        assert_eq!(result[5], "d3d4");
        assert_eq!(result[6], "d3d5");
        assert_eq!(result[7], "f3h2");
        assert_eq!(result[8], "f3d2");
        assert_eq!(result[9], "h3h4");
        assert_eq!(result[10], "h3h5");
        assert_eq!(result[11], "c2c3");
        assert_eq!(result[12], "c2c4");
        assert_eq!(result[13], "f2g3");
        assert_eq!(result[14], "f2h4");
        assert_eq!(result[15], "f2i5");
        assert_eq!(result[16], "f2k6");
        assert_eq!(result[17], "f2e3");
        assert_eq!(result[18], "f2d4");
        assert_eq!(result[19], "f2c5");
        assert_eq!(result[20], "f2b6");
        assert_eq!(result[21], "i2i3");
        assert_eq!(result[22], "i2i4");
        assert_eq!(result[23], "b1b2");
        assert_eq!(result[24], "b1b3");
        assert_eq!(result[25], "c1d2");
        assert_eq!(result[26], "c1e3");
        assert_eq!(result[27], "c1f4");
        assert_eq!(result[28], "d1f4");
        assert_eq!(result[29], "d1g2");
        assert_eq!(result[30], "d1b2");
        assert_eq!(result[31], "d1c3");
        assert_eq!(result[32], "e1e2");
        assert_eq!(result[33], "e1e3");
        assert_eq!(result[34], "e1d2");
        assert_eq!(result[35], "e1c3");
        assert_eq!(result[36], "e1b4");
        assert_eq!(result[37], "e1a5");
        assert_eq!(result[38], "f1g2");
        assert_eq!(result[39], "f1e2");
        assert_eq!(result[40], "g1g2");
        assert_eq!(result[41], "g1h2");
        assert_eq!(result[42], "h1i3");
        assert_eq!(result[43], "h1k2");
        assert_eq!(result[44], "h1e2");
        assert_eq!(result[45], "h1f4");
        assert_eq!(result[46], "i1h2");
        assert_eq!(result[47], "i1g3");
        assert_eq!(result[48], "i1f4");
        assert_eq!(result[49], "k1k2");
        assert_eq!(result[50], "k1k3");
    }

    #[test]
    fn find_kings_by_color() {
        let hexchess = Hexchess::init();

        assert_eq!(hexchess.find_king(Color::Black), Some(h!("g10")));
        assert_eq!(hexchess.find_king(Color::White), Some(h!("g1")));
    }

    #[test]
    fn test_get() {
        let hexchess = Hexchess::init();

        assert_eq!(hexchess.get("g10"), Some(Piece::BlackKing));
        assert_eq!(hexchess.get("g1"), Some(Piece::WhiteKing));
        assert_eq!(hexchess.get("a4"), None);
        assert_eq!(hexchess.get("whoops"), None);
    }

    #[test]
    fn get_color() {
        let hexchess = Hexchess::init();
        let results = hexchess.get_color(Color::Black);

        assert_eq!(results.len(), 18);
        assert_eq!(results[0], h!("f11"));
        assert_eq!(results[1], h!("e10"));
        assert_eq!(results[2], h!("f10"));
        assert_eq!(results[3], h!("g10"));
        assert_eq!(results[4], h!("d9"));
        assert_eq!(results[5], h!("f9"));
        assert_eq!(results[6], h!("h9"));
        assert_eq!(results[7], h!("c8"));
        assert_eq!(results[8], h!("i8"));
        assert_eq!(results[9], h!("b7"));
        assert_eq!(results[10], h!("c7"));
        assert_eq!(results[11], h!("d7"));
        assert_eq!(results[12], h!("e7"));
        assert_eq!(results[13], h!("f7"));
        assert_eq!(results[14], h!("g7"));
        assert_eq!(results[15], h!("h7"));
        assert_eq!(results[16], h!("i7"));
        assert_eq!(results[17], h!("k7"));
    }

    mod is_check {
        use super::*;

        #[test]
        fn no_king() {
            let hexchess = Hexchess::new();

            assert_eq!(hexchess.is_check(), false);
        }

        #[test]
        fn not_in_check() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_check(), false);
        }

        #[test]
        fn in_check() {
            let hexchess = Hexchess::parse("K/3/5/7/9/5r5/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_check(), true);
        }
    }

    #[test]
    fn is_checkmate() {
        let mut hexchess = Hexchess::parse("K/3/5/3q3/2q6/11/11/11/11/11/11 b - 0 1").unwrap();

        assert_eq!(hexchess.is_checkmate(), false);
  
        let _ = hexchess.apply_move(&s!("d7f9"));
  
        assert_eq!(hexchess.is_checkmate(), true);
    }

    mod is_legal {
        use super::*;

        #[test]
        fn legal_move() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.is_legal(&s!("g4g5")), true);
        }

        #[test]
        fn illegal_move() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.is_legal(&s!("b1b4")), false);
        }

        #[test]
        fn illegal_move_out_of_turn() {
            let mut hexchess = Hexchess::init();

            assert_eq!(hexchess.is_legal(&s!("g7g6")), false);

            hexchess.turn = Color::Black;

            assert_eq!(hexchess.is_legal(&s!("g7g6")), true);
        }

        #[test]
        fn white_cannot_promote_on_blacks_positions() {
            let hexchess = Hexchess::parse("1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr w - 0 1").unwrap();

            let b1b2 = San { from: h!("b1"), to: h!("b2"), promotion: None };
            let b1b2q = San { from: h!("b1"), to: h!("b2"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&b1b2), true);
            assert_eq!(hexchess.is_legal(&b1b2q), false);

            let k1l1 = San { from: h!("k1"), to: h!("l1"), promotion: None };
            let k1l1q = San { from: h!("k1"), to: h!("l1"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&k1l1), true);
            assert_eq!(hexchess.is_legal(&k1l1q), false);
        }

        #[test]
        fn black_cannot_promote_on_whites_positions() {
            let hexchess = Hexchess::parse("1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr b - 0 1").unwrap();

            let b7a6 = San { from: h!("b7"), to: h!("a6"), promotion: None };
            let b7a6q = San { from: h!("b7"), to: h!("a6"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&b7a6), true);
            assert_eq!(hexchess.is_legal(&b7a6q), false);

            let k7l6 = San { from: h!("k7"), to: h!("l6"), promotion: None };
            let k7l6q = San { from: h!("k7"), to: h!("l6"), promotion: Some(PromotionPiece::Queen) };
            assert_eq!(hexchess.is_legal(&k7l6), true);
            assert_eq!(hexchess.is_legal(&k7l6q), false);
        }

        #[test]
        fn pawn_must_promote_on_final_rank() {
            let mut hexchess = Hexchess::parse("1/1P1/5/7/9/11/11/11/11/5p5/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_legal(&s!("f10f11")), false);
            assert_eq!(hexchess.is_legal(&s!("f10f11q")), true);

            hexchess.turn = Color::Black;

            assert_eq!(hexchess.is_legal(&s!("f2f1")), false);
            assert_eq!(hexchess.is_legal(&s!("f2f1q")), true);
        }
    }

    #[test]
    fn is_stalemate() {
        let mut hexchess = Hexchess::parse("k/1P1/5/3K3/9/11/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(hexchess.is_stalemate(), false);
  
        let _ = hexchess.apply_move(&s!("f8f9"));
  
        assert_eq!(hexchess.is_stalemate(), true);
    }

    mod is_threatened {
        use super::*;

        #[test]
        fn unattacked_position_is_not_threatened() {
            let hexchess = Hexchess::parse("1/2K/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(hexchess.is_threatened(h!("g10")), false);
        }

        #[test]
        fn threatened_by_enemy_piece() {
            let hexchess = Hexchess::parse("1/2K/5/7/9/11/11/11/11/11/6r4 w - 0 1").unwrap();

            assert_eq!(hexchess.is_threatened(h!("g10")), true);
        }
        
        #[test]
        fn not_threatened_by_friendly_piece() {
            let hexchess = Hexchess::parse("1/2K/5/7/9/11/11/11/11/11/6R4 w - 0 1").unwrap();

            assert_eq!(hexchess.is_threatened(h!("g10")), false);
        }

        #[test]
        fn  position_is_threatened_in_and_out_of_turn() {
            let mut hexchess = Hexchess::parse("1/3/5/7/4q4/5K5/11/11/11/11/11 w - 0 1").unwrap();

            hexchess.turn = Color::Black;
            assert_eq!(hexchess.is_threatened(h!("f6")), true);

            hexchess.turn = Color::White;
            assert_eq!(hexchess.is_threatened(h!("f6")), true);
        }

        #[test]
        fn unoccupied_position_is_not_threatened() {
            let hexchess = Hexchess::new();

            assert_eq!(hexchess.is_threatened(h!("f5")), false);
        }
    }

    mod moves_from {
        use super::*;

        #[test]
        fn returns_empty_vector_for_empty_position() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.moves_from(h!("a4")).len(), 0);
            assert_eq!(hexchess.moves_from_unsafe(h!("a4")).len(), 0);
        }
    }

    mod parsing {
        use crate::h;
        use super::*;

        #[test]
        fn empty_state() {
            let hexchess = Hexchess::new();
            
            assert!(hexchess.board.iter().all(|&square| square.is_none()));
            assert_eq!(hexchess.ep, None);
            assert_eq!(hexchess.fullmove, 1);
            assert_eq!(hexchess.halfmove, 0);
            assert_eq!(hexchess.turn, Color::White);
        }

        #[test]
        fn initial_state() {
            let hexchess = Hexchess::init();

            assert!(hexchess.board.iter().eq([
                Some(Piece::BlackBishop),
                Some(Piece::BlackQueen),
                Some(Piece::BlackBishop),
                Some(Piece::BlackKing),
                Some(Piece::BlackKnight),
                None,
                Some(Piece::BlackBishop),
                None,
                Some(Piece::BlackKnight),
                Some(Piece::BlackRook),
                None,
                None,
                None,
                None,
                None,
                Some(Piece::BlackRook),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                Some(Piece::WhiteBishop),
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                Some(Piece::WhiteBishop),
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                Some(Piece::WhiteRook),
                Some(Piece::WhiteKnight),
                Some(Piece::WhiteQueen),
                Some(Piece::WhiteBishop),
                Some(Piece::WhiteKing),
                Some(Piece::WhiteKnight),
                Some(Piece::WhiteRook),
                Some(Piece::WhitePawn),
                None,
            ].iter()));
        
            assert_eq!(hexchess.turn, Color::White);
            assert_eq!(hexchess.ep, None);
            assert_eq!(hexchess.halfmove, 0);
            assert_eq!(hexchess.fullmove, 1);
        }

        #[test]
        fn empty_string() {
            let hexchess = Hexchess::parse("");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "board not found"
            );
        }

        #[test]
        fn invalid() {
            let hexchess = Hexchess::parse("whoops");

            assert!(hexchess.is_err());
        }

        #[test]
        fn turn_color() {
            let white = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(white.turn, Color::White);

            let black = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 b - 0 1").unwrap();

            assert_eq!(black.turn, Color::Black);
        }

        #[test]
        fn invalid_turn_color() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 x - 0 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid turn color: x"
            );
        }

        #[test]
        fn missing_turn_color() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11").unwrap();

            assert_eq!(hexchess.turn, Color::White);
        }

        #[test]
        fn en_passant_black() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w e6 0 1");

            assert_eq!(hexchess.unwrap().ep, Some(h!("e6")));
        }

        #[test]
        fn en_passant_white() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w g5 0 1");

            assert_eq!(hexchess.unwrap().ep, Some(h!("g5")));
        }
    
        #[test]
        fn invalid_en_passant() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w x 0 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid en passant position: x"
            );
        }

        #[test]
        fn missing_en_passant() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w").unwrap();

            assert_eq!(hexchess.ep, None);
        }

        #[test]
        fn illegal_en_passant() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w a1 0 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "illegal en passant position: a1"
            );
        }

        #[test]
        fn invalid_halfmove() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - x 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid halfmove: x"
            );
        }

        #[test]
        fn multiple_black_kings() {
            let hexchess = Hexchess::parse("1/k1k/5/7/9/11/11/11/11/11/11 w - 0 1");

            assert!(hexchess.is_err());
            assert_eq!(hexchess.unwrap_err(), "multiple black kings");
        }

        #[test]
        fn multiple_white_kings() {
            let hexchess = Hexchess::parse("1/K1K/5/7/9/11/11/11/11/11/11 w - 0 1");

            assert!(hexchess.is_err());
            assert_eq!(hexchess.unwrap_err(), "multiple white kings");
        }

        #[test]
        fn invalid_character() {
            let hexchess = Hexchess::parse("x/3/5/7/9/11/11/11/11/11/11 w - 0 1");

            assert!(hexchess.is_err());
            assert_eq!(hexchess.unwrap_err(), "invalid character at index 0: x");
        }

        #[test]
        fn board_overflow() {
            let hexchess = Hexchess::parse("2/3/5/7/9/11/11/11/11/11/11 w - 0 1");

            assert!(hexchess.is_err());
            assert_eq!(hexchess.unwrap_err(), "board overflow");
        }
    
        #[test]
        fn missing_halfmove() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w -").unwrap();

            assert_eq!(hexchess.halfmove, 0);
        }

        #[test]
        fn invalid_fullmove() {
            let invalid1 = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 x");
            assert!(invalid1.is_err());
            assert_eq!(invalid1.unwrap_err(), "invalid fullmove: x");

            let invalid2 = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 0");
            assert!(invalid2.is_err());
            assert_eq!(invalid2.unwrap_err(), "invalid fullmove: 0");
        }

        #[test]
        fn missing_fullmove() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0").unwrap();

            assert_eq!(hexchess.fullmove, 1);
        }

        #[test]
        fn fen_with_skip_1() {
            let hexchess = Hexchess::parse("1/3/5/7/9/1p9/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("b6")]);
        }

        #[test]
        fn fen_with_skip_2() {
            let hexchess = Hexchess::parse("1/3/5/7/9/2p8/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("c6")]);
        }

        #[test]
        fn fen_with_skip_3() {
        let hexchess = Hexchess::parse("1/3/5/7/9/3p7/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("d6")]);
        }

        #[test]
        fn fen_with_skip_4() {
        let hexchess = Hexchess::parse("1/3/5/7/9/4p6/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("e6")]);
        }

        #[test]
        fn fen_with_skip_5() {
        let hexchess = Hexchess::parse("1/3/5/7/9/5p5/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("f6")]);
        }

        #[test]
        fn fen_with_skip_6() {
        let hexchess = Hexchess::parse("1/3/5/7/9/6p4/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("g6")]);
        }

        #[test]
        fn fen_with_skip_7() {
        let hexchess = Hexchess::parse("1/3/5/7/9/7p3/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("h6")]);
        }

        #[test]
        fn fen_with_skip_8() {
        let hexchess = Hexchess::parse("1/3/5/7/9/8p2/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("i6")]);
        }

        #[test]
        fn fen_with_skip_9() {
        let hexchess = Hexchess::parse("1/3/5/7/9/9p1/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("k6")]);
        }

        #[test]
        fn fen_with_skip_10() {
            let hexchess = Hexchess::parse("1/3/5/7/9/p10/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("a6")]);
        }

        #[test]
        fn fen_with_skip_11() {
            let hexchess = Hexchess::parse("1/3/5/7/9/11/p10/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("a5")]);
        }

    }

    mod self_check {
        use super::*;

        #[test]
        fn cannot_step_out_of_a_pin() {
            let hexchess = Hexchess::parse("1/3/5/7/4K4/5R5/5q5/11/11/11/11 w - 0 1").unwrap();

            let moves = hexchess.moves_from(h!("f6"));
            assert_eq!(moves.len(), 1);
            assert_eq!(moves[0], s!("f6f5"));
        }

        // cannot self check on opponent's turn
        #[test]
        fn cannot_self_check_on_opponents_turn() {
            let hexchess    = Hexchess::parse("1/3/5/7/4K4/5R5/5q5/11/11/11/11 b - 0 1").unwrap();
            let moves = hexchess.moves_from(h!("f6"));

            assert_eq!(moves.len(), 1);
            assert_eq!(moves[0], s!("f6f5"));
        }

        // king cannot step into check
        #[test]
        fn king_cannot_step_into_check() {
            let hexchess = Hexchess::parse("K/3/2q2/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
            let moves = hexchess.moves_from(h!("f11"));

            assert_eq!(moves.len(), 0);
        }
    }

    #[test]
    fn test_to_piece() {
        assert_eq!(to_piece('b'), Ok(Piece::BlackBishop));
        assert_eq!(to_piece('B'), Ok(Piece::WhiteBishop));
        assert_eq!(to_piece('k'), Ok(Piece::BlackKing)); // <- not called during normal board parsing
        assert_eq!(to_piece('K'), Ok(Piece::WhiteKing)); // <- not called during normal board parsing
        assert_eq!(to_piece('n'), Ok(Piece::BlackKnight));
        assert_eq!(to_piece('N'), Ok(Piece::WhiteKnight));
        assert_eq!(to_piece('p'), Ok(Piece::BlackPawn));
        assert_eq!(to_piece('P'), Ok(Piece::WhitePawn));
        assert_eq!(to_piece('q'), Ok(Piece::BlackQueen));
        assert_eq!(to_piece('Q'), Ok(Piece::WhiteQueen));
        assert_eq!(to_piece('r'), Ok(Piece::BlackRook));
        assert_eq!(to_piece('R'), Ok(Piece::WhiteRook));
    }

    #[test]
    fn test_to_piece_invalid() {
        assert_eq!(to_piece('x'), Err("invalid_piece_character"));
        assert_eq!(to_piece('1'), Err("invalid_piece_character"));
        assert_eq!(to_piece('/'), Err("invalid_piece_character"));
        assert_eq!(to_piece(' '), Err("invalid_piece_character"));
    }

    mod to_string {
        use super::*;

        #[test]
        fn empty_position() {
            let hexchess = Hexchess::new();

            assert_eq!(hexchess.to_string(), "1/3/5/7/9/11/11/11/11/11/11 w - 0 1");
        }

        #[test]
        fn initial_position() {
            let hexchess = Hexchess::init();

            assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1");
        }

        #[test]
        fn with_en_passant() {
            let mut hexchess = Hexchess::init();

            let _ = hexchess.apply_move(&s!("g4g6"));

            assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/6P4/5P5/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 1");
        }
    }
}
