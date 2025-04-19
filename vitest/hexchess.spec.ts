import { describe, expect, test } from 'vitest'
import { Hexchess, initialPosition, positions } from '../src'

describe('Hexchess', () => {
  test('clone', () => {
    const hexchess = Hexchess.init()
    const clone = hexchess.clone()

    expect(clone.board).toEqual(hexchess.board)
    expect(clone.ep).toEqual(hexchess.ep)
    expect(clone.turn).toEqual(hexchess.turn)
    expect(clone.halfmove).toEqual(hexchess.halfmove)
    expect(clone.fullmove).toEqual(hexchess.fullmove)

    expect(clone.board).not.toBe(hexchess.board)
    expect(clone).not.toBe(hexchess)
  })

  test('get', () => {
    const hexchess = Hexchess.init()

    expect(hexchess.get('g10')).toBe('k')
    expect(hexchess.get('g1')).toBe('K')
    expect(hexchess.get('a4')).toBe(null)
    // @ts-expect-error - invalid position
    expect(hexchess.get('whoops')).toBe(null)
  })

  describe('isThreatened', () => {
    test('unattacked position is not threatened', () => {
      const hexchess = Hexchess.parse('1/2K/5/7/9/11/11/11/11/11/11 w - 0 1')

      expect(hexchess.isThreatened('g10')).toBe(false)
    })

    test('threatened by enemy piece', () => {
      const hexchess = Hexchess.parse('1/2K/5/7/9/11/11/11/11/11/6r4 w - 0 1')

      expect(hexchess.isThreatened('g10')).toBe(true)
    })

    test('not threatened by friendly piece', () => {
      const hexchess = Hexchess.parse('1/2K/5/7/9/11/11/11/11/11/6R4 w - 0 1')

      expect(hexchess.isThreatened('g10')).toBe(false)
    })

    test('position is threatened in and out of turn', () => {
      const hexchess = Hexchess.parse('1/3/5/7/4q4/5K5/11/11/11/11/11 w - 0 1')

      hexchess.turn = 'b'
      expect(hexchess.isThreatened('f6')).toBe(true)

      hexchess.turn = 'w'
      expect(hexchess.isThreatened('f6')).toBe(true)
    })

    test('unoccupied position is not threatened', () => {
      const hexchess = new Hexchess()

      for (const position of positions) {
        expect(hexchess.isThreatened(position)).toBe(false)
      }
    })
  })

  test('parse', () => {
    const hexchess = Hexchess.parse(initialPosition)

    expect(hexchess.board).toEqual([
      'b', 'q', 'b', 'k', 'n', null, 'b', null, 'n', 'r',
      null, null, null, null, null, 'r', 'p', 'p', 'p', 'p',
      'p', 'p', 'p', 'p', 'p', null, null, null, null, null,
      null, null, null, null, null, null, null, null, null, null,
      null, 'P', null, null, null, null, null, null, null, null,
      null, 'P', null, 'P', null, null, null, null, null, null,
      null, 'P', null, 'B', null, 'P', null, null, null, null,
      null, 'P', null, null, 'B', null, null, 'P', null, null,
      null, 'P', 'R', 'N', 'Q', 'B', 'K', 'N', 'R', 'P',
      null,
    ])

    expect(hexchess.ep).toBeNull()

    expect(hexchess.turn).toBe('w')

    expect(hexchess.halfmove).toBe(0)

    expect(hexchess.fullmove).toBe(1)
  })

  describe('toString', () => {
    test('empty', () => {
      const hexchess = new Hexchess()

      expect(hexchess.toString()).toBe('1/3/5/7/9/11/11/11/11/11/11 w - 0 1')
    })

    test('initial position', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.toString()).toBe('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')
    })
  })
})

// #[cfg(test)]
// mod tests {
//     use crate::{h, s};
//     use super::*;

//     mod apply_move {
//         use super::*;

//         #[test]
//         fn sets_to_and_from_positions() {
//             let mut hexchess = Hexchess::init();
//             let _ = hexchess.apply_move(&s!("g4g5"));
//             let _ = hexchess.apply_move(&s!("e7e6"));
//             assert_eq!(hexchess.board[h!("g5")], Some(Piece::WhitePawn));
//             assert_eq!(hexchess.board[h!("g4")], None);
//             assert_eq!(hexchess.board[h!("e6")], Some(Piece::BlackPawn));
//             assert_eq!(hexchess.board[h!("e7")], None);
//         }

