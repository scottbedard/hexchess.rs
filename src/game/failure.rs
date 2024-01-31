use std::fmt::{Display, Formatter, Result};

/// Known hexchess failure
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Failure {
    IllegalMove,
    InvalidBoard,
    InvalidColor,
    InvalidFullmove,
    InvalidHalfmove,
    InvalidNotation,
    InvalidPiece,
    InvalidPosition,
    InvalidPromotion,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Failure::IllegalMove => write!(f, "illegal_move"),
            Failure::InvalidBoard => write!(f, "invalid_board"),
            Failure::InvalidColor => write!(f, "invalid_color"),
            Failure::InvalidFullmove => write!(f, "invalid_fullmove"),
            Failure::InvalidHalfmove => write!(f, "invalid_halfmove"),
            Failure::InvalidNotation => write!(f, "invalid_notation"),
            Failure::InvalidPiece => write!(f, "invalid_piece"),
            Failure::InvalidPosition => write!(f, "invalid_position"),
            Failure::InvalidPromotion => write!(f, "invalid_promotion"),
        }
    }
}