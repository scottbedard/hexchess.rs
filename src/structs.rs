use crate::constants::{
    Color,
    Piece,
    PromotionPiece,
};

#[derive(Clone, Copy)]
pub struct Hexchess {
    pub board: [Option<Piece>; 91],
    pub ep: Option<u8>,
    pub fullmove: u16,
    pub halfmove: u8,
    pub turn: Color,
}

#[derive(Clone, Copy)]
pub struct San {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<PromotionPiece>,
}