//         #[test]
//         fn clears_en_pasant_capture_white() {
//             let mut hexchess = Hexchess::parse("b/qbk/n1b1n/r5r/ppppp1ppp/5P5/6p4/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w g6 0 2").unwrap();
//             let _ = hexchess.apply_move(&s!("f6g6"));

//             assert_eq!(hexchess.get("g5"), None);
//         }

//         #[test]
//         fn clears_en_passant_capture_black() {
//             let mut hexchess = Hexchess::parse("b/qbk/n1b1n/r5r/pppp1pppp/5pP4/4PP5/11/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 2").unwrap();
//             let _ = hexchess.apply_move(&s!("f6g5"));

//             assert_eq!(hexchess.get("g6"), None);
//         }

//         #[test]
//         fn only_pawns_capture_en_passant() {
//             let mut hexchess = Hexchess::parse("b/qbk/n1b1n/r5r/ppppp1ppp/11/5Pp4/4P1PB3/3P1B1P3/2P5P2/1PRNQBKNRP1 w g6 0 2").unwrap();
//             let _ = hexchess.apply_move(&s!("h4g6")); // <- bishop to en passant

//             assert_eq!(hexchess.get("g5"), Some(Piece::BlackPawn));
//         }

//         #[test]
//         fn alternate_color_back_and_forth() {
//             let mut hexchess = Hexchess::init();

//             assert_eq!(hexchess.turn, Color::White);
//             let _ = hexchess.apply_move(&s!("g4g5"));
//             assert_eq!(hexchess.turn, Color::Black);
//             let _ = hexchess.apply_move(&s!("e7e6"));
//             assert_eq!(hexchess.turn, Color::White);
//             let _ = hexchess.apply_move(&s!("f5f6"));
//             assert_eq!(hexchess.turn, Color::Black);
//         }

//         #[test]
//         fn sets_and_unsets_en_passant() {
//             let mut hexchess = Hexchess::init();

//             let _ = hexchess.apply_move(&s!("g4g6"));
//             assert_eq!(hexchess.ep, Some(h!("g5")));

//             let _ = hexchess.apply_move(&s!("e7e5"));
//             assert_eq!(hexchess.ep, Some(h!("e6")));

//             let _ = hexchess.apply_move(&s!("b1b2"));
//             assert_eq!(hexchess.ep, None);
//         }

//         #[test]
//         fn sets_halfmove_and_fullmove() {
//             let mut hexchess = Hexchess::init();

//             assert_eq!(hexchess.halfmove, 0);
//             assert_eq!(hexchess.fullmove, 1);

//             let _ = hexchess.apply_move(&s!("e4e5"));
//             assert_eq!(hexchess.halfmove, 0);
//             assert_eq!(hexchess.fullmove, 1);

//             let _ = hexchess.apply_move(&s!("f7f6"));
//             assert_eq!(hexchess.halfmove, 0);
//             assert_eq!(hexchess.fullmove, 2);

//             let _ = hexchess.apply_move(&s!("f3c6"));
//             assert_eq!(hexchess.halfmove, 1);
//             assert_eq!(hexchess.fullmove, 2);

//             let _ = hexchess.apply_move(&s!("i8h8"));
//             assert_eq!(hexchess.halfmove, 2);
//             assert_eq!(hexchess.fullmove, 3);

//             let _ = hexchess.apply_move(&s!("c6e10"));
//             assert_eq!(hexchess.halfmove, 0);
//             assert_eq!(hexchess.fullmove, 3);
//         }

//         // promote white pieces
//         #[test]
//         fn white_and_black_promotions() {
//             let mut hexchess = Hexchess::parse("1/3/1P1P1/7/1P5P1/11/11/11/11/2p1p1p1p2/11 w - 0 1").unwrap();

//             let _ = hexchess.apply_move(&s!("c7c8r"));
//             assert_eq!(hexchess.board[h!("c8")], Some(Piece::WhiteRook));

//             let _ = hexchess.apply_move(&s!("c2c1r"));
//             assert_eq!(hexchess.board[h!("c1")], Some(Piece::BlackRook));

