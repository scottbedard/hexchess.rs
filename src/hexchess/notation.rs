use core::panic;

use crate::constants::{
    Piece,
    PromotionPiece,
    San,
};

use crate::hexchess::utils::{
    is_promotion_position,
    to_index,
};

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

/// parse the board segment of fen
pub fn parse_board(source: String) -> [Option<Piece>; 91] {
    let mut arr = [None; 91];
    let mut black = false;
    let mut white = false;
    let mut fen_index: u8 = 0;

    for (index, current) in source.chars().enumerate() {
        match current {
            '/' => continue,
            '1' => match source.chars().nth(index as usize + 1) {
                Some('0') => fen_index += 9,
                Some('1') => fen_index += 10,
                _ => fen_index += 1,
            },
            '2' => fen_index += 2,
            '3' => fen_index += 3,
            '4' => fen_index += 4,
            '5' => fen_index += 5,
            '6' => fen_index += 6,
            '7' => fen_index += 7,
            '8' => fen_index += 8,
            '9' => fen_index += 9,
            'b' |
            'B' |
            'n' |
            'N' |
            'p' |
            'P' |
            'Q' |
            'q' |
            'r' |
            'R' => {
                arr[fen_index as usize] = Some(to_piece(current));
                fen_index += 1;
            }
            'k' => {
                if black {
                    panic!("Multiple black kings");
                }

                arr[fen_index as usize] = Some(Piece::BlackKing);
                black = true;
                fen_index += 1;
            }
            'K' => {
                if white {
                    panic!("Multiple white kings");
                }

                arr[fen_index as usize] = Some(Piece::WhiteKing);
                white = true;
                fen_index += 1;
            },
            _ => panic!("Invalid character in FEN at index {}: {}", index, current),
        }
    }

    if fen_index != 91 {
        panic!("Invalid FEN length {}: {}", fen_index, source);
    }

    arr
}

/// parse standard algebraic notation
pub fn parse_move<'a>(source: &str) -> Result<San, &'a str> {
    let mut chars = source.chars();

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
            None => return Err("missing_from_file"),
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
            false => return Err("invalid_to_second_character"),
        },
        None => return Err("missing_to_second_character"),
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
    let from = match to_index(&(from_file.to_string() + &from_rank)) {
        Ok(value) => value,
        Err(_) => return Err("invalid_from_position"),
    };
    
    let to = match to_index(&(to_file.to_string() + &to_rank)) {
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
                  _ => return Err("invalid_promotion_character"),
                },
                _ => None,
            }
        },
        None => None
    };

    // validate promotion to is valid
    if promotion.is_some() && !is_promotion_position(to) {
        return Err("invalid_promotion_position");
    }

    // prohibit post-promotion characters
    if chars.next().is_some() {
        return Err("post_promotion_character");
    }

    Ok(San { from, promotion, to })
}

