use crate::game::board::Position;
use crate::game::failure::Failure;
use crate::game::piece::PromotionPiece;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use tsify::Tsify;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Notation {
    pub from: Position,

    pub promotion: Option<PromotionPiece>,

    pub to: Position,
}

impl Notation {
    pub fn from(value: &str) -> Result<Self, Failure> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<from>a1|a2|a3|a4|a5|a6|b1|b2|b3|b4|b5|b6|b7|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|d9|e1|e2|e3|e4|e5|e6|e7|e8|e9|e10|f1|f2|f3|f4|f5|f6|f7|f8|f9|f10|f11|g1|g2|g3|g4|g5|g6|g7|g8|g9|g10|h1|h2|h3|h4|h5|h6|h7|h8|h9|i1|i2|i3|i4|i5|i6|i7|i8|k1|k2|k3|k4|k5|k6|k7|l1|l2|l3|l4|l5|l6)(?<to>a1|a2|a3|a4|a5|a6|b1|b2|b3|b4|b5|b6|b7|c1|c2|c3|c4|c5|c6|c7|c8|d1|d2|d3|d4|d5|d6|d7|d8|d9|e1|e2|e3|e4|e5|e6|e7|e8|e9|e10|f1|f2|f3|f4|f5|f6|f7|f8|f9|f10|f11|g1|g2|g3|g4|g5|g6|g7|g8|g9|g10|h1|h2|h3|h4|h5|h6|h7|h8|h9|i1|i2|i3|i4|i5|i6|i7|i8|k1|k2|k3|k4|k5|k6|k7|l1|l2|l3|l4|l5|l6)(?<promotion>b|n|q|r)?$").unwrap());

        let captures = match RE.captures(value) {
            Some(value) => value,
            None => return Err(Failure::InvalidNotation),
        };

        // it's safe to unwrap because the regex guarantees that these values are valid
        let from = Position::from(captures.name("from").unwrap().as_str()).unwrap();

        let to = Position::from(captures.name("to").unwrap().as_str()).unwrap();

        if from == to {
            return Err(Failure::InvalidNotation);
        }

        let promotion = match captures.name("promotion") {
            Some(value) => Some(PromotionPiece::from(value.as_str()).unwrap()),
            None => None,
        };

        if promotion.is_some() {
            match to {
                Position::A6 |
                Position::B7 |
                Position::C8 |
                Position::D9 |
                Position::E10 |
                Position::F11 |
                Position::G10 |
                Position::H9 |
                Position::I8 |
                Position::K7 |
                Position::L6 |
                Position::A1 |
                Position::B1 |
                Position::C1 |
                Position::D1 |
                Position::E1 |
                Position::F1 |
                Position::G1 |
                Position::H1 |
                Position::I1 |
                Position::K1 |
                Position::L1 => (),
                _ => return Err(Failure::InvalidNotation),
            }
        }

        Ok(Self {
            from,
            promotion,
            to,
        })
    }
}

impl fmt::Display for Notation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.promotion {
            Some(value) => write!(f, "{}{}{}", self.from, self.to, value),
            None => write!(f, "{}{}", self.from, self.to),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_notation_without_promotion() {
        assert_eq!(Ok(Notation {
            from: Position::A1,
            promotion: None,
            to: Position::A2,
        }), Notation::from("a1a2"));
    }

    #[test]
    fn test_parse_notation_with_promotion() {
        assert_eq!(Ok(Notation {
            from: Position::F10,
            promotion: Some(PromotionPiece::Bishop),
            to: Position::F11,
        }), Notation::from("f10f11b"));
    }

    #[test]
    fn test_parse_promotion_on_non_promotion_position() {
        assert_eq!(Err(Failure::InvalidNotation), Notation::from("a1a2b"));
    }

    #[test]
    fn test_parse_notation_with_invalid_promotion() {
        assert_eq!(Err(Failure::InvalidNotation), Notation::from("f10f11x"));
    }

    #[test]
    fn test_parse_notation_with_invalid_from() {
        assert_eq!(Err(Failure::InvalidNotation), Notation::from("x1a2"));
    }

    #[test]
    fn test_parse_notation_with_invalid_to() {
        assert_eq!(Err(Failure::InvalidNotation), Notation::from("a1x2"));
    }

    #[test]
    fn test_parse_notation_with_invalid_from_and_to() {
        assert_eq!(Err(Failure::InvalidNotation), Notation::from("x1x2"));
    }

    #[test]
    fn test_parse_notation_with_identical_from_and_to() {
        assert_eq!(Err(Failure::InvalidNotation), Notation::from("a1a1"));
    }

    #[test]
    fn test_stringify_notation() {
        assert_eq!("a1a2", Notation::from("a1a2").unwrap().to_string());
        assert_eq!("f10f11b", Notation::from("f10f11b").unwrap().to_string());
    }
}