//             let _ = hexchess.apply_move(&s!("e9e10b"));
//             assert_eq!(hexchess.board[h!("e10")], Some(Piece::WhiteBishop));

//             let _ = hexchess.apply_move(&s!("e2e1b"));
//             assert_eq!(hexchess.board[h!("e1")], Some(Piece::BlackBishop));

//             let _ = hexchess.apply_move(&s!("g9g10q"));
//             assert_eq!(hexchess.board[h!("g10")], Some(Piece::WhiteQueen));

//             let _ = hexchess.apply_move(&s!("g2g1q"));
//             assert_eq!(hexchess.board[h!("g1")], Some(Piece::BlackQueen));

//             let _ = hexchess.apply_move(&s!("i7i8n"));
//             assert_eq!(hexchess.board[h!("i8")], Some(Piece::WhiteKnight));

//             let _ = hexchess.apply_move(&s!("i2i1n"));
//             assert_eq!(hexchess.board[h!("i1")], Some(Piece::BlackKnight));
//         }

//         #[test]
//         fn errors_on_illegal_move() {
//             let mut hexchess = Hexchess::init();

//             assert_eq!(hexchess.apply_move(&s!("a4a5")).is_err(), true);
//         }

//         #[test]
//         #[should_panic]
//         fn apply_move_unsafe_panics_on_empty_positions() {
//             let mut hexchess = Hexchess::init();

//             hexchess.apply_move_unsafe(&s!("a4a5"));
//         }
//     }

//     mod apply_sequence {
//         use super::*;

//         #[test]
//         fn test_applying_a_sequence_of_moves() {
//             let mut hexchess = Hexchess::init();
//             let _ = hexchess.apply_sequence("g4g6 f7g6 f5f7 g6f6");

//             assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3");
//         }

//         #[test]
//         fn test_apply_sequence_with_invalid_san() {
//             let mut hexchess = Hexchess::init();
//             let result = hexchess.apply_sequence("whoops");

//             assert_eq!(result, Err("invalid san at index 0: whoops".to_string()));
//         }

//         #[test]
//         fn test_apply_sequence_with_illegal_move() {
//             let mut hexchess = Hexchess::init();
//             let result = hexchess.apply_sequence("g4g5 a6a5");

//             assert_eq!(result, Err("illegal move at index 1: a6a5".to_string()));
//         }
//     }

//     #[test]
//     fn test_current_moves() {
//         let hexchess = Hexchess::init();
//         let result = hexchess.current_moves().iter().map(|s| s.to_string()).collect::<Vec<String>>();

//         assert_eq!(result.len(), 51);
//         assert_eq!(result[0], "f5f6");
//         assert_eq!(result[1], "e4e5");
//         assert_eq!(result[2], "e4e6");
//         assert_eq!(result[3], "g4g5");
//         assert_eq!(result[4], "g4g6");
//         assert_eq!(result[5], "d3d4");
//         assert_eq!(result[6], "d3d5");
//         assert_eq!(result[7], "f3h2");
//         assert_eq!(result[8], "f3d2");
//         assert_eq!(result[9], "h3h4");
//         assert_eq!(result[10], "h3h5");
//         assert_eq!(result[11], "c2c3");
//         assert_eq!(result[12], "c2c4");
//         assert_eq!(result[13], "f2g3");
//         assert_eq!(result[14], "f2h4");
//         assert_eq!(result[15], "f2i5");
//         assert_eq!(result[16], "f2k6");
//         assert_eq!(result[17], "f2e3");
//         assert_eq!(result[18], "f2d4");
//         assert_eq!(result[19], "f2c5");
//         assert_eq!(result[20], "f2b6");
//         assert_eq!(result[21], "i2i3");
//         assert_eq!(result[22], "i2i4");
//         assert_eq!(result[23], "b1b2");
//         assert_eq!(result[24], "b1b3");
//         assert_eq!(result[25], "c1d2");
//         assert_eq!(result[26], "c1e3");
//         assert_eq!(result[27], "c1f4");
//         assert_eq!(result[28], "d1f4");
//         assert_eq!(result[29], "d1g2");
//         assert_eq!(result[30], "d1b2");
//         assert_eq!(result[31], "d1c3");
//         assert_eq!(result[32], "e1e2");
//         assert_eq!(result[33], "e1e3");
//         assert_eq!(result[34], "e1d2");
//         assert_eq!(result[35], "e1c3");
//         assert_eq!(result[36], "e1b4");
//         assert_eq!(result[37], "e1a5");
//         assert_eq!(result[38], "f1g2");
//         assert_eq!(result[39], "f1e2");
//         assert_eq!(result[40], "g1g2");
//         assert_eq!(result[41], "g1h2");
//         assert_eq!(result[42], "h1i3");
//         assert_eq!(result[43], "h1k2");
//         assert_eq!(result[44], "h1e2");
//         assert_eq!(result[45], "h1f4");
//         assert_eq!(result[46], "i1h2");
//         assert_eq!(result[47], "i1g3");
//         assert_eq!(result[48], "i1f4");
//         assert_eq!(result[49], "k1k2");
//         assert_eq!(result[50], "k1k3");
//     }