/// convert character to piece
pub fn to_piece(source: char) -> Piece {
    match source {
        'p' => Piece::BlackPawn,
        'n' => Piece::BlackKnight,
        'b' => Piece::BlackBishop,
        'r' => Piece::BlackRook,
        'q' => Piece::BlackQueen,
        'k' => Piece::BlackKing,
        'P' => Piece::WhitePawn,
        'N' => Piece::WhiteKnight,
        'B' => Piece::WhiteBishop,
        'R' => Piece::WhiteRook,
        'Q' => Piece::WhiteQueen,
        'K' => Piece::WhiteKing,
        _ => panic!("Invalid piece symbol: {}", source),
    }
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    mod parse_board {
        use super::*;
      
        #[test]
        fn parse_piece() {
            assert_eq!(to_piece('p'), Piece::BlackPawn);
            assert_eq!(to_piece('n'), Piece::BlackKnight);
            assert_eq!(to_piece('b'), Piece::BlackBishop);
            assert_eq!(to_piece('r'), Piece::BlackRook);
            assert_eq!(to_piece('q'), Piece::BlackQueen);
            assert_eq!(to_piece('k'), Piece::BlackKing);
            assert_eq!(to_piece('P'), Piece::WhitePawn);
            assert_eq!(to_piece('N'), Piece::WhiteKnight);
            assert_eq!(to_piece('B'), Piece::WhiteBishop);
            assert_eq!(to_piece('R'), Piece::WhiteRook);
            assert_eq!(to_piece('Q'), Piece::WhiteQueen);
            assert_eq!(to_piece('K'), Piece::WhiteKing);
        }

        #[test]
        fn empty_board() {
            let result = parse_board("1/3/5/7/9/11/11/11/11/11/11".to_string());

            assert_eq!(result, [None; 91]);
        }

        #[test]
        fn initial_board() {
            let result = parse_board("b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string());

            assert!(result.iter().eq([
                Some(Piece::BlackBishop),
                Some(Piece::BlackQueen),
                Some(Piece::BlackBishop),
                Some(Piece::BlackKing),
                Some(Piece::BlackKnight),
                None,
                Some(Piece::BlackBishop),
                None,
                Some(Piece::BlackKnight),
                Some(Piece::BlackRook),
                None,
                None,
                None,
                None,
                None,
                Some(Piece::BlackRook),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                Some(Piece::BlackPawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                Some(Piece::WhiteBishop),
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                Some(Piece::WhiteBishop),
                None,
                None,
                Some(Piece::WhitePawn),
                None,
                None,
                None,
                Some(Piece::WhitePawn),
                Some(Piece::WhiteRook),
                Some(Piece::WhiteKnight),
                Some(Piece::WhiteQueen),
                Some(Piece::WhiteBishop),
                Some(Piece::WhiteKing),
                Some(Piece::WhiteKnight),
                Some(Piece::WhiteRook),
                Some(Piece::WhitePawn),
                None,
            ].iter()));
        }
  
    }
    

    mod parse_move {
        use super::*;

        #[test]
        fn no_promotion() {
            assert_eq!(
                parse_move("a1b2"),
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
                parse_move("f10f11b"),
                Ok(San {
                    from: hex!("f10"),
                    promotion: Some(PromotionPiece::Bishop),
                    to: hex!("f11"),
                })
            );
        }
    
        #[test]
        fn empty_string() {
            assert_eq!(parse_move(""), Err("missing_from_file"));
        }
    
        #[test]
        fn missing_rank() {
            assert_eq!(parse_move("a"), Err("missing_second_character"));
        }
    
        #[test]
        fn missing_third_character() {
            assert_eq!(parse_move("a1"), Err("missing_third_character"));
        }
    
        #[test]
        fn invalid_second_character() {
            assert_eq!(parse_move("ax"), Err("invalid_second_character"));
        }
    
        #[test]
        fn invalid_to_file() {
            assert_eq!(parse_move("a1x"), Err("invalid_to_file"));
            assert_eq!(parse_move("a10x"), Err("invalid_to_file"));
            assert_eq!(parse_move("a11x"), Err("invalid_to_file"));
        }
    
        #[test]
        fn invalid_to_second_char() {
            assert_eq!(parse_move("a1ax"), Err("invalid_to_second_character"));
        }
          
        #[test]
        fn missing_to_file() {
            assert_eq!(parse_move("a10"), Err("missing_from_file"));
        }
    
        #[test]
        fn missing_to_second_char() {
            assert_eq!(parse_move("f1f"), Err("missing_to_second_character"));
            assert_eq!(parse_move("f10f"), Err("missing_to_second_character"));
            assert_eq!(parse_move("f11f"), Err("missing_to_second_character"));
        }
          
        #[test]
        fn invalid_to_rank() {
            assert_eq!(parse_move("a1f12"), Err("invalid_to_rank"));
        }
          
        #[test]
        fn invalid_to_second_character() {
            assert_eq!(parse_move("a1abc2"), Err("invalid_to_second_character"));
        }
          
        #[test]
        fn invalid_from_position() {
            assert_eq!(parse_move("a9a1"), Err("invalid_from_position"));
        }
          
        #[test]
        fn invalid_to_position() {
            assert_eq!(parse_move("a1a9"), Err("invalid_to_position"));
        }
          
        #[test]
        fn invalid_promotion_character() {
            assert_eq!(parse_move("f10f11x"), Err("invalid_promotion_character"));
        }
          
        #[test]
        fn notation_with_invalid_from_and_to() {
            assert_eq!(parse_move("x1x2"), Err("invalid_from_file"));
        }
          
        #[test]
        fn notation_with_identical_from_and_to() {
            assert_eq!(parse_move("a1a1"), Err("identical_from_and_to"));
        }
          
        #[test]
        fn post_promotion_character() {
            assert_eq!(parse_move("f10f11qq"), Err("post_promotion_character"));
        }
          
        #[test]
        fn invalid_promotion_position() {
            assert_eq!(parse_move("f10f6q"), Err("invalid_promotion_position"));
        }
    }
}
