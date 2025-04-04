use crate::h;
use crate::hexchess::pieces::king::king_moves_unsafe;
use crate::hexchess::pieces::knight::knight_moves_unsafe;
use crate::hexchess::pieces::pawn::pawn_moves_unsafe;
use crate::hexchess::pieces::straight_line::straight_line_moves_unsafe;
use crate::hexchess::san::San;

use crate::constants::{
    Color,
    INITIAL_POSITION,
    Piece,
    PromotionPiece,
};

use crate::hexchess::utils::{
    get_color,
    is_legal_en_passant,
    to_index,
};

#[derive(Clone, Copy, Debug)]
pub struct Hexchess {
    pub board: [Option<Piece>; 91],

    pub ep: Option<u8>,

    pub fullmove: u16,

    pub halfmove: u8,

    pub turn: Color,
}

impl Hexchess {
    /// apply move
    pub fn apply_move(&mut self, san: &San) {
        self.apply_move_unsafe(san);
    }

    /// apply move, regardless of legality
    pub fn apply_move_unsafe(&mut self, san: &San) {
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
    }

    /// get moves a position
    pub fn moves_from(&self, from: u8) -> Vec<San> {
        self.moves_from_unsafe(from)
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

    /// create hexchess instance from fen
    pub fn from(source: &str) -> Result<Self, String> {
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
                _ => match to_index(&part) {
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

    /// initialize a hexchess instance to the starting position
    pub fn init() -> Self {
        Self::from(INITIAL_POSITION).unwrap()
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
            'b' |
            'B' |
            'n' |
            'N' |
            'p' |
            'P' |
            'Q' |
            'q' |
            'r' |
            'R' => {
                arr[fen_index as usize] = match to_piece(current) {
                    Ok(piece) => Some(piece),
                    Err(_) => return Err(format!("invalid piece at index {}: {}", index, current)),
                };
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
    use super::*;

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
            let hexchess = Hexchess::from("");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "board not found"
            );
        }

        #[test]
        fn turn_color() {
            let white = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(white.turn, Color::White);

            let black = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 b - 0 1").unwrap();

            assert_eq!(black.turn, Color::Black);
        }

        #[test]
        fn invalid_turn_color() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 x - 0 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid turn color: x"
            );
        }

        #[test]
        fn missing_turn_color() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11").unwrap();

            assert_eq!(hexchess.turn, Color::White);
        }

        #[test]
        fn en_passant() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w f6 0 1");

            assert_eq!(hexchess.unwrap().ep, Some(h!("f6")));
        }

        #[test]
        fn invalid_en_passant() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w x 0 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid en passant position: x"
            );
        }

        #[test]
        fn missing_en_passant() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w").unwrap();

            assert_eq!(hexchess.ep, None);
        }

        #[test]
        fn illegal_en_passant() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w a1 0 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "illegal en passant position: a1"
            );
        }

        #[test]
        fn invalid_halfmove() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - x 1");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid halfmove: x"
            );
        }

        #[test]
        fn missing_halfmove() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w -").unwrap();

            assert_eq!(hexchess.halfmove, 0);
        }

        #[test]
        fn invalid_fullmove() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 x");

            assert!(hexchess.is_err());
            assert_eq!(
                hexchess.unwrap_err(),
                "invalid fullmove: x"
            );
        }

        #[test]
        fn missing_fullmove() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0").unwrap();

            assert_eq!(hexchess.fullmove, 1);
        }

        #[test]
        fn fen_with_skip_1() {
            let hexchess = Hexchess::from("1/3/5/7/9/1p9/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("b6")]);
        }

        #[test]
        fn fen_with_skip_2() {
            let hexchess = Hexchess::from("1/3/5/7/9/2p8/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("c6")]);
        }

        #[test]
        fn fen_with_skip_3() {
        let hexchess = Hexchess::from("1/3/5/7/9/3p7/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("d6")]);
        }

        #[test]
        fn fen_with_skip_4() {
        let hexchess = Hexchess::from("1/3/5/7/9/4p6/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("e6")]);
        }

        #[test]
        fn fen_with_skip_5() {
        let hexchess = Hexchess::from("1/3/5/7/9/5p5/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("f6")]);
        }

        #[test]
        fn fen_with_skip_6() {
        let hexchess = Hexchess::from("1/3/5/7/9/6p4/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("g6")]);
        }

        #[test]
        fn fen_with_skip_7() {
        let hexchess = Hexchess::from("1/3/5/7/9/7p3/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("h6")]);
        }

        #[test]
        fn fen_with_skip_8() {
        let hexchess = Hexchess::from("1/3/5/7/9/8p2/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("i6")]);
        }

        #[test]
        fn fen_with_skip_9() {
        let hexchess = Hexchess::from("1/3/5/7/9/9p1/11/11/11/11/11 w - 0 1").unwrap();

        assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("k6")]);
        }

        #[test]
        fn fen_with_skip_10() {
            let hexchess = Hexchess::from("1/3/5/7/9/p10/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("a6")]);
        }

        #[test]
        fn fen_with_skip_11() {
            let hexchess = Hexchess::from("1/3/5/7/9/11/p10/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("a5")]);
        }

    }

    mod apply_move {
        use crate::{h, s};
        use super::*;

        #[test]
        fn sets_to_and_from_positions() {
            let mut hexchess = Hexchess::init();

            hexchess.apply_move(&s!("g4g5"));
            hexchess.apply_move(&s!("e7e6"));

            assert_eq!(hexchess.board[h!("g5")], Some(Piece::WhitePawn));
            assert_eq!(hexchess.board[h!("g4")], None);
            assert_eq!(hexchess.board[h!("e6")], Some(Piece::BlackPawn));
            assert_eq!(hexchess.board[h!("e7")], None);
        }

        #[test]
        fn sets_and_unsets_en_passant() {
            let mut hexchess = Hexchess::init();

            hexchess.apply_move(&s!("g4g6"));
            assert_eq!(hexchess.ep, Some(h!("g5")));

            hexchess.apply_move(&s!("e7e5"));
            assert_eq!(hexchess.ep, Some(h!("e6")));

            hexchess.apply_move(&s!("b1b2"));
            assert_eq!(hexchess.ep, None);
        }

        // cannot step out of pin

        // cannot self check on opponent's turn

        // king cannot step into check

        // promote white pieces

        // promote black pieces

        // white cannot promote on black's promotion positions

        // black cannot promote on white's promotion positions

        // out of turn error
    }
}