//     #[test]
//     fn find_kings_by_color() {
//         let hexchess = Hexchess::init();

//         assert_eq!(hexchess.find_king(Color::Black), Some(h!("g10")));
//         assert_eq!(hexchess.find_king(Color::White), Some(h!("g1")));
//     }

//     #[test]
//     fn test_get() {
//         let hexchess = Hexchess::init();

//         assert_eq!(hexchess.get("g10"), Some(Piece::BlackKing));
//         assert_eq!(hexchess.get("g1"), Some(Piece::WhiteKing));
//         assert_eq!(hexchess.get("a4"), None);
//         assert_eq!(hexchess.get("whoops"), None);
//     }

//     #[test]
//     fn get_color() {
//         let hexchess = Hexchess::init();
//         let results = hexchess.get_color(Color::Black);

//         assert_eq!(results.len(), 18);
//         assert_eq!(results[0], h!("f11"));
//         assert_eq!(results[1], h!("e10"));
//         assert_eq!(results[2], h!("f10"));
//         assert_eq!(results[3], h!("g10"));
//         assert_eq!(results[4], h!("d9"));
//         assert_eq!(results[5], h!("f9"));
//         assert_eq!(results[6], h!("h9"));
//         assert_eq!(results[7], h!("c8"));
//         assert_eq!(results[8], h!("i8"));
//         assert_eq!(results[9], h!("b7"));
//         assert_eq!(results[10], h!("c7"));
//         assert_eq!(results[11], h!("d7"));
//         assert_eq!(results[12], h!("e7"));
//         assert_eq!(results[13], h!("f7"));
//         assert_eq!(results[14], h!("g7"));
//         assert_eq!(results[15], h!("h7"));
//         assert_eq!(results[16], h!("i7"));
//         assert_eq!(results[17], h!("k7"));
//     }

//     mod is_legal {
//         use super::*;

//         #[test]
//         fn legal_move() {
//             let hexchess = Hexchess::init();

//             assert_eq!(hexchess.is_legal(&s!("g4g5")), true);
//         }

//         #[test]
//         fn illegal_move() {
//             let hexchess = Hexchess::init();

//             assert_eq!(hexchess.is_legal(&s!("b1b4")), false);
//         }

//         #[test]
//         fn illegal_move_out_of_turn() {
//             let mut hexchess = Hexchess::init();

//             assert_eq!(hexchess.is_legal(&s!("g7g6")), false);

//             hexchess.turn = Color::Black;

//             assert_eq!(hexchess.is_legal(&s!("g7g6")), true);
//         }

//         #[test]
//         fn white_cannot_promote_on_blacks_positions() {
//             let hexchess = Hexchess::parse("1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr w - 0 1").unwrap();

//             let b1b2 = San { from: h!("b1"), to: h!("b2"), promotion: None };
//             let b1b2q = San { from: h!("b1"), to: h!("b2"), promotion: Some(PromotionPiece::Queen) };
//             assert_eq!(hexchess.is_legal(&b1b2), true);
//             assert_eq!(hexchess.is_legal(&b1b2q), false);

//             let k1l1 = San { from: h!("k1"), to: h!("l1"), promotion: None };
//             let k1l1q = San { from: h!("k1"), to: h!("l1"), promotion: Some(PromotionPiece::Queen) };
//             assert_eq!(hexchess.is_legal(&k1l1), true);
//             assert_eq!(hexchess.is_legal(&k1l1q), false);
//         }

