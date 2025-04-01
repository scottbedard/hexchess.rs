use crate::constants::PromotionPiece;

use crate::hexchess::utils::{
    is_promotion_position,
    to_index,
};

#[derive(Clone, Copy, Debug, PartialEq)]
struct San {
    from: u8,
    promotion: Option<PromotionPiece>,
    to: u8,
}

impl San {
    pub fn from(source: &String) -> Result<Self, String> {
        let mut chars = source.chars();

        // first file
        let from_file = match chars.next() {
            Some(val) => match is_file(val) {
                true => val,
                false => return Err(format!("invalid from file: {}", val)),
            },
            None => return Err("missing from file".to_string()),
        };
    
        // get next two chars to determine if from rank is 11
        let second_char = match chars.next() {
            Some(val) => match is_rank(val) {
                true => val,
                false => return Err(format!("invalid second character: {}", val)),
            },
            None => return Err("missing second character".to_string()),
        };
    
        let third_char = match chars.next() {
            Some(c) => c,
            None => return Err("missing third character".to_string()),
        };
    
        // first rank
        let from_rank = match (second_char, third_char) {
            ('1', '0') => String::from("10"),
            ('1', '1') => String::from("11"),
            _ => match is_rank(second_char) {
                true => second_char.to_string(),
                false => return Err(format!("invalid from rank: {}", second_char)),
            }
        };
    
        let to_file = match from_rank.as_str() {
            "10" | "11" => match chars.next() {
                Some(val) => match is_file(val) {
                  true => val,
                  false => return Err(format!("invalid to file: {}", val)),
                },
                None => return Err("missing from file".to_string()),
            },
            _ => match is_file(third_char) {
                true => third_char,
                false => return Err(format!("invalid to file: {}", third_char)),
            },
        };
    
        // gather next two chars to determine if to rank is 11
        let to_second_char = match chars.next() {
            Some(val) => match is_rank(val) {
                true => val,
                false => return Err(format!("invalid second to character: {}", val)),
            },
            None => return Err("missing second to character".to_string()),
        };
    
        let to_third_char = chars.next();
    
        // to rank
        let to_rank = match (to_second_char, to_third_char) {
            ('1', Some('0')) => String::from("10"),
            ('1', Some('1')) => String::from("11"),
            _ => match (is_rank(second_char), to_third_char) {
                (true, Some('b' | 'n' | 'r' | 'q') | None) => to_second_char.to_string(),
                _ => return Err("invalid to rank".to_string()),
            }
        };
    
        // assemble and validate from and to positions
        let from_source = from_file.to_string() + &from_rank;

        let from = match to_index(&from_source) {
            Ok(value) => value,
            Err(_) => return Err(format!("invalid from position: {}", from_source)),
        };

        let to_source = to_file.to_string() + &to_rank;

        let to = match to_index(&to_source) {
            Ok(value) => value,
            Err(_) => return Err(format!("invalid to position: {}", to_source)),
        };
        
        if from == to {
            return Err("to and from positions are the same".to_string());
        }
    
        // parse and validate promotion
        let promotion = match to_third_char {
            Some(val) => match val {
                'b' => Some(PromotionPiece::Bishop),
                'n' => Some(PromotionPiece::Knight),
                'q' => Some(PromotionPiece::Queen),
                'r' => Some(PromotionPiece::Rook),
                _ => match chars.next() {
                    Some(val_2) => match val_2 {
                        'b' => Some(PromotionPiece::Bishop),
                        'n' => Some(PromotionPiece::Knight),
                        'q' => Some(PromotionPiece::Queen),
                        'r' => Some(PromotionPiece::Rook),
                        _ => return Err(format!("invalid promotion character: {}", val_2)),
                    },
                    _ => None,
                }
            },
            None => None
        };
    
        // validate promotion to is valid
        if promotion.is_some() && !is_promotion_position(&to) {
            return Err(format!("invalid promotion position: {}", to_source));
        }
    
        // prohibit post-promotion characters
        if chars.next().is_some() {
            return Err("post promotion character".to_string());
        }
    
        Ok(Self { from, promotion, to })
    }
}

