use crate::game::constants::{INITIAL_BOARD, SORTED_POSITIONS};
use crate::game::failure::Failure;
use crate::game::piece::{Color, Piece};
use serde::{Deserialize, Serialize};
use tsify::Tsify;

/// Unique board position
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Position {
    #[serde(rename(deserialize = "a1", serialize = "a1"))]
    A1,

    #[serde(rename(deserialize = "a2", serialize = "a2"))]
    A2,

    #[serde(rename(deserialize = "a3", serialize = "a3"))]
    A3,

    #[serde(rename(deserialize = "a4", serialize = "a4"))]
    A4,

    #[serde(rename(deserialize = "a5", serialize = "a5"))]
    A5,

    #[serde(rename(deserialize = "a6", serialize = "a6"))]
    A6,

    #[serde(rename(deserialize = "b1", serialize = "b1"))]
    B1,

    #[serde(rename(deserialize = "b2", serialize = "b2"))]
    B2,

    #[serde(rename(deserialize = "b3", serialize = "b3"))]
    B3,

    #[serde(rename(deserialize = "b4", serialize = "b4"))]
    B4,

    #[serde(rename(deserialize = "b5", serialize = "b5"))]
    B5,

    #[serde(rename(deserialize = "b6", serialize = "b6"))]
    B6,

    #[serde(rename(deserialize = "b7", serialize = "b7"))]
    B7,

    #[serde(rename(deserialize = "c1", serialize = "c1"))]
    C1,

    #[serde(rename(deserialize = "c2", serialize = "c2"))]
    C2,

    #[serde(rename(deserialize = "c3", serialize = "c3"))]
    C3,

    #[serde(rename(deserialize = "c4", serialize = "c4"))]
    C4,

    #[serde(rename(deserialize = "c5", serialize = "c5"))]
    C5,

    #[serde(rename(deserialize = "c6", serialize = "c6"))]
    C6,

    #[serde(rename(deserialize = "c7", serialize = "c7"))]
    C7,

    #[serde(rename(deserialize = "c8", serialize = "c8"))]
    C8,

    #[serde(rename(deserialize = "d1", serialize = "d1"))]
    D1,

    #[serde(rename(deserialize = "d2", serialize = "d2"))]
    D2,

    #[serde(rename(deserialize = "d3", serialize = "d3"))]
    D3,

    #[serde(rename(deserialize = "d4", serialize = "d4"))]
    D4,

    #[serde(rename(deserialize = "d5", serialize = "d5"))]
    D5,

    #[serde(rename(deserialize = "d6", serialize = "d6"))]
    D6,

    #[serde(rename(deserialize = "d7", serialize = "d7"))]
    D7,

    #[serde(rename(deserialize = "d8", serialize = "d8"))]
    D8,

    #[serde(rename(deserialize = "d9", serialize = "d9"))]
    D9,

    #[serde(rename(deserialize = "e1", serialize = "e1"))]
    E1,

    #[serde(rename(deserialize = "e2", serialize = "e2"))]
    E2,

    #[serde(rename(deserialize = "e3", serialize = "e3"))]
    E3,

    #[serde(rename(deserialize = "e4", serialize = "e4"))]
    E4,

    #[serde(rename(deserialize = "e5", serialize = "e5"))]
    E5,

    #[serde(rename(deserialize = "e6", serialize = "e6"))]
    E6,

    #[serde(rename(deserialize = "e7", serialize = "e7"))]
    E7,

    #[serde(rename(deserialize = "e8", serialize = "e8"))]
    E8,

    #[serde(rename(deserialize = "e9", serialize = "e9"))]
    E9,

    #[serde(rename(deserialize = "e10", serialize = "e10"))]
    E10,

    #[serde(rename(deserialize = "f1", serialize = "f1"))]
    F1,

    #[serde(rename(deserialize = "f2", serialize = "f2"))]
    F2,

    #[serde(rename(deserialize = "f3", serialize = "f3"))]
    F3,

    #[serde(rename(deserialize = "f4", serialize = "f4"))]
    F4,

    #[serde(rename(deserialize = "f5", serialize = "f5"))]
    F5,

    #[serde(rename(deserialize = "f6", serialize = "f6"))]
    F6,

    #[serde(rename(deserialize = "f7", serialize = "f7"))]
    F7,

    #[serde(rename(deserialize = "f8", serialize = "f8"))]
    F8,

    #[serde(rename(deserialize = "f9", serialize = "f9"))]
    F9,

    #[serde(rename(deserialize = "f10", serialize = "f10"))]
    F10,

    #[serde(rename(deserialize = "f11", serialize = "f11"))]
    F11,

    #[serde(rename(deserialize = "g1", serialize = "g1"))]
    G1,

    #[serde(rename(deserialize = "g2", serialize = "g2"))]
    G2,

    #[serde(rename(deserialize = "g3", serialize = "g3"))]
    G3,

    #[serde(rename(deserialize = "g4", serialize = "g4"))]
    G4,

    #[serde(rename(deserialize = "g5", serialize = "g5"))]
    G5,

    #[serde(rename(deserialize = "g6", serialize = "g6"))]
    G6,

    #[serde(rename(deserialize = "g7", serialize = "g7"))]
    G7,

    #[serde(rename(deserialize = "g8", serialize = "g8"))]
    G8,

    #[serde(rename(deserialize = "g9", serialize = "g9"))]
    G9,

    #[serde(rename(deserialize = "g10", serialize = "g10"))]
    G10,

    #[serde(rename(deserialize = "h1", serialize = "h1"))]
    H1,

    #[serde(rename(deserialize = "h2", serialize = "h2"))]
    H2,

    #[serde(rename(deserialize = "h3", serialize = "h3"))]
    H3,

    #[serde(rename(deserialize = "h4", serialize = "h4"))]
    H4,

    #[serde(rename(deserialize = "h5", serialize = "h5"))]
    H5,

    #[serde(rename(deserialize = "h6", serialize = "h6"))]
    H6,

    #[serde(rename(deserialize = "h7", serialize = "h7"))]
    H7,

    #[serde(rename(deserialize = "h8", serialize = "h8"))]
    H8,

    #[serde(rename(deserialize = "h9", serialize = "h9"))]
    H9,

    #[serde(rename(deserialize = "i1", serialize = "i1"))]
    I1,

    #[serde(rename(deserialize = "i2", serialize = "i2"))]
    I2,

    #[serde(rename(deserialize = "i3", serialize = "i3"))]
    I3,

    #[serde(rename(deserialize = "i4", serialize = "i4"))]
    I4,

    #[serde(rename(deserialize = "i5", serialize = "i5"))]
    I5,

    #[serde(rename(deserialize = "i6", serialize = "i6"))]
    I6,

    #[serde(rename(deserialize = "i7", serialize = "i7"))]
    I7,

    #[serde(rename(deserialize = "i8", serialize = "i8"))]
    I8,

    #[serde(rename(deserialize = "k1", serialize = "k1"))]
    K1,

    #[serde(rename(deserialize = "k2", serialize = "k2"))]
    K2,

    #[serde(rename(deserialize = "k3", serialize = "k3"))]
    K3,

    #[serde(rename(deserialize = "k4", serialize = "k4"))]
    K4,

    #[serde(rename(deserialize = "k5", serialize = "k5"))]
    K5,

    #[serde(rename(deserialize = "k6", serialize = "k6"))]
    K6,

    #[serde(rename(deserialize = "k7", serialize = "k7"))]
    K7,

    #[serde(rename(deserialize = "l1", serialize = "l1"))]
    L1,

    #[serde(rename(deserialize = "l2", serialize = "l2"))]
    L2,

    #[serde(rename(deserialize = "l3", serialize = "l3"))]
    L3,

    #[serde(rename(deserialize = "l4", serialize = "l4"))]
    L4,

    #[serde(rename(deserialize = "l5", serialize = "l5"))]
    L5,

    #[serde(rename(deserialize = "l6", serialize = "l6"))]
    L6,
}