//         #[test]
//         fn black_cannot_promote_on_whites_positions() {
//             let hexchess = Hexchess::parse("1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr b - 0 1").unwrap();

//             let b7a6 = San { from: h!("b7"), to: h!("a6"), promotion: None };
//             let b7a6q = San { from: h!("b7"), to: h!("a6"), promotion: Some(PromotionPiece::Queen) };
//             assert_eq!(hexchess.is_legal(&b7a6), true);
//             assert_eq!(hexchess.is_legal(&b7a6q), false);

//             let k7l6 = San { from: h!("k7"), to: h!("l6"), promotion: None };
//             let k7l6q = San { from: h!("k7"), to: h!("l6"), promotion: Some(PromotionPiece::Queen) };
//             assert_eq!(hexchess.is_legal(&k7l6), true);
//             assert_eq!(hexchess.is_legal(&k7l6q), false);
//         }

//         #[test]
//         fn pawn_must_promote_on_final_rank() {
//             let mut hexchess = Hexchess::parse("1/1P1/5/7/9/11/11/11/11/5p5/11 w - 0 1").unwrap();

//             assert_eq!(hexchess.is_legal(&s!("f10f11")), false);
//             assert_eq!(hexchess.is_legal(&s!("f10f11q")), true);

//             hexchess.turn = Color::Black;

//             assert_eq!(hexchess.is_legal(&s!("f2f1")), false);
//             assert_eq!(hexchess.is_legal(&s!("f2f1q")), true);
//         }
//     }

//     mod moves_from {
//         use super::*;

//         #[test]
//         fn returns_empty_vector_for_empty_position() {
//             let hexchess = Hexchess::init();

//             assert_eq!(hexchess.moves_from(h!("a4")).len(), 0);
//             assert_eq!(hexchess.moves_from_unsafe(h!("a4")).len(), 0);
//         }
//     }

//     mod parsing {
//         use crate::h;
//         use super::*;

//         #[test]
//         fn empty_state() {
//             let hexchess = Hexchess::new();

//             assert!(hexchess.board.iter().all(|&square| square.is_none()));
//             assert_eq!(hexchess.ep, None);
//             assert_eq!(hexchess.fullmove, 1);
//             assert_eq!(hexchess.halfmove, 0);
//             assert_eq!(hexchess.turn, Color::White);
//         }

//         #[test]
//         fn initial_state() {
//             let hexchess = Hexchess::init();

//             assert!(hexchess.board.iter().eq([
//                 Some(Piece::BlackBishop),
//                 Some(Piece::BlackQueen),
//                 Some(Piece::BlackBishop),
//                 Some(Piece::BlackKing),
//                 Some(Piece::BlackKnight),
//                 None,
//                 Some(Piece::BlackBishop),
//                 None,
//                 Some(Piece::BlackKnight),
//                 Some(Piece::BlackRook),
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 Some(Piece::BlackRook),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 Some(Piece::BlackPawn),
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 Some(Piece::WhiteBishop),
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 None,
//                 None,
//                 None,
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 None,
//                 Some(Piece::WhiteBishop),
//                 None,
//                 None,
//                 Some(Piece::WhitePawn),
//                 None,
//                 None,
//                 None,
//                 Some(Piece::WhitePawn),
//                 Some(Piece::WhiteRook),
//                 Some(Piece::WhiteKnight),
//                 Some(Piece::WhiteQueen),
//                 Some(Piece::WhiteBishop),
//                 Some(Piece::WhiteKing),
//                 Some(Piece::WhiteKnight),
//                 Some(Piece::WhiteRook),
//                 Some(Piece::WhitePawn),
//                 None,
//             ].iter()));

//             assert_eq!(hexchess.turn, Color::White);
//             assert_eq!(hexchess.ep, None);
//             assert_eq!(hexchess.halfmove, 0);
//             assert_eq!(hexchess.fullmove, 1);
//         }

//         #[test]
//         fn empty_string() {
//             let hexchess = Hexchess::parse("");

//             assert!(hexchess.is_err());
//             assert_eq!(
//                 hexchess.unwrap_err(),
//                 "board not found"
//             );
//         }

