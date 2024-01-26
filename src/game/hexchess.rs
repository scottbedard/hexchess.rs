use crate::game::board::{Board, Position};
use crate::game::failure::Failure;
use crate::game::piece::{Color, Piece};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// Fen for a new game
// pub const NEW_GAME_FEN: &str = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1";
pub const NEW_GAME_FEN: &str = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1";

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
            Some(value) => match Board::from(value) {
                Ok(board) => board,
                Err(failure) => return Err(failure)
            },
            None => Board::new(),
        };

        match parts.next() {
            Some(value) => println!("color: {:?}", value),
            None => (),
        }

        match parts.next() {
            Some(value) => println!("en passant: {:?}", value),
            None => (),
        }

        Ok(Hexchess {
            board,
            en_passant: None,
            fullmove: 0,
            halfmove: 0,
            turn: Color::White,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hexchess_from_fen() {
        let hexchess = match Hexchess::from(NEW_GAME_FEN) {
            Ok(value) => value,
            Err(failure) => panic!("failure: {:?}", failure),
        };
        
        assert_eq!(Some(Piece::BlackQueen), hexchess.board.e1);
    }
}
