use crate::constants::{Color, Piece, Position};

#[derive(Clone, Copy)]
pub struct Hexchess {
    pub board: [Option<Piece>; 91],
    pub ep: Option<Position>,
    pub fullmove: u16,
    pub halfmove: u8,
    pub turn: Color,
}