//         #[test]
//         fn invalid() {
//             let hexchess = Hexchess::parse("whoops");

//             assert!(hexchess.is_err());
//         }

//         #[test]
//         fn turn_color() {
//             let white = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 1").unwrap();

//             assert_eq!(white.turn, Color::White);

//             let black = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 b - 0 1").unwrap();

//             assert_eq!(black.turn, Color::Black);
//         }

//         #[test]
//         fn invalid_turn_color() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 x - 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(
//                 hexchess.unwrap_err(),
//                 "invalid turn color: x"
//             );
//         }

//         #[test]
//         fn missing_turn_color() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11").unwrap();

//             assert_eq!(hexchess.turn, Color::White);
//         }

//         #[test]
//         fn en_passant_black() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w e6 0 1");

//             assert_eq!(hexchess.unwrap().ep, Some(h!("e6")));
//         }

//         #[test]
//         fn en_passant_white() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w g5 0 1");

//             assert_eq!(hexchess.unwrap().ep, Some(h!("g5")));
//         }

//         #[test]
//         fn invalid_en_passant() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w x 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(
//                 hexchess.unwrap_err(),
//                 "invalid en passant position: x"
//             );
//         }

//         #[test]
//         fn missing_en_passant() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w").unwrap();

//             assert_eq!(hexchess.ep, None);
//         }

//         #[test]
//         fn illegal_en_passant() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w a1 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(
//                 hexchess.unwrap_err(),
//                 "illegal en passant position: a1"
//             );
//         }

//         #[test]
//         fn invalid_halfmove() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - x 1");

//             assert!(hexchess.is_err());
//             assert_eq!(
//                 hexchess.unwrap_err(),
//                 "invalid halfmove: x"
//             );
//         }

//         #[test]
//         fn multiple_black_kings() {
//             let hexchess = Hexchess::parse("1/k1k/5/7/9/11/11/11/11/11/11 w - 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(hexchess.unwrap_err(), "multiple black kings");
//         }

//         #[test]
//         fn multiple_white_kings() {
//             let hexchess = Hexchess::parse("1/K1K/5/7/9/11/11/11/11/11/11 w - 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(hexchess.unwrap_err(), "multiple white kings");
//         }

//         #[test]
//         fn invalid_character() {
//             let hexchess = Hexchess::parse("x/3/5/7/9/11/11/11/11/11/11 w - 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(hexchess.unwrap_err(), "invalid character at index 0: x");
//         }

//         #[test]
//         fn board_overflow() {
//             let hexchess = Hexchess::parse("2/3/5/7/9/11/11/11/11/11/11 w - 0 1");

//             assert!(hexchess.is_err());
//             assert_eq!(hexchess.unwrap_err(), "board overflow");
//         }

//         #[test]
//         fn missing_halfmove() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w -").unwrap();

//             assert_eq!(hexchess.halfmove, 0);
//         }

//         #[test]
//         fn invalid_fullmove() {
//             let invalid1 = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 x");
//             assert!(invalid1.is_err());
//             assert_eq!(invalid1.unwrap_err(), "invalid fullmove: x");

//             let invalid2 = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0 0");
//             assert!(invalid2.is_err());
//             assert_eq!(invalid2.unwrap_err(), "invalid fullmove: 0");
//         }

//         #[test]
//         fn missing_fullmove() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/11/11/11/11/11 w - 0").unwrap();

//             assert_eq!(hexchess.fullmove, 1);
//         }

//         #[test]
//         fn fen_with_skip_1() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/1p9/11/11/11/11/11 w - 0 1").unwrap();

//             assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("b6")]);
//         }

//         #[test]
//         fn fen_with_skip_2() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/2p8/11/11/11/11/11 w - 0 1").unwrap();

//             assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("c6")]);
//         }

//         #[test]
//         fn fen_with_skip_3() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/3p7/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("d6")]);
//         }

//         #[test]
//         fn fen_with_skip_4() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/4p6/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("e6")]);
//         }

//         #[test]
//         fn fen_with_skip_5() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/5p5/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("f6")]);
//         }

//         #[test]
//         fn fen_with_skip_6() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/6p4/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("g6")]);
//         }

//         #[test]
//         fn fen_with_skip_7() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/7p3/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("h6")]);
//         }

