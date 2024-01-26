use std::fmt::{Display, Formatter, Result};

/// Known hexchess failure
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Failure {
    InvalidBoard,
    InvalidColor,
    InvalidFullmove,
    InvalidHalfmove,
    InvalidPiece,
    InvalidPosition,
}

impl Display for Failure {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Failure::InvalidBoard => write!(f, "invalid_board"),
            Failure::InvalidColor => write!(f, "invalid_color"),
            Failure::InvalidFullmove => write!(f, "invalid_fullmove"),
            Failure::InvalidHalfmove => write!(f, "invalid_halfmove"),
            Failure::InvalidPiece => write!(f, "invalid_piece"),
            Failure::InvalidPosition => write!(f, "invalid_position"),
        }
    }
}