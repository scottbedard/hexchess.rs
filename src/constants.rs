/// Initial game position
pub const INITIAL_POSITION: &str = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1";

/// Piece color
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

/// Piece symbols
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

/// Promotion pieces
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PromotionPiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}

/// parsed standard algebraic notation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct San {
    pub from: u8,
    pub promotion: Option<PromotionPiece>,
    pub to: u8,
}