//         #[test]
//         fn fen_with_skip_8() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/8p2/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("i6")]);
//         }

//         #[test]
//         fn fen_with_skip_9() {
//         let hexchess = Hexchess::parse("1/3/5/7/9/9p1/11/11/11/11/11 w - 0 1").unwrap();

//         assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("k6")]);
//         }

//         #[test]
//         fn fen_with_skip_10() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/p10/11/11/11/11/11 w - 0 1").unwrap();

//             assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("a6")]);
//         }

//         #[test]
//         fn fen_with_skip_11() {
//             let hexchess = Hexchess::parse("1/3/5/7/9/11/p10/11/11/11/11 w - 0 1").unwrap();

//             assert_eq!(Some(Piece::BlackPawn), hexchess.board[h!("a5")]);
//         }

//     }

//     mod self_check {
//         use super::*;

//         #[test]
//         fn cannot_step_out_of_a_pin() {
//             let hexchess = Hexchess::parse("1/3/5/7/4K4/5R5/5q5/11/11/11/11 w - 0 1").unwrap();

//             let moves = hexchess.moves_from(h!("f6"));
//             assert_eq!(moves.len(), 1);
//             assert_eq!(moves[0], s!("f6f5"));
//         }

//         // cannot self check on opponent's turn
//         #[test]
//         fn cannot_self_check_on_opponents_turn() {
//             let hexchess    = Hexchess::parse("1/3/5/7/4K4/5R5/5q5/11/11/11/11 b - 0 1").unwrap();
//             let moves = hexchess.moves_from(h!("f6"));

//             assert_eq!(moves.len(), 1);
//             assert_eq!(moves[0], s!("f6f5"));
//         }

//         // king cannot step into check
//         #[test]
//         fn king_cannot_step_into_check() {
//             let hexchess = Hexchess::parse("K/3/2q2/7/9/11/11/11/11/11/11 w - 0 1").unwrap();
//             let moves = hexchess.moves_from(h!("f11"));

//             assert_eq!(moves.len(), 0);
//         }
//     }

//     #[test]
//     fn test_to_piece() {
//         assert_eq!(to_piece('b'), Ok(Piece::BlackBishop));
//         assert_eq!(to_piece('B'), Ok(Piece::WhiteBishop));
//         assert_eq!(to_piece('k'), Ok(Piece::BlackKing)); // <- not called during normal board parsing
//         assert_eq!(to_piece('K'), Ok(Piece::WhiteKing)); // <- not called during normal board parsing
//         assert_eq!(to_piece('n'), Ok(Piece::BlackKnight));
//         assert_eq!(to_piece('N'), Ok(Piece::WhiteKnight));
//         assert_eq!(to_piece('p'), Ok(Piece::BlackPawn));
//         assert_eq!(to_piece('P'), Ok(Piece::WhitePawn));
//         assert_eq!(to_piece('q'), Ok(Piece::BlackQueen));
//         assert_eq!(to_piece('Q'), Ok(Piece::WhiteQueen));
//         assert_eq!(to_piece('r'), Ok(Piece::BlackRook));
//         assert_eq!(to_piece('R'), Ok(Piece::WhiteRook));
//     }

//     #[test]
//     fn test_to_piece_invalid() {
//         assert_eq!(to_piece('x'), Err("invalid_piece_character"));
//         assert_eq!(to_piece('1'), Err("invalid_piece_character"));
//         assert_eq!(to_piece('/'), Err("invalid_piece_character"));
//         assert_eq!(to_piece(' '), Err("invalid_piece_character"));
//     }

//     mod to_string {
//         use super::*;

//         #[test]
//         fn empty_position() {
//             let hexchess = Hexchess::new();

//             assert_eq!(hexchess.to_string(), "1/3/5/7/9/11/11/11/11/11/11 w - 0 1");
//         }

//         #[test]
//         fn initial_position() {
//             let hexchess = Hexchess::init();

//             assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1");
//         }

//         #[test]
//         fn with_en_passant() {
//             let mut hexchess = Hexchess::init();

//             let _ = hexchess.apply_move(&s!("g4g6"));

//             assert_eq!(hexchess.to_string(), "b/qbk/n1b1n/r5r/ppppppppp/6P4/5P5/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 1");
//         }
//     }
// }
