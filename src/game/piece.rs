use crate::constants;
use crate::game::failure::Failure::{InvalidColor, InvalidPiece, InvalidPromotion};
use serde::{Deserialize, Serialize};
use std::fmt;

use super::failure::Failure;

/// Piece color
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Color {
    #[serde(rename(deserialize = "w", serialize = "w"))]
    White,

    #[serde(rename(deserialize = "b", serialize = "b"))]
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::WhitePawn => write!(f, "P"),
            Piece::WhiteKnight => write!(f, "N"),
            Piece::WhiteBishop => write!(f, "B"),
            Piece::WhiteRook => write!(f, "R"),
            Piece::WhiteQueen => write!(f, "Q"),
            Piece::WhiteKing => write!(f, "K"),
            Piece::BlackPawn => write!(f, "p"),
            Piece::BlackKnight => write!(f, "n"),
            Piece::BlackBishop => write!(f, "b"),
            Piece::BlackRook => write!(f, "r"),
            Piece::BlackQueen => write!(f, "q"),
            Piece::BlackKing => write!(f, "k"),
        }
    }
}

/// Generic piece type
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum PromotionPiece {
    #[serde(rename(deserialize = "n", serialize = "n"))]
    Knight,

    #[serde(rename(deserialize = "b", serialize = "b"))]
    Bishop,

    #[serde(rename(deserialize = "r", serialize = "r"))]
    Rook,

    #[serde(rename(deserialize = "q", serialize = "q"))]
    Queen,
}

impl PromotionPiece {
    pub fn from(value: &str) -> Result<Self, Failure> {
        match value {
            "n" => Ok(PromotionPiece::Knight),
            "b" => Ok(PromotionPiece::Bishop),
            "r" => Ok(PromotionPiece::Rook),
            "q" => Ok(PromotionPiece::Queen),
            _ => Err(InvalidPromotion),
        }
    }
}

impl fmt::Display for PromotionPiece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PromotionPiece::Knight => write!(f, "n"),
            PromotionPiece::Bishop => write!(f, "b"),
            PromotionPiece::Rook => write!(f, "r"),
            PromotionPiece::Queen => write!(f, "q"),
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

    #[test]
    fn test_convert_piece_to_string() {
        assert_eq!("P", Piece::WhitePawn.to_string());
        assert_eq!("N", Piece::WhiteKnight.to_string());
        assert_eq!("B", Piece::WhiteBishop.to_string());
        assert_eq!("R", Piece::WhiteRook.to_string());
        assert_eq!("Q", Piece::WhiteQueen.to_string());
        assert_eq!("K", Piece::WhiteKing.to_string());
        assert_eq!("p", Piece::BlackPawn.to_string());
        assert_eq!("n", Piece::BlackKnight.to_string());
        assert_eq!("b", Piece::BlackBishop.to_string());
        assert_eq!("r", Piece::BlackRook.to_string());
        assert_eq!("q", Piece::BlackQueen.to_string());
        assert_eq!("k", Piece::BlackKing.to_string());
    }

    #[test]
    fn test_get_promotion_piece() {
        assert_eq!(Ok(PromotionPiece::Knight), PromotionPiece::from("n"));
        assert_eq!(Ok(PromotionPiece::Bishop), PromotionPiece::from("b"));
        assert_eq!(Ok(PromotionPiece::Rook), PromotionPiece::from("r"));
        assert_eq!(Ok(PromotionPiece::Queen), PromotionPiece::from("q"));
        assert_eq!(Err(InvalidPromotion), PromotionPiece::from("whoops"));
    }
}