impl Position {
    pub fn from(value: &str) -> Result<Self, Failure> {
        match value {
            "a1" => Ok(Position::A1),
            "a2" => Ok(Position::A2),
            "a3" => Ok(Position::A3),
            "a4" => Ok(Position::A4),
            "a5" => Ok(Position::A5),
            "a6" => Ok(Position::A6),
            "b1" => Ok(Position::B1),
            "b2" => Ok(Position::B2),
            "b3" => Ok(Position::B3),
            "b4" => Ok(Position::B4),
            "b5" => Ok(Position::B5),
            "b6" => Ok(Position::B6),
            "b7" => Ok(Position::B7),
            "c1" => Ok(Position::C1),
            "c2" => Ok(Position::C2),
            "c3" => Ok(Position::C3),
            "c4" => Ok(Position::C4),
            "c5" => Ok(Position::C5),
            "c6" => Ok(Position::C6),
            "c7" => Ok(Position::C7),
            "c8" => Ok(Position::C8),
            "d1" => Ok(Position::D1),
            "d2" => Ok(Position::D2),
            "d3" => Ok(Position::D3),
            "d4" => Ok(Position::D4),
            "d5" => Ok(Position::D5),
            "d6" => Ok(Position::D6),
            "d7" => Ok(Position::D7),
            "d8" => Ok(Position::D8),
            "d9" => Ok(Position::D9),
            "e1" => Ok(Position::E1),
            "e2" => Ok(Position::E2),
            "e3" => Ok(Position::E3),
            "e4" => Ok(Position::E4),
            "e5" => Ok(Position::E5),
            "e6" => Ok(Position::E6),
            "e7" => Ok(Position::E7),
            "e8" => Ok(Position::E8),
            "e9" => Ok(Position::E9),
            "e10" => Ok(Position::E10),
            "f1" => Ok(Position::F1),
            "f2" => Ok(Position::F2),
            "f3" => Ok(Position::F3),
            "f4" => Ok(Position::F4),
            "f5" => Ok(Position::F5),
            "f6" => Ok(Position::F6),
            "f7" => Ok(Position::F7),
            "f8" => Ok(Position::F8),
            "f9" => Ok(Position::F9),
            "f10" => Ok(Position::F10),
            "f11" => Ok(Position::F11),
            "g1" => Ok(Position::G1),
            "g2" => Ok(Position::G2),
            "g3" => Ok(Position::G3),
            "g4" => Ok(Position::G4),
            "g5" => Ok(Position::G5),
            "g6" => Ok(Position::G6),
            "g7" => Ok(Position::G7),
            "g8" => Ok(Position::G8),
            "g9" => Ok(Position::G9),
            "g10" => Ok(Position::G10),
            "h1" => Ok(Position::H1),
            "h2" => Ok(Position::H2),
            "h3" => Ok(Position::H3),
            "h4" => Ok(Position::H4),
            "h5" => Ok(Position::H5),
            "h6" => Ok(Position::H6),
            "h7" => Ok(Position::H7),
            "h8" => Ok(Position::H8),
            "h9" => Ok(Position::H9),
            "i1" => Ok(Position::I1),
            "i2" => Ok(Position::I2),
            "i3" => Ok(Position::I3),
            "i4" => Ok(Position::I4),
            "i5" => Ok(Position::I5),
            "i6" => Ok(Position::I6),
            "i7" => Ok(Position::I7),
            "i8" => Ok(Position::I8),
            "k1" => Ok(Position::K1),
            "k2" => Ok(Position::K2),
            "k3" => Ok(Position::K3),
            "k4" => Ok(Position::K4),
            "k5" => Ok(Position::K5),
            "k6" => Ok(Position::K6),
            "k7" => Ok(Position::K7),
            "l1" => Ok(Position::L1),
            "l2" => Ok(Position::L2),
            "l3" => Ok(Position::L3),
            "l4" => Ok(Position::L4),
            "l5" => Ok(Position::L5),
            "l6" => Ok(Position::L6),
            _ => return Err(Failure::InvalidPosition),
        }
    }
}

