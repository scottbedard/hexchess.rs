use crate::constants;
use crate::game::failure::Failure::{InvalidColor, InvalidPiece};
use serde::{Deserialize, Serialize};

use super::failure::Failure;

/// Piece color
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Color {
    #[serde(rename(deserialize = "W", serialize = "W"))]
    White,

    #[serde(rename(deserialize = "B", serialize = "B"))]
    Black,
}

impl Color {
    pub fn from(value: &str) -> Result<Self, Failure> {
        match value {
            "W" | "w" => Ok(Color::White),
            "B" | "b" => Ok(Color::Black),
            _ => Err(InvalidColor),
        }
    }
}

/// Colored piece types
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Piece {
    #[serde(rename(deserialize = "P", serialize = "P"))]
    WhitePawn,

    #[serde(rename(deserialize = "N", serialize = "N"))]
    WhiteKnight,

    #[serde(rename(deserialize = "B", serialize = "B"))]
    WhiteBishop,

    #[serde(rename(deserialize = "R", serialize = "R"))]
    WhiteRook,

    #[serde(rename(deserialize = "Q", serialize = "Q"))]
    WhiteQueen,

    #[serde(rename(deserialize = "K", serialize = "K"))]
    WhiteKing,

    #[serde(rename(deserialize = "p", serialize = "p"))]
    BlackPawn,

    #[serde(rename(deserialize = "n", serialize = "n"))]
    BlackKnight,

    #[serde(rename(deserialize = "b", serialize = "b"))]
    BlackBishop,

    #[serde(rename(deserialize = "r", serialize = "r"))]
    BlackRook,

    #[serde(rename(deserialize = "q", serialize = "q"))]
    BlackQueen,

    #[serde(rename(deserialize = "k", serialize = "k"))]
    BlackKing,
}

impl Piece {
    pub fn color(&self) -> Color {
        match self {
            Piece::WhitePawn
            | Piece::WhiteKnight
            | Piece::WhiteBishop
            | Piece::WhiteRook
            | Piece::WhiteQueen
            | Piece::WhiteKing => Color::White,
            Piece::BlackPawn
            | Piece::BlackKnight
            | Piece::BlackBishop
            | Piece::BlackRook
            | Piece::BlackQueen
            | Piece::BlackKing => Color::Black,
        }
    }

    pub fn from(value: &str) -> Result<Self, Failure> {
        match value {
            "P" => Ok(Piece::WhitePawn),
            "N" => Ok(Piece::WhiteKnight),
            "B" => Ok(Piece::WhiteBishop),
            "R" => Ok(Piece::WhiteRook),
            "Q" => Ok(Piece::WhiteQueen),
            "K" => Ok(Piece::WhiteKing),
            "p" => Ok(Piece::BlackPawn),
            "n" => Ok(Piece::BlackKnight),
            "b" => Ok(Piece::BlackBishop),
            "r" => Ok(Piece::BlackRook),
            "q" => Ok(Piece::BlackQueen),
            "k" => Ok(Piece::BlackKing),
            _ => Err(InvalidPiece),
        }
    }

    pub fn from_char(value: char) -> Result<Self, Failure> {
        match value {
            'P' => Ok(Piece::WhitePawn),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'R' => Ok(Piece::WhiteRook),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            'p' => Ok(Piece::BlackPawn),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'r' => Ok(Piece::BlackRook),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            _ => Err(InvalidPiece),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_color_from_value() {
        assert_eq!(Ok(Color::White), Color::from("W"));
        assert_eq!(Ok(Color::Black), Color::from("B"));
        assert_eq!(Ok(Color::White), Color::from("w"));
        assert_eq!(Ok(Color::Black), Color::from("b"));
        assert_eq!(Err(InvalidColor), Color::from("whoops"));
    }

    #[test]
    fn test_create_piece_from_str_value() {
        assert_eq!(Ok(Piece::WhitePawn), Piece::from("P"));
        assert_eq!(Ok(Piece::WhiteKnight), Piece::from("N"));
        assert_eq!(Ok(Piece::WhiteBishop), Piece::from("B"));
        assert_eq!(Ok(Piece::WhiteRook), Piece::from("R"));
        assert_eq!(Ok(Piece::WhiteQueen), Piece::from("Q"));
        assert_eq!(Ok(Piece::WhiteKing), Piece::from("K"));
        assert_eq!(Ok(Piece::BlackPawn), Piece::from("p"));
        assert_eq!(Ok(Piece::BlackKnight), Piece::from("n"));
        assert_eq!(Ok(Piece::BlackBishop), Piece::from("b"));
        assert_eq!(Ok(Piece::BlackRook), Piece::from("r"));
        assert_eq!(Ok(Piece::BlackQueen), Piece::from("q"));
        assert_eq!(Ok(Piece::BlackKing), Piece::from("k"));
        assert_eq!(Err(InvalidPiece), Piece::from("whoops"));
    }

    #[test]
    fn test_create_piece_from_char_value() {
        assert_eq!(Ok(Piece::WhitePawn), Piece::from_char('P'));
        assert_eq!(Ok(Piece::WhiteKnight), Piece::from_char('N'));
        assert_eq!(Ok(Piece::WhiteBishop), Piece::from_char('B'));
        assert_eq!(Ok(Piece::WhiteRook), Piece::from_char('R'));
        assert_eq!(Ok(Piece::WhiteQueen), Piece::from_char('Q'));
        assert_eq!(Ok(Piece::WhiteKing), Piece::from_char('K'));
        assert_eq!(Ok(Piece::BlackPawn), Piece::from_char('p'));
        assert_eq!(Ok(Piece::BlackKnight), Piece::from_char('n'));
        assert_eq!(Ok(Piece::BlackBishop), Piece::from_char('b'));
        assert_eq!(Ok(Piece::BlackRook), Piece::from_char('r'));
        assert_eq!(Ok(Piece::BlackQueen), Piece::from_char('q'));
        assert_eq!(Ok(Piece::BlackKing), Piece::from_char('k'));
        assert_eq!(Err(InvalidPiece), Piece::from_char('x'));
    }
}
