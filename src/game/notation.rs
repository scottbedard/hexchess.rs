use crate::game::board::Position;
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
    pub fn from(value: &str) -> Result<Self, &str> {
        let mut chars = value.chars();

        // first file
        let from_file = match chars.next() {
            Some(c) => match is_file(c) {
                true => c,
                false => return Err("invalid_from_file"),
            },
            None => return Err("missing_from_file"),
        };
        
        // get next two chars to determine if from rank is 11
        let second_char = match chars.next() {
            Some(c) => match is_rank(c) {
                true => c,
                false => return Err("invalid_second_character"),
            },
            None => return Err("missing_second_character"),
        };

        let third_char = match chars.next() {
            Some(c) => c,
            None => return Err("missing_third_character"),
        };
        
        // first rank
        let from_rank = match (second_char, third_char) {
            ('1', '0') => String::from("10"),
            ('1', '1') => String::from("11"),
            _ => match is_rank(second_char) {
                true => second_char.to_string(),
                false => return Err("invalid_from_rank"),
            }
        };
        
        let to_file = match from_rank.as_str() {
            "10" | "11" => match chars.next() {
                Some(c) => match is_file(c) {
                    true => c,
                    false => return Err("invalid_to_file"),
                },
                None => return Err("missing_to_file"),
            },
            _ => match is_file(third_char) {
                true => third_char,
                false => return Err("invalid_to_file"),
            },
        };

        // gather next two chars to determine if to rank is 11
        let to_second_char = match chars.next() {
            Some(c) => match is_rank(c) {
                true => c,
                false => return Err("invalid_to_second_char"),
            },
            None => return Err("missing_to_second_char"),
        };

        let to_third_char = chars.next();

        // to rank
        let to_rank = match (to_second_char, to_third_char) {
            ('1', Some('0')) => String::from("10"),
            ('1', Some('1')) => String::from("11"),
            _ => match (is_rank(second_char), to_third_char) {
                (true, Some('b' | 'n' | 'r' | 'q') | None) => to_second_char.to_string(),
                _ => return Err("invalid_to_rank"),
            }
        };

        // assemble and validate from and to positions
        let from = match Position::from((from_file.to_string() + from_rank.as_str()).as_str()) {
            Ok(value) => value,
            Err(_) => return Err("invalid_from_position"),
        };

        let to = match Position::from((to_file.to_string() + to_rank.as_str()).as_str()) {
            Ok(value) => value,
            Err(_) => return Err("invalid_to_position"),
        };
        
        if from == to {
            return Err("identical_from_and_to");
        }

        // parse and validate promotion
        let promotion = match to_third_char {
            Some(c) => match c {
                'b' => Some(PromotionPiece::Bishop),
                'n' => Some(PromotionPiece::Knight),
                'q' => Some(PromotionPiece::Queen),
                'r' => Some(PromotionPiece::Rook),
                _ => match chars.next() {
                    Some(c2) => match c2 {
                        'b' => Some(PromotionPiece::Bishop),
                        'n' => Some(PromotionPiece::Knight),
                        'q' => Some(PromotionPiece::Queen),
                        'r' => Some(PromotionPiece::Rook),
                        _ => return Err("invalid_promotion_character")
                    },
                    _ => None,
                }
            },
            None => None
        };

        // validate promotion to is valid
        if promotion.is_some() && !is_promotable(to) {
            return Err("invalid_promotion_position");
        }

        // successful parsing
        Ok(Self { from, promotion, to })
    }
}

fn is_file(c: char) -> bool {
    match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'k' | 'l' => true,
        _ => false,
    }
}

fn is_rank(c: char) -> bool {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
        _ => false,
    }
}

fn is_promotable(position: Position) -> bool {
    match position {
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
        Position::L1 => true,
        _ => false,
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
    fn test_parse_empty_string() {
        assert_eq!(Err("missing_from_file"), Notation::from(""));
    }

    #[test]
    fn test_parse_missing_rank() {
        assert_eq!(Err("missing_second_character"), Notation::from("a"));
    }

    #[test]
    fn test_parse_missing_third_character() {
        assert_eq!(Err("missing_third_character"), Notation::from("a1"));
    }

    #[test]
    fn test_parse_invalid_to_file() {
        assert_eq!(Err("invalid_to_file"), Notation::from("a1x"));
    }

    #[test]
    fn test_parse_invalid_to_second_char() {
        assert_eq!(Err("invalid_to_second_char"), Notation::from("a1ax"));
    }

    #[test]
    fn test_parse_invalid_to_rank() {
        assert_eq!(Err("invalid_to_rank"), Notation::from("a1f12"));
    }

    #[test]
    fn test_parse_missing_to_file() {
        assert_eq!(Err("invalid_to_second_char"), Notation::from("a1abc2"));
    }

    #[test]
    fn test_parse_invalid_from_position() {
        assert_eq!(Err("invalid_from_position"), Notation::from("a9a1"));
    }

    #[test]
    fn test_parse_invalid_to_position() {
        assert_eq!(Err("invalid_to_position"), Notation::from("a1a9"));
    }

    #[test]
    fn test_parse_invalid_promotion_character() {
        assert_eq!(Err("invalid_promotion_character"), Notation::from("f10f11x"));
    }

    //

    #[test]
    fn test_parse_notation_with_invalid_from_and_to() {
        assert_eq!(Err("invalid_from_file"), Notation::from("x1x2"));
    }

    #[test]
    fn test_parse_notation_with_identical_from_and_to() {
        assert_eq!(Err("identical_from_and_to"), Notation::from("a1a1"));
    }

    #[test]
    fn test_parse_notation_with_invalid_second_file() {
        assert_eq!(Err("invalid_to_second_char"), Notation::from("a1abc2"));
    }

    #[test]
    fn test_stringify_notation() {
        assert_eq!("a1a2", Notation::from("a1a2").unwrap().to_string());
        assert_eq!("f10f11b", Notation::from("f10f11b").unwrap().to_string());
    }
}