/// Pieces at each position of a hexboard
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Board {
    pub a1: Option<Piece>,
    pub a2: Option<Piece>,
    pub a3: Option<Piece>,
    pub a4: Option<Piece>,
    pub a5: Option<Piece>,
    pub a6: Option<Piece>,
    pub b1: Option<Piece>,
    pub b2: Option<Piece>,
    pub b3: Option<Piece>,
    pub b4: Option<Piece>,
    pub b5: Option<Piece>,
    pub b6: Option<Piece>,
    pub b7: Option<Piece>,
    pub c1: Option<Piece>,
    pub c2: Option<Piece>,
    pub c3: Option<Piece>,
    pub c4: Option<Piece>,
    pub c5: Option<Piece>,
    pub c6: Option<Piece>,
    pub c7: Option<Piece>,
    pub c8: Option<Piece>,
    pub d1: Option<Piece>,
    pub d2: Option<Piece>,
    pub d3: Option<Piece>,
    pub d4: Option<Piece>,
    pub d5: Option<Piece>,
    pub d6: Option<Piece>,
    pub d7: Option<Piece>,
    pub d8: Option<Piece>,
    pub d9: Option<Piece>,
    pub e1: Option<Piece>,
    pub e2: Option<Piece>,
    pub e3: Option<Piece>,
    pub e4: Option<Piece>,
    pub e5: Option<Piece>,
    pub e6: Option<Piece>,
    pub e7: Option<Piece>,
    pub e8: Option<Piece>,
    pub e9: Option<Piece>,
    pub e10: Option<Piece>,
    pub f1: Option<Piece>,
    pub f2: Option<Piece>,
    pub f3: Option<Piece>,
    pub f4: Option<Piece>,
    pub f5: Option<Piece>,
    pub f6: Option<Piece>,
    pub f7: Option<Piece>,
    pub f8: Option<Piece>,
    pub f9: Option<Piece>,
    pub f10: Option<Piece>,
    pub f11: Option<Piece>,
    pub g1: Option<Piece>,
    pub g2: Option<Piece>,
    pub g3: Option<Piece>,
    pub g4: Option<Piece>,
    pub g5: Option<Piece>,
    pub g6: Option<Piece>,
    pub g7: Option<Piece>,
    pub g8: Option<Piece>,
    pub g9: Option<Piece>,
    pub g10: Option<Piece>,
    pub h1: Option<Piece>,
    pub h2: Option<Piece>,
    pub h3: Option<Piece>,
    pub h4: Option<Piece>,
    pub h5: Option<Piece>,
    pub h6: Option<Piece>,
    pub h7: Option<Piece>,
    pub h8: Option<Piece>,
    pub h9: Option<Piece>,
    pub i1: Option<Piece>,
    pub i2: Option<Piece>,
    pub i3: Option<Piece>,
    pub i4: Option<Piece>,
    pub i5: Option<Piece>,
    pub i6: Option<Piece>,
    pub i7: Option<Piece>,
    pub i8: Option<Piece>,
    pub k1: Option<Piece>,
    pub k2: Option<Piece>,
    pub k3: Option<Piece>,
    pub k4: Option<Piece>,
    pub k5: Option<Piece>,
    pub k6: Option<Piece>,
    pub k7: Option<Piece>,
    pub l1: Option<Piece>,
    pub l2: Option<Piece>,
    pub l3: Option<Piece>,
    pub l4: Option<Piece>,
    pub l5: Option<Piece>,
    pub l6: Option<Piece>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            a1: None,
            a2: None,
            a3: None,
            a4: None,
            a5: None,
            a6: None,
            b1: None,
            b2: None,
            b3: None,
            b4: None,
            b5: None,
            b6: None,
            b7: None,
            c1: None,
            c2: None,
            c3: None,
            c4: None,
            c5: None,
            c6: None,
            c7: None,
            c8: None,
            d1: None,
            d2: None,
            d3: None,
            d4: None,
            d5: None,
            d6: None,
            d7: None,
            d8: None,
            d9: None,
            e1: None,
            e2: None,
            e3: None,
            e4: None,
            e5: None,
            e6: None,
            e7: None,
            e8: None,
            e9: None,
            e10: None,
            f1: None,
            f2: None,
            f3: None,
            f4: None,
            f5: None,
            f6: None,
            f7: None,
            f8: None,
            f9: None,
            f10: None,
            f11: None,
            g1: None,
            g2: None,
            g3: None,
            g4: None,
            g5: None,
            g6: None,
            g7: None,
            g8: None,
            g9: None,
            g10: None,
            h1: None,
            h2: None,
            h3: None,
            h4: None,
            h5: None,
            h6: None,
            h7: None,
            h8: None,
            h9: None,
            i1: None,
            i2: None,
            i3: None,
            i4: None,
            i5: None,
            i6: None,
            i7: None,
            i8: None,
            k1: None,
            k2: None,
            k3: None,
            k4: None,
            k5: None,
            k6: None,
            k7: None,
            l1: None,
            l2: None,
            l3: None,
            l4: None,
            l5: None,
            l6: None,
        }
    }

    pub fn from(value: &str) -> Result<Self, Failure> {
        let mut board = Self::new();

        let mut valid: bool = true;

        value
            .replace("11", "___________")
            .replace("10", "__________")
            .replace("9", "_________")
            .replace("8", "________")
            .replace("7", "_______")
            .replace("6", "______")
            .replace("5", "_____")
            .replace("4", "____")
            .replace("3", "___")
            .replace("2", "__")
            .replace("1", "_")
            .replace("/", "")
            .chars()
            .enumerate()
            .for_each(|(index, p)| {
                if !valid {
                    return
                }

                let piece = match Piece::from(p.to_string().as_str()) {
                    Ok(piece) => piece,
                    Err(_) => {
                        valid = false;
                        return
                    },
                };

                match SORTED_POSITIONS[index] {
                    Position::F11 => board.f11 = Some(piece),
                    Position::E10 => board.e10 = Some(piece),
                    Position::F10 => board.f10 = Some(piece),
                    Position::G10 => board.g10 = Some(piece),
                    Position::D9 => board.d9 = Some(piece),
                    Position::E9 => board.e9 = Some(piece),
                    Position::F9 => board.f9 = Some(piece),
                    Position::G9 => board.g9 = Some(piece),
                    Position::H9 => board.h9 = Some(piece),
                    Position::C8 => board.c8 = Some(piece),
                    Position::D8 => board.d8 = Some(piece),
                    Position::E8 => board.e8 = Some(piece),
                    Position::F8 => board.f8 = Some(piece),
                    Position::G8 => board.g8 = Some(piece),
                    Position::H8 => board.h8 = Some(piece),
                    Position::I8 => board.i8 = Some(piece),
                    Position::B7 => board.b7 = Some(piece),
                    Position::C7 => board.c7 = Some(piece),
                    Position::D7 => board.d7 = Some(piece),
                    Position::E7 => board.e7 = Some(piece),
                    Position::F7 => board.f7 = Some(piece),
                    Position::G7 => board.g7 = Some(piece),
                    Position::H7 => board.h7 = Some(piece),
                    Position::I7 => board.i7 = Some(piece),
                    Position::K7 => board.k7 = Some(piece),
                    Position::A6 => board.a6 = Some(piece),
                    Position::B6 => board.b6 = Some(piece),
                    Position::C6 => board.c6 = Some(piece),
                    Position::D6 => board.d6 = Some(piece),
                    Position::E6 => board.e6 = Some(piece),
                    Position::F6 => board.f6 = Some(piece),
                    Position::G6 => board.g6 = Some(piece),
                    Position::H6 => board.h6 = Some(piece),
                    Position::I6 => board.i6 = Some(piece),
                    Position::K6 => board.k6 = Some(piece),
                    Position::L6 => board.l6 = Some(piece),
                    Position::A5 => board.a5 = Some(piece),
                    Position::B5 => board.b5 = Some(piece),
                    Position::C5 => board.c5 = Some(piece),
                    Position::D5 => board.d5 = Some(piece),
                    Position::E5 => board.e5 = Some(piece),
                    Position::F5 => board.f5 = Some(piece),
                    Position::G5 => board.g5 = Some(piece),
                    Position::H5 => board.h5 = Some(piece),
                    Position::I5 => board.i5 = Some(piece),
                    Position::K5 => board.k5 = Some(piece),
                    Position::L5 => board.l5 = Some(piece),
                    Position::A4 => board.a4 = Some(piece),
                    Position::B4 => board.b4 = Some(piece),
                    Position::C4 => board.c4 = Some(piece),
                    Position::D4 => board.d4 = Some(piece),
                    Position::E4 => board.e4 = Some(piece),
                    Position::F4 => board.f4 = Some(piece),
                    Position::G4 => board.g4 = Some(piece),
                    Position::H4 => board.h4 = Some(piece),
                    Position::I4 => board.i4 = Some(piece),
                    Position::K4 => board.k4 = Some(piece),
                    Position::L4 => board.l4 = Some(piece),
                    Position::A3 => board.a3 = Some(piece),
                    Position::B3 => board.b3 = Some(piece),
                    Position::C3 => board.c3 = Some(piece),
                    Position::D3 => board.d3 = Some(piece),
                    Position::E3 => board.e3 = Some(piece),
                    Position::F3 => board.f3 = Some(piece),
                    Position::G3 => board.g3 = Some(piece),
                    Position::H3 => board.h3 = Some(piece),
                    Position::I3 => board.i3 = Some(piece),
                    Position::K3 => board.k3 = Some(piece),
                    Position::L3 => board.l3 = Some(piece),
                    Position::A2 => board.a2 = Some(piece),
                    Position::B2 => board.b2 = Some(piece),
                    Position::C2 => board.c2 = Some(piece),
                    Position::D2 => board.d2 = Some(piece),
                    Position::E2 => board.e2 = Some(piece),
                    Position::F2 => board.f2 = Some(piece),
                    Position::G2 => board.g2 = Some(piece),
                    Position::H2 => board.h2 = Some(piece),
                    Position::I2 => board.i2 = Some(piece),
                    Position::K2 => board.k2 = Some(piece),
                    Position::L2 => board.l2 = Some(piece),
                    Position::A1 => board.a1 = Some(piece),
                    Position::B1 => board.b1 = Some(piece),
                    Position::C1 => board.c1 = Some(piece),
                    Position::D1 => board.d1 = Some(piece),
                    Position::E1 => board.e1 = Some(piece),
                    Position::F1 => board.f1 = Some(piece),
                    Position::G1 => board.g1 = Some(piece),
                    Position::H1 => board.h1 = Some(piece),
                    Position::I1 => board.i1 = Some(piece),
                    Position::K1 => board.k1 = Some(piece),
                    Position::L1 => board.l1 = Some(piece),
                }
            });

        match valid {
            true => Ok(board),
            false => Err(Failure::InvalidBoard),
        }

    }
}