/// test if character is a file
fn is_file(c: char) -> bool {
    match c {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'k' | 'l' => true,
        _ => false,
    }
}

/// test if character could be part of a rank
fn is_rank(c: char) -> bool {
    match c {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    #[test]
    fn no_promotion() {
        assert_eq!(
            San::from(&"a1b2".to_string()),
            Ok(San {
                from: hex!("a1"),
                promotion: None,
                to: hex!("b2"),
            })
        );
    }

    #[test]
    fn with_promotion() {
        assert_eq!(
            San::from(&"f10f11b".to_string()),
            Ok(San {
                from: hex!("f10"),
                promotion: Some(PromotionPiece::Bishop),
                to: hex!("f11"),
            })
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(San::from(&"".to_string()), Err("missing from file".to_string()));
    }

    #[test]
    fn missing_rank() {
        assert_eq!(San::from(&"a".to_string()), Err("missing second character".to_string()));
    }

    #[test]
    fn missing_third_character() {
        assert_eq!(San::from(&"a1".to_string()), Err("missing third character".to_string()));
    }

    #[test]
    fn invalid_second_character() {
        assert_eq!(San::from(&"ax".to_string()), Err("invalid second character: x".to_string()));
    }

    #[test]
    fn invalid_to_file() {
        assert_eq!(San::from(&"a1x".to_string()), Err("invalid to file: x".to_string()));
        assert_eq!(San::from(&"a10x".to_string()), Err("invalid to file: x".to_string()));
        assert_eq!(San::from(&"a11x".to_string()), Err("invalid to file: x".to_string()));
    }

    #[test]
    fn invalid_to_second_char() {
        assert_eq!(San::from(&"a1ax".to_string()), Err("invalid second to character: x".to_string()));
    }

    #[test]
    fn missing_to_file() {
        assert_eq!(San::from(&"a10".to_string()), Err("missing from file".to_string()));
    }

    #[test]
    fn missing_to_second_char() {
        assert_eq!(San::from(&"f1f".to_string()), Err("missing second to character".to_string()));
        assert_eq!(San::from(&"f10f".to_string()), Err("missing second to character".to_string()));
        assert_eq!(San::from(&"f11f".to_string()), Err("missing second to character".to_string()));
    }

    #[test]
    fn invalid_to_rank() {
      assert_eq!(San::from(&"a1f12".to_string()), Err("invalid to rank".to_string()));
    }

    #[test]
    fn invalid_to_second_character() {
      assert_eq!(San::from(&"a1abc2".to_string()), Err("invalid second to character: b".to_string()));
    }

    #[test]
    fn invalid_from_position() {
      assert_eq!(San::from(&"a9a1".to_string()), Err("invalid from position: a9".to_string()));
    }

    #[test]
    fn invalid_to_position() {
      assert_eq!(San::from(&"a1a9".to_string()), Err("invalid to position: a9".to_string()));
    }

    #[test]
    fn invalid_promotion_character() {
      assert_eq!(San::from(&"f10f11x".to_string()), Err("invalid promotion character: x".to_string()));
    }

    #[test]
    fn notation_with_invalid_from_and_to() {
      assert_eq!(San::from(&"x1x2".to_string()), Err("invalid from file: x".to_string()));
    }

    #[test]
    fn notation_with_identical_from_and_to() {
      assert_eq!(San::from(&"a1a1".to_string()), Err("to and from positions are the same".to_string()));
    }

    #[test]
    fn post_promotion_character() {
      assert_eq!(San::from(&"f10f11qq".to_string()), Err("post promotion character".to_string()));
    }

    #[test]
    fn invalid_promotion_position() {
      assert_eq!(San::from(&"f10f6q".to_string()), Err("invalid promotion position: f6".to_string()));
    }
}