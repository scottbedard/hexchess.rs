use crate::game::board::Position;
use crate::game::failure::Failure;
use crate::game::piece::PromotionPiece;
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
        let mut from = String::from("");
        let mut parsing_from = false;
        let mut to = String::from("");
        let mut parsing_to = false;
        let mut promotion = None;

        for c in value.chars() {
            match c {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'k' | 'l' | 'q' | 'r' | 'n' => {
                    if !parsing_from {
                        parsing_from = true;
                        from.push(c);
                    } else if !parsing_to {
                        parsing_to = true;
                        to.push(c);
                    } else {
                        promotion = match c {
                            'b' => Some(PromotionPiece::Bishop),
                            'n' => Some(PromotionPiece::Knight),
                            'q' => Some(PromotionPiece::Queen),
                            'r' => Some(PromotionPiece::Rook),
                            _ => None
                        }
                    }
                },
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if !parsing_to {
                        from.push(c);
                    } else {
                        to.push(c);
                    }
                },
                _ => return Err(Failure::InvalidNotation),
            };
        }

        if from == to {
            return Err(Failure::InvalidNotation);
        }

        let to_str = to.as_str();

        if promotion.is_some() {
            match to_str {
                "a6" | "b7" | "c8"| "d9" | "e10" | "f11" | "g10" | "h9" | "i8" | "k7" | "l6" |
                "a1" | "b1" | "c1" | "d1" | "e1" | "f1" | "g1" | "h1" | "i1" | "k1" | "l1" => {},
                _ => return Err(Failure::InvalidNotation),
            };
        }

        Ok(Self {
            from: Position::from(from.as_str()).unwrap(),
            promotion,
            to: Position::from(to_str).unwrap(),
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
            to: Position::B2,
        }), Notation::from("a1b2"));
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