/// Get the siblings of a position.
/// 
/// These describe the relationship of positions relative to one another. Think
/// of these like the hands of a clock, with our selected position at the center.
/// The first sibling is at 12 o'clock, the second is at 1 o'clock, and so on.
pub fn get_siblings(position: Position) -> [Option<Position>; 12] {
    match position {
        Position::A1 => [
            Some(Position::A2),
            Some(Position::B3),
            Some(Position::B2),
            Some(Position::C2),
            Some(Position::B1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        Position::A2 => [
            Some(Position::A3),
            Some(Position::B4),
            Some(Position::B3),
            Some(Position::C3),
            Some(Position::B2),
            Some(Position::B1),
            Some(Position::A1),
            None,
            None,
            None,
            None,
            None,
        ],
        Position::A3 => [
            Some(Position::A4),
            Some(Position::B5),
            Some(Position::B4),
            Some(Position::C4),
            Some(Position::B3),
            Some(Position::B2),
            Some(Position::A2),
            None,
            None,
            None,
            None,
            None,
        ],
        Position::A4 => [
            Some(Position::A5),
            Some(Position::B6),
            Some(Position::B5),
            Some(Position::C5),
            Some(Position::B4),
            Some(Position::B3),
            Some(Position::A3),
            None,
            None,
            None,
            None,
            None,
        ],
        Position::A5 => [
            Some(Position::A6),
            Some(Position::B7),
            Some(Position::B6),
            Some(Position::C6),
            Some(Position::B5),
            Some(Position::B4),
            Some(Position::A4),
            None,
            None,
            None,
            None,
            None,
        ],
        Position::A6 => [
            None,
            None,
            Some(Position::B7),
            Some(Position::C7),
            Some(Position::B6),
            Some(Position::B5),
            Some(Position::A5),
            None,
            None,
            None,
            None,
            None,
        ],
        Position::B1 => [
            Some(Position::B2),
            Some(Position::C3),
            Some(Position::C2),
            Some(Position::D2),
            Some(Position::C1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::A1),
            Some(Position::A2),
        ],
        Position::B2 => [
            Some(Position::B3),
            Some(Position::C4),
            Some(Position::C3),
            Some(Position::D3),
            Some(Position::C2),
            Some(Position::C1),
            Some(Position::B1),
            None,
            Some(Position::A1),
            None,
            Some(Position::A2),
            Some(Position::A3),
        ],
        Position::B3 => [
            Some(Position::B4),
            Some(Position::C5),
            Some(Position::C4),
            Some(Position::D4),
            Some(Position::C3),
            Some(Position::C2),
            Some(Position::B2),
            Some(Position::A1),
            Some(Position::A2),
            None,
            Some(Position::A3),
            Some(Position::A4),
        ],
        Position::B4 => [
            Some(Position::B5),
            Some(Position::C6),
            Some(Position::C5),
            Some(Position::D5),
            Some(Position::C4),
            Some(Position::C3),
            Some(Position::B3),
            Some(Position::A2),
            Some(Position::A3),
            None,
            Some(Position::A4),
            Some(Position::A5),
        ],
        Position::B5 => [
            Some(Position::B6),
            Some(Position::C7),
            Some(Position::C6),
            Some(Position::D6),
            Some(Position::C5),
            Some(Position::C4),
            Some(Position::B4),
            Some(Position::A3),
            Some(Position::A4),
            None,
            Some(Position::A5),
            Some(Position::A6),
        ],
        Position::B6 => [
            Some(Position::B7),
            Some(Position::C8),
            Some(Position::C7),
            Some(Position::D7),
            Some(Position::C6),
            Some(Position::C5),
            Some(Position::B5),
            Some(Position::A4),
            Some(Position::A5),
            None,
            Some(Position::A6),
            None,
        ],
        Position::B7 => [
            None,
            None,
            Some(Position::C8),
            Some(Position::D8),
            Some(Position::C7),
            Some(Position::C6),
            Some(Position::B6),
            Some(Position::A5),
            Some(Position::A6),
            None,
            None,
            None,
        ],
        Position::C1 => [
            Some(Position::C2),
            Some(Position::D3),
            Some(Position::D2),
            Some(Position::E2),
            Some(Position::D1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::B1),
            Some(Position::B2),
        ],
        Position::C2 => [
            Some(Position::C3),
            Some(Position::D4),
            Some(Position::D3),
            Some(Position::E3),
            Some(Position::D2),
            Some(Position::D1),
            Some(Position::C1),
            None,
            Some(Position::B1),
            Some(Position::A1),
            Some(Position::B2),
            Some(Position::B3),
        ],
        Position::C3 => [
            Some(Position::C4),
            Some(Position::D5),
            Some(Position::D4),
            Some(Position::E4),
            Some(Position::D3),
            Some(Position::D2),
            Some(Position::C2),
            Some(Position::B1),
            Some(Position::B2),
            Some(Position::A2),
            Some(Position::B3),
            Some(Position::B4),
        ],
        Position::C4 => [
            Some(Position::C5),
            Some(Position::D6),
            Some(Position::D5),
            Some(Position::E5),
            Some(Position::D4),
            Some(Position::D3),
            Some(Position::C3),
            Some(Position::B2),
            Some(Position::B3),
            Some(Position::A3),
            Some(Position::B4),
            Some(Position::B5),
        ],
        Position::C5 => [
            Some(Position::C6),
            Some(Position::D7),
            Some(Position::D6),
            Some(Position::E6),
            Some(Position::D5),
            Some(Position::D4),
            Some(Position::C4),
            Some(Position::B3),
            Some(Position::B4),
            Some(Position::A4),
            Some(Position::B5),
            Some(Position::B6),
        ],
        Position::C6 => [
            Some(Position::C7),
            Some(Position::D8),
            Some(Position::D7),
            Some(Position::E7),
            Some(Position::D6),
            Some(Position::D5),
            Some(Position::C5),
            Some(Position::B4),
            Some(Position::B5),
            Some(Position::A5),
            Some(Position::B6),
            Some(Position::B7),
        ],
        Position::C7 => [
            Some(Position::C8),
            Some(Position::D9),
            Some(Position::D8),
            Some(Position::E8),
            Some(Position::D7),
            Some(Position::D6),
            Some(Position::C6),
            Some(Position::B5),
            Some(Position::B6),
            Some(Position::A6),
            Some(Position::B7),
            None,
        ],
        Position::C8 => [
            None,
            None,
            Some(Position::D9),
            Some(Position::E9),
            Some(Position::D8),
            Some(Position::D7),
            Some(Position::C7),
            Some(Position::B6),
            Some(Position::B7),
            None,
            None,
            None,
        ],
        Position::D1 => [
            Some(Position::D2),
            Some(Position::E3),
            Some(Position::E2),
            Some(Position::F2),
            Some(Position::E1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::C1),
            Some(Position::C2),
        ],
        Position::D2 => [
            Some(Position::D3),
            Some(Position::E4),
            Some(Position::E3),
            Some(Position::F3),
            Some(Position::E2),
            Some(Position::E1),
            Some(Position::D1),
            None,
            Some(Position::C1),
            Some(Position::B1),
            Some(Position::C2),
            Some(Position::C3),
        ],
        Position::D3 => [
            Some(Position::D4),
            Some(Position::E5),
            Some(Position::E4),
            Some(Position::F4),
            Some(Position::E3),
            Some(Position::E2),
            Some(Position::D2),
            Some(Position::C1),
            Some(Position::C2),
            Some(Position::B2),
            Some(Position::C3),
            Some(Position::C4),
        ],
        Position::D4 => [
            Some(Position::D5),
            Some(Position::E6),
            Some(Position::E5),
            Some(Position::F5),
            Some(Position::E4),
            Some(Position::E3),
            Some(Position::D3),
            Some(Position::C2),
            Some(Position::C3),
            Some(Position::B3),
            Some(Position::C4),
            Some(Position::C5),
        ],
        Position::D5 => [
            Some(Position::D6),
            Some(Position::E7),
            Some(Position::E6),
            Some(Position::F6),
            Some(Position::E5),
            Some(Position::E4),
            Some(Position::D4),
            Some(Position::C3),
            Some(Position::C4),
            Some(Position::B4),
            Some(Position::C5),
            Some(Position::C6),
        ],
        Position::D6 => [
            Some(Position::D7),
            Some(Position::E8),
            Some(Position::E7),
            Some(Position::F7),
            Some(Position::E6),
            Some(Position::E5),
            Some(Position::D5),
            Some(Position::C4),
            Some(Position::C5),
            Some(Position::B5),
            Some(Position::C6),
            Some(Position::C7),
        ],
        Position::D7 => [
            Some(Position::D8),
            Some(Position::E9),
            Some(Position::E8),
            Some(Position::F8),
            Some(Position::E7),
            Some(Position::E6),
            Some(Position::D6),
            Some(Position::C5),
            Some(Position::C6),
            Some(Position::B6),
            Some(Position::C7),
            Some(Position::C8),
        ],
        Position::D8 => [
            Some(Position::D9),
            Some(Position::E10),
            Some(Position::E9),
            Some(Position::F9),
            Some(Position::E8),
            Some(Position::E7),
            Some(Position::D7),
            Some(Position::C6),
            Some(Position::C7),
            Some(Position::B7),
            Some(Position::C8),
            None,
        ],
        Position::D9 => [
            None,
            None,
            Some(Position::E10),
            Some(Position::F10),
            Some(Position::E9),
            Some(Position::E8),
            Some(Position::D8),
            Some(Position::C7),
            Some(Position::C8),
            None,
            None,
            None,
        ],
        Position::E1 => [
            Some(Position::E2),
            Some(Position::F3),
            Some(Position::F2),
            Some(Position::G1),
            Some(Position::F1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::D1),
            Some(Position::D2),
        ],
        Position::E2 => [
            Some(Position::E3),
            Some(Position::F4),
            Some(Position::F3),
            Some(Position::G2),
            Some(Position::F2),
            Some(Position::F1),
            Some(Position::E1),
            None,
            Some(Position::D1),
            Some(Position::C1),
            Some(Position::D2),
            Some(Position::D3),
        ],
        Position::E3 => [
            Some(Position::E4),
            Some(Position::F5),
            Some(Position::F4),
            Some(Position::G3),
            Some(Position::F3),
            Some(Position::F2),
            Some(Position::E2),
            Some(Position::D1),
            Some(Position::D2),
            Some(Position::C2),
            Some(Position::D3),
            Some(Position::D4),
        ],
        Position::E4 => [
            Some(Position::E5),
            Some(Position::F6),
            Some(Position::F5),
            Some(Position::G4),
            Some(Position::F4),
            Some(Position::F3),
            Some(Position::E3),
            Some(Position::D2),
            Some(Position::D3),
            Some(Position::C3),
            Some(Position::D4),
            Some(Position::D5),
        ],
        Position::E5 => [
            Some(Position::E6),
            Some(Position::F7),
            Some(Position::F6),
            Some(Position::G5),
            Some(Position::F5),
            Some(Position::F4),
            Some(Position::E4),
            Some(Position::D3),
            Some(Position::D4),
            Some(Position::C4),
            Some(Position::D5),
            Some(Position::D6),
        ],
        Position::E6 => [
            Some(Position::E7),
            Some(Position::F8),
            Some(Position::F7),
            Some(Position::G6),
            Some(Position::F6),
            Some(Position::F5),
            Some(Position::E5),
            Some(Position::D4),
            Some(Position::D5),
            Some(Position::C5),
            Some(Position::D6),
            Some(Position::D7),
        ],
        Position::E7 => [
            Some(Position::E8),
            Some(Position::F9),
            Some(Position::F8),
            Some(Position::G7),
            Some(Position::F7),
            Some(Position::F6),
            Some(Position::E6),
            Some(Position::D5),
            Some(Position::D6),
            Some(Position::C6),
            Some(Position::D7),
            Some(Position::D8),
        ],
        Position::E8 => [
            Some(Position::E9),
            Some(Position::F10),
            Some(Position::F9),
            Some(Position::G8),
            Some(Position::F8),
            Some(Position::F7),
            Some(Position::E7),
            Some(Position::D6),
            Some(Position::D7),
            Some(Position::C7),
            Some(Position::D8),
            Some(Position::D9),
        ],
        Position::E9 => [
            Some(Position::E10),
            Some(Position::F11),
            Some(Position::F10),
            Some(Position::G9),
            Some(Position::F9),
            Some(Position::F8),
            Some(Position::E8),
            Some(Position::D7),
            Some(Position::D8),
            Some(Position::C8),
            Some(Position::D9),
            None,
        ],
        Position::E10 => [
            None,
            None,
            Some(Position::F11),
            Some(Position::G10),
            Some(Position::F10),
            Some(Position::F9),
            Some(Position::E9),
            Some(Position::D8),
            Some(Position::D9),
            None,
            None,
            None,
        ],
        Position::F1 => [
            Some(Position::F2),
            Some(Position::G2),
            Some(Position::G1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Position::E1),
            Some(Position::E2),
        ],
        Position::F2 => [
            Some(Position::F3),
            Some(Position::G3),
            Some(Position::G2),
            Some(Position::H1),
            Some(Position::G1),
            None,
            Some(Position::F1),
            None,
            Some(Position::E1),
            Some(Position::D1),
            Some(Position::E2),
            Some(Position::E3),
        ],
        Position::F3 => [
            Some(Position::F4),
            Some(Position::G4),
            Some(Position::G3),
            Some(Position::H2),
            Some(Position::G2),
            Some(Position::G1),
            Some(Position::F2),
            Some(Position::E1),
            Some(Position::E2),
            Some(Position::D2),
            Some(Position::E3),
            Some(Position::E4),
        ],
        Position::F4 => [
            Some(Position::F5),
            Some(Position::G5),
            Some(Position::G4),
            Some(Position::H3),
            Some(Position::G3),
            Some(Position::G2),
            Some(Position::F3),
            Some(Position::E2),
            Some(Position::E3),
            Some(Position::D3),
            Some(Position::E4),
            Some(Position::E5),
        ],
        Position::F5 => [
            Some(Position::F6),
            Some(Position::G6),
            Some(Position::G5),
            Some(Position::H4),
            Some(Position::G4),
            Some(Position::G3),
            Some(Position::F4),
            Some(Position::E3),
            Some(Position::E4),
            Some(Position::D4),
            Some(Position::E5),
            Some(Position::E6),
        ],
        Position::F6 => [
            Some(Position::F7),
            Some(Position::G7),
            Some(Position::G6),
            Some(Position::H5),
            Some(Position::G5),
            Some(Position::G4),
            Some(Position::F5),
            Some(Position::E4),
            Some(Position::E5),
            Some(Position::D5),
            Some(Position::E6),
            Some(Position::E7),
        ],
        Position::F7 => [
            Some(Position::F8),
            Some(Position::G8),
            Some(Position::G7),
            Some(Position::H6),
            Some(Position::G6),
            Some(Position::G5),
            Some(Position::F6),
            Some(Position::E5),
            Some(Position::E6),
            Some(Position::D6),
            Some(Position::E7),
            Some(Position::E8),
        ],
        Position::F8 => [
            Some(Position::F9),
            Some(Position::G9),
            Some(Position::G8),
            Some(Position::H7),
            Some(Position::G7),
            Some(Position::G6),
            Some(Position::F7),
            Some(Position::E6),
            Some(Position::E7),
            Some(Position::D7),
            Some(Position::E8),
            Some(Position::E9),
        ],
        Position::F9 => [
            Some(Position::F10),
            Some(Position::G10),
            Some(Position::G9),
            Some(Position::H8),
            Some(Position::G8),
            Some(Position::G7),
            Some(Position::F8),
            Some(Position::E7),
            Some(Position::E8),
            Some(Position::D8),
            Some(Position::E9),
            Some(Position::E10),
        ],
        Position::F10 => [
            Some(Position::F11),
            None,
            Some(Position::G10),
            Some(Position::H9),
            Some(Position::G9),
            Some(Position::G8),
            Some(Position::F9),
            Some(Position::E8),
            Some(Position::E9),
            Some(Position::D9),
            Some(Position::E10),
            None,
        ],
        Position::F11 => [
            None,
            None,
            None,
            None,
            Some(Position::G10),
            Some(Position::G9),
            Some(Position::F10),
            Some(Position::E9),
            Some(Position::E10),
            None,
            None,
            None,
        ],
        Position::G1 => [
            Some(Position::G2),
            Some(Position::H2),
            Some(Position::H1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::F1),
            Some(Position::E1),
            Some(Position::F2),
            Some(Position::F3),
        ],
        Position::G2 => [
            Some(Position::G3),
            Some(Position::H3),
            Some(Position::H2),
            Some(Position::I1),
            Some(Position::H1),
            None,
            Some(Position::G1),
            Some(Position::F1),
            Some(Position::F2),
            Some(Position::E2),
            Some(Position::F3),
            Some(Position::F4),
        ],
        Position::G3 => [
            Some(Position::G4),
            Some(Position::H4),
            Some(Position::H3),
            Some(Position::I2),
            Some(Position::H2),
            Some(Position::H1),
            Some(Position::G2),
            Some(Position::F2),
            Some(Position::F3),
            Some(Position::E3),
            Some(Position::F4),
            Some(Position::F5),
        ],
        Position::G4 => [
            Some(Position::G5),
            Some(Position::H5),
            Some(Position::H4),
            Some(Position::I3),
            Some(Position::H3),
            Some(Position::H2),
            Some(Position::G3),
            Some(Position::F3),
            Some(Position::F4),
            Some(Position::E4),
            Some(Position::F5),
            Some(Position::F6),
        ],
        Position::G5 => [
            Some(Position::G6),
            Some(Position::H6),
            Some(Position::H5),
            Some(Position::I4),
            Some(Position::H4),
            Some(Position::H3),
            Some(Position::G4),
            Some(Position::F4),
            Some(Position::F5),
            Some(Position::E5),
            Some(Position::F6),
            Some(Position::F7),
        ],
        Position::G6 => [
            Some(Position::G7),
            Some(Position::H7),
            Some(Position::H6),
            Some(Position::I5),
            Some(Position::H5),
            Some(Position::H4),
            Some(Position::G5),
            Some(Position::F5),
            Some(Position::F6),
            Some(Position::E6),
            Some(Position::F7),
            Some(Position::F8),
        ],
        Position::G7 => [
            Some(Position::G8),
            Some(Position::H8),
            Some(Position::H7),
            Some(Position::I6),
            Some(Position::H6),
            Some(Position::H5),
            Some(Position::G6),
            Some(Position::F6),
            Some(Position::F7),
            Some(Position::E7),
            Some(Position::F8),
            Some(Position::F9),
        ],
        Position::G8 => [
            Some(Position::G9),
            Some(Position::H9),
            Some(Position::H8),
            Some(Position::I7),
            Some(Position::H7),
            Some(Position::H6),
            Some(Position::G7),
            Some(Position::F7),
            Some(Position::F8),
            Some(Position::E8),
            Some(Position::F9),
            Some(Position::F10),
        ],
        Position::G9 => [
            Some(Position::G10),
            None,
            Some(Position::H9),
            Some(Position::I8),
            Some(Position::H8),
            Some(Position::H7),
            Some(Position::G8),
            Some(Position::F8),
            Some(Position::F9),
            Some(Position::E9),
            Some(Position::F10),
            Some(Position::F11),
        ],
        Position::G10 => [
            None,
            None,
            None,
            None,
            Some(Position::H9),
            Some(Position::H8),
            Some(Position::G9),
            Some(Position::F9),
            Some(Position::F10),
            Some(Position::E10),
            Some(Position::F11),
            None,
        ],
        Position::H1 => [
            Some(Position::H2),
            Some(Position::I2),
            Some(Position::I1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::G1),
            Some(Position::F2),
            Some(Position::G2),
            Some(Position::G3),
        ],
        Position::H2 => [
            Some(Position::H3),
            Some(Position::I3),
            Some(Position::I2),
            Some(Position::K1),
            Some(Position::I1),
            None,
            Some(Position::H1),
            Some(Position::G1),
            Some(Position::G2),
            Some(Position::F3),
            Some(Position::G3),
            Some(Position::G4),
        ],
        Position::H3 => [
            Some(Position::H4),
            Some(Position::I4),
            Some(Position::I3),
            Some(Position::K2),
            Some(Position::I2),
            Some(Position::I1),
            Some(Position::H2),
            Some(Position::G2),
            Some(Position::G3),
            Some(Position::F4),
            Some(Position::G4),
            Some(Position::G5),
        ],
        Position::H4 => [
            Some(Position::H5),
            Some(Position::I5),
            Some(Position::I4),
            Some(Position::K3),
            Some(Position::I3),
            Some(Position::I2),
            Some(Position::H3),
            Some(Position::G3),
            Some(Position::G4),
            Some(Position::F5),
            Some(Position::G5),
            Some(Position::G6),
        ],
        Position::H5 => [
            Some(Position::H6),
            Some(Position::I6),
            Some(Position::I5),
            Some(Position::K4),
            Some(Position::I4),
            Some(Position::I3),
            Some(Position::H4),
            Some(Position::G4),
            Some(Position::G5),
            Some(Position::F6),
            Some(Position::G6),
            Some(Position::G7),
        ],
        Position::H6 => [
            Some(Position::H7),
            Some(Position::I7),
            Some(Position::I6),
            Some(Position::K5),
            Some(Position::I5),
            Some(Position::I4),
            Some(Position::H5),
            Some(Position::G5),
            Some(Position::G6),
            Some(Position::F7),
            Some(Position::G7),
            Some(Position::G8),
        ],
        Position::H7 => [
            Some(Position::H8),
            Some(Position::I8),
            Some(Position::I7),
            Some(Position::K6),
            Some(Position::I6),
            Some(Position::I5),
            Some(Position::H6),
            Some(Position::G6),
            Some(Position::G7),
            Some(Position::F8),
            Some(Position::G8),
            Some(Position::G9),
        ],
        Position::H8 => [
            Some(Position::H9),
            None,
            Some(Position::I8),
            Some(Position::K7),
            Some(Position::I7),
            Some(Position::I6),
            Some(Position::H7),
            Some(Position::G7),
            Some(Position::G8),
            Some(Position::F9),
            Some(Position::G9),
            Some(Position::G10),
        ],
        Position::H9 => [
            None,
            None,
            None,
            None,
            Some(Position::I8),
            Some(Position::I7),
            Some(Position::H8),
            Some(Position::G8),
            Some(Position::G9),
            Some(Position::F10),
            Some(Position::G10),
            None,
        ],
        Position::I1 => [
            Some(Position::I2),
            Some(Position::K2),
            Some(Position::K1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::H1),
            Some(Position::G2),
            Some(Position::H2),
            Some(Position::H3),
        ],
        Position::I2 => [
            Some(Position::I3),
            Some(Position::K3),
            Some(Position::K2),
            Some(Position::L1),
            Some(Position::K1),
            None,
            Some(Position::I1),
            Some(Position::H1),
            Some(Position::H2),
            Some(Position::G3),
            Some(Position::H3),
            Some(Position::H4),
        ],
        Position::I3 => [
            Some(Position::I4),
            Some(Position::K4),
            Some(Position::K3),
            Some(Position::L2),
            Some(Position::K2),
            Some(Position::K1),
            Some(Position::I2),
            Some(Position::H2),
            Some(Position::H3),
            Some(Position::G4),
            Some(Position::H4),
            Some(Position::H5),
        ],
        Position::I4 => [
            Some(Position::I5),
            Some(Position::K5),
            Some(Position::K4),
            Some(Position::L3),
            Some(Position::K3),
            Some(Position::K2),
            Some(Position::I3),
            Some(Position::H3),
            Some(Position::H4),
            Some(Position::G5),
            Some(Position::H5),
            Some(Position::H6),
        ],
        Position::I5 => [
            Some(Position::I6),
            Some(Position::K6),
            Some(Position::K5),
            Some(Position::L4),
            Some(Position::K4),
            Some(Position::K3),
            Some(Position::I4),
            Some(Position::H4),
            Some(Position::H5),
            Some(Position::G6),
            Some(Position::H6),
            Some(Position::H7),
        ],
        Position::I6 => [
            Some(Position::I7),
            Some(Position::K7),
            Some(Position::K6),
            Some(Position::L5),
            Some(Position::K5),
            Some(Position::K4),
            Some(Position::I5),
            Some(Position::H5),
            Some(Position::H6),
            Some(Position::G7),
            Some(Position::H7),
            Some(Position::H8),
        ],
        Position::I7 => [
            Some(Position::I8),
            None,
            Some(Position::K7),
            Some(Position::L6),
            Some(Position::K6),
            Some(Position::K5),
            Some(Position::I6),
            Some(Position::H6),
            Some(Position::H7),
            Some(Position::G8),
            Some(Position::H8),
            Some(Position::H9),
        ],
        Position::I8 => [
            None,
            None,
            None,
            None,
            Some(Position::K7),
            Some(Position::K6),
            Some(Position::I7),
            Some(Position::H7),
            Some(Position::H8),
            Some(Position::G9),
            Some(Position::H9),
            None,
        ],
        Position::K1 => [
            Some(Position::K2),
            Some(Position::L2),
            Some(Position::L1),
            None,
            None,
            None,
            None,
            None,
            Some(Position::I1),
            Some(Position::H2),
            Some(Position::I2),
            Some(Position::I3),
        ],
        Position::K2 => [
            Some(Position::K3),
            Some(Position::L3),
            Some(Position::L2),
            None,
            Some(Position::L1),
            None,
            Some(Position::K1),
            Some(Position::I1),
            Some(Position::I2),
            Some(Position::H3),
            Some(Position::I3),
            Some(Position::I4),
        ],
        Position::K3 => [
            Some(Position::K4),
            Some(Position::L4),
            Some(Position::L3),
            None,
            Some(Position::L2),
            Some(Position::L1),
            Some(Position::K2),
            Some(Position::I2),
            Some(Position::I3),
            Some(Position::H4),
            Some(Position::I4),
            Some(Position::I5),
        ],
        Position::K4 => [
            Some(Position::K5),
            Some(Position::L5),
            Some(Position::L4),
            None,
            Some(Position::L3),
            Some(Position::L2),
            Some(Position::K3),
            Some(Position::I3),
            Some(Position::I4),
            Some(Position::H5),
            Some(Position::I5),
            Some(Position::I6),
        ],
        Position::K5 => [
            Some(Position::K6),
            Some(Position::L6),
            Some(Position::L5),
            None,
            Some(Position::L4),
            Some(Position::L3),
            Some(Position::K4),
            Some(Position::I4),
            Some(Position::I5),
            Some(Position::H6),
            Some(Position::I6),
            Some(Position::I7),
        ],
        Position::K6 => [
            Some(Position::K7),
            None,
            Some(Position::L6),
            None,
            Some(Position::L5),
            Some(Position::L4),
            Some(Position::K5),
            Some(Position::I5),
            Some(Position::I6),
            Some(Position::H7),
            Some(Position::I7),
            Some(Position::I8),
        ],
        Position::K7 => [
            None,
            None,
            None,
            None,
            Some(Position::L6),
            Some(Position::L5),
            Some(Position::K6),
            Some(Position::I6),
            Some(Position::I7),
            Some(Position::H8),
            Some(Position::I8),
            None,
        ],
        Position::L1 => [
            Some(Position::L2),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Position::K1),
            Some(Position::I2),
            Some(Position::K2),
            Some(Position::K3),
        ],
        Position::L2 => [
            Some(Position::L3),
            None,
            None,
            None,
            None,
            None,
            Some(Position::L1),
            Some(Position::K1),
            Some(Position::K2),
            Some(Position::I3),
            Some(Position::K3),
            Some(Position::K4),
        ],
        Position::L3 => [
            Some(Position::L4),
            None,
            None,
            None,
            None,
            None,
            Some(Position::L2),
            Some(Position::K2),
            Some(Position::K3),
            Some(Position::I4),
            Some(Position::K4),
            Some(Position::K5),
        ],
        Position::L4 => [
            Some(Position::L5),
            None,
            None,
            None,
            None,
            None,
            Some(Position::L3),
            Some(Position::K3),
            Some(Position::K4),
            Some(Position::I5),
            Some(Position::K5),
            Some(Position::K6),
        ],
        Position::L5 => [
            Some(Position::L6),
            None,
            None,
            None,
            None,
            None,
            Some(Position::L4),
            Some(Position::K4),
            Some(Position::K5),
            Some(Position::I6),
            Some(Position::K6),
            Some(Position::K7),
        ],
        Position::L6 => [
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Position::L5),
            Some(Position::K5),
            Some(Position::K6),
            Some(Position::I7),
            Some(Position::K7),
            None,
        ],
    }
}

