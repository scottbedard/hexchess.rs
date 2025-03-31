use crate::constants::{Color, Piece};
use crate::structs::Hexchess;

pub fn create_hexchess() -> Hexchess {
    Hexchess {
        board: [None; 91],
        ep: None,
        fullmove: 1,
        halfmove: 0,
        turn: Color::White,
    }
}

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

/// parse the board segment of fen string
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_piece() {
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
    fn test_parse_board_empty() {
        let result = parse_board("1/3/5/7/9/11/11/11/11/11/11".to_string());

        assert_eq!(result, [None; 91]);
    }

    #[test]
    fn test_parse_board_initial() {
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
