use crate::game::board::{Board, Position};
use crate::game::constants::{INITIAL_BOARD, INITIAL_HEXCHESS};
use crate::game::failure::Failure;
use crate::game::piece::{Color, Piece};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// Hexchess game state
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Hexchess {
    board: Board,

    en_passant: Option<Position>,

    fullmove: u16,

    halfmove: u8,

    turn: Color,
}

/// Create hexchess from fen
impl Hexchess {
    pub fn from(value: &str) -> Result<Hexchess, Failure> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hexchess_from_initial_board_fen() {
        let hexchess = match Hexchess::from(INITIAL_BOARD) {
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
    fn test_invalid_turn_color_results_in_error()
    {
        let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 ? - 0 1");

        assert_eq!(Err(Failure::InvalidColor), hexchess);
    }

    #[test]
    fn test_invalid_en_passant_results_in_error()
    {
        let hexchess = Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w ? 0 1");

        assert_eq!(Err(Failure::InvalidPosition), hexchess);
    }

    #[test]
    fn test_invalid_halfmove_results_in_error()
    {
        assert_eq!(Err(Failure::InvalidHalfmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - ? 1"));
        assert_eq!(Err(Failure::InvalidHalfmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0.5 1"));
        assert_eq!(Err(Failure::InvalidHalfmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - -6 1"));
    }

    #[test]
    fn test_invalid_fullmove_results_in_error()
    {
        assert_eq!(Err(Failure::InvalidFullmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 ?"));
        assert_eq!(Err(Failure::InvalidFullmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 1.5"));
        assert_eq!(Err(Failure::InvalidFullmove), Hexchess::from("1/3/5/7/9/11/11/11/11/11/11 w - 0 0")); // <- less than 1
    }
}