/// Step from one position to a sibling position
pub fn get_step(position: Position, direction: u8) -> Option<Position> {
    match direction {
        0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 => get_siblings(position)[direction as usize],
        _ => panic!("[hexchess] invalid step direction ({})", direction)
    }
}

/// test if a pawn position is eligible for promotion
pub fn is_promotion_position(color: Color, position: Position) -> bool {
    match color {
        Color::White => match position {
            Position::A6 | Position::B7 | Position::C8 | Position::D9 | Position::E10 | Position::F11 | Position::G10 | Position::H9 | Position::I8 | Position::K7 | Position::L6 => true,
            _ => false,
        },
        Color::Black => match position {
            Position::A1 | Position::B1 | Position::C1 | Position::D1 | Position::E1 | Position::F1 | Position::G1 | Position::H1 | Position::I1 | Position::K1 | Position::L1 => true,
            _ => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_step() {
        assert_eq!(Some(Position::F7), get_step(Position::F6, 0));
        assert_eq!(Some(Position::G7), get_step(Position::F6, 1));
        assert_eq!(Some(Position::G6), get_step(Position::F6, 2));
        assert_eq!(Some(Position::H5), get_step(Position::F6, 3));
        assert_eq!(Some(Position::G5), get_step(Position::F6, 4));
        assert_eq!(Some(Position::G4), get_step(Position::F6, 5));
        assert_eq!(Some(Position::F5), get_step(Position::F6, 6));
        assert_eq!(Some(Position::E4), get_step(Position::F6, 7));
        assert_eq!(Some(Position::E5), get_step(Position::F6, 8));
        assert_eq!(Some(Position::D5), get_step(Position::F6, 9));
        assert_eq!(Some(Position::E6), get_step(Position::F6, 10));
        assert_eq!(Some(Position::E7), get_step(Position::F6, 11));
    }

    #[test]
    fn test_is_promotion_position() {
        assert_eq!(true, is_promotion_position(Color::White, Position::B7));
        assert_eq!(false, is_promotion_position(Color::Black, Position::B7));

        assert_eq!(true, is_promotion_position(Color::Black, Position::A1));
        assert_eq!(false, is_promotion_position(Color::White, Position::A1));
    }

    #[test]
    fn test_to_position() {
        assert_eq!(Ok(Position::A1), Position::from("a1"));
        assert_eq!(Err(Failure::InvalidPosition), Position::from("whoops"));
    }

    #[test]
    fn test_create_board_from_value() {
        let board = Board::from(INITIAL_BOARD);

        assert_eq!(board.unwrap().a1, None);
        assert_eq!(board.unwrap().a2, None);
        assert_eq!(board.unwrap().a3, None);
        assert_eq!(board.unwrap().a4, None);
        assert_eq!(board.unwrap().a5, None);
        assert_eq!(board.unwrap().a6, None);
        assert_eq!(board.unwrap().b1, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().b2, None);
        assert_eq!(board.unwrap().b3, None);
        assert_eq!(board.unwrap().b4, None);
        assert_eq!(board.unwrap().b5, None);
        assert_eq!(board.unwrap().b6, None);
        assert_eq!(board.unwrap().b7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().c1, Some(Piece::WhiteRook));
        assert_eq!(board.unwrap().c2, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().c3, None);
        assert_eq!(board.unwrap().c4, None);
        assert_eq!(board.unwrap().c5, None);
        assert_eq!(board.unwrap().c6, None);
        assert_eq!(board.unwrap().c7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().c8, Some(Piece::BlackRook));
        assert_eq!(board.unwrap().d1, Some(Piece::WhiteKnight));
        assert_eq!(board.unwrap().d2, None);
        assert_eq!(board.unwrap().d3, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().d4, None);
        assert_eq!(board.unwrap().d5, None);
        assert_eq!(board.unwrap().d6, None);
        assert_eq!(board.unwrap().d7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().d8, None);
        assert_eq!(board.unwrap().d9, Some(Piece::BlackKnight));
        assert_eq!(board.unwrap().e1, Some(Piece::WhiteQueen));
        assert_eq!(board.unwrap().e2, None);
        assert_eq!(board.unwrap().e3, None);
        assert_eq!(board.unwrap().e4, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().e5, None);
        assert_eq!(board.unwrap().e6, None);
        assert_eq!(board.unwrap().e7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().e8, None);
        assert_eq!(board.unwrap().e9, None);
        assert_eq!(board.unwrap().e10, Some(Piece::BlackQueen));
        assert_eq!(board.unwrap().f1, Some(Piece::WhiteBishop));
        assert_eq!(board.unwrap().f2, Some(Piece::WhiteBishop));
        assert_eq!(board.unwrap().f3, Some(Piece::WhiteBishop));
        assert_eq!(board.unwrap().f4, None);
        assert_eq!(board.unwrap().f5, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().f6, None);
        assert_eq!(board.unwrap().f7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().f8, None);
        assert_eq!(board.unwrap().f9, Some(Piece::BlackBishop));
        assert_eq!(board.unwrap().f10, Some(Piece::BlackBishop));
        assert_eq!(board.unwrap().f11, Some(Piece::BlackBishop));
        assert_eq!(board.unwrap().g1, Some(Piece::WhiteKing));
        assert_eq!(board.unwrap().g2, None);
        assert_eq!(board.unwrap().g3, None);
        assert_eq!(board.unwrap().g4, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().g5, None);
        assert_eq!(board.unwrap().g6, None);
        assert_eq!(board.unwrap().g7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().g8, None);
        assert_eq!(board.unwrap().g9, None);
        assert_eq!(board.unwrap().g10, Some(Piece::BlackKing));
        assert_eq!(board.unwrap().h1, Some(Piece::WhiteKnight));
        assert_eq!(board.unwrap().h2, None);
        assert_eq!(board.unwrap().h3, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().h4, None);
        assert_eq!(board.unwrap().h5, None);
        assert_eq!(board.unwrap().h6, None);
        assert_eq!(board.unwrap().h7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().h8, None);
        assert_eq!(board.unwrap().h9, Some(Piece::BlackKnight));
        assert_eq!(board.unwrap().i1, Some(Piece::WhiteRook));
        assert_eq!(board.unwrap().i2, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().i3, None);
        assert_eq!(board.unwrap().i4, None);
        assert_eq!(board.unwrap().i5, None);
        assert_eq!(board.unwrap().i6, None);
        assert_eq!(board.unwrap().i7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().i8, Some(Piece::BlackRook));
        assert_eq!(board.unwrap().k1, Some(Piece::WhitePawn));
        assert_eq!(board.unwrap().k2, None);
        assert_eq!(board.unwrap().k3, None);
        assert_eq!(board.unwrap().k4, None);
        assert_eq!(board.unwrap().k5, None);
        assert_eq!(board.unwrap().k6, None);
        assert_eq!(board.unwrap().k7, Some(Piece::BlackPawn));
        assert_eq!(board.unwrap().l1, None);
        assert_eq!(board.unwrap().l2, None);
        assert_eq!(board.unwrap().l3, None);
        assert_eq!(board.unwrap().l4, None);
        assert_eq!(board.unwrap().l5, None);
        assert_eq!(board.unwrap().l6, None);
    }
}