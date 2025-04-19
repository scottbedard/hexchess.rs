import { describe, expect, test } from 'vitest'
import {
  Hexchess,
  index,
  initialPosition,
  positions,
  San
} from '../src'

describe('Hexchess', () => {
  describe('applyMove', () => {
    test('sets to and from positions', () => {
      const hexchess = Hexchess.init()

      hexchess.applyMove('g4g5')
      hexchess.applyMove('e7e6')
      expect(hexchess.get('g5')).toBe('P')
      expect(hexchess.get('g4')).toBe(null)
      expect(hexchess.get('e6')).toBe('p')
      expect(hexchess.get('e7')).toBe(null)
    })

    test('clears en passant capture white', () => {
      const hexchess = Hexchess.parse('b/qbk/n1b1n/r5r/ppppp1ppp/5P5/6p4/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w g6 0 2')

      hexchess.applyMove('f6g6')
      expect(hexchess.get('g5')).toBe(null)
    })

    test('clears en passant capture black', () => {
      const hexchess = Hexchess.parse('b/qbk/n1b1n/r5r/pppp1pppp/5pP4/4PP5/11/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b g5 0 2')

      hexchess.applyMove('f6g5')
      expect(hexchess.get('g6')).toBe(null)
    })

    test('only pawns capture en passant', () => {
      const hexchess = Hexchess.parse('b/qbk/n1b1n/r5r/ppppp1ppp/11/5Pp4/4P1PB3/3P1B1P3/2P5P2/1PRNQBKNRP1 w g6 0 2')

      hexchess.applyMove('h4g6') // <- bishop to en passant
      expect(hexchess.get('g5')).toBe('p')
    })

    test('alternate color back and forth', () => {
      const hexchess = Hexchess.init()
      expect(hexchess.turn).toBe('w')

      hexchess.applyMove('g4g5')
      expect(hexchess.turn).toBe('b')

      hexchess.applyMove('e7e6')
      expect(hexchess.turn).toBe('w')

      hexchess.applyMove('f5f6')
      expect(hexchess.turn).toBe('b')
    })

    test('sets and unsets en passant', () => {
      const hexchess = Hexchess.init()

      hexchess.applyMove('g4g6')
      expect(hexchess.ep).toBe(index('g5'))

      hexchess.applyMove('e7e5')
      expect(hexchess.ep).toBe(index('e6'))

      hexchess.applyMove('b1b2')
      expect(hexchess.ep).toBe(null)
    })

    test('sets halfmove and fullmove', () => {
      const hexchess = Hexchess.init()
      expect(hexchess.halfmove).toBe(0)
      expect(hexchess.fullmove).toBe(1)

      hexchess.applyMove('e4e5')
      expect(hexchess.halfmove).toBe(0)
      expect(hexchess.fullmove).toBe(1)

      hexchess.applyMove('f7f6')
      expect(hexchess.halfmove).toBe(0)
      expect(hexchess.fullmove).toBe(2)

      hexchess.applyMove('f3c6')
      expect(hexchess.halfmove).toBe(1)
      expect(hexchess.fullmove).toBe(2)

      hexchess.applyMove('i8h8')
      expect(hexchess.halfmove).toBe(2)
      expect(hexchess.fullmove).toBe(3)

      hexchess.applyMove('c6e10')
      expect(hexchess.halfmove).toBe(0)
      expect(hexchess.fullmove).toBe(3)
    })

    test('promote white and black pieces', () => {
      const hexchess = Hexchess.parse('1/3/1P1P1/7/1P5P1/11/11/11/11/2p1p1p1p2/11 w - 0 1')

      hexchess.applyMove('c7c8r')
      expect(hexchess.get('c8')).toBe('R')

      hexchess.applyMove('c2c1r')
      expect(hexchess.get('c1')).toBe('r')

      hexchess.applyMove('e9e10b')
      expect(hexchess.get('e10')).toBe('B')

      hexchess.applyMove('e2e1b')
      expect(hexchess.get('e1')).toBe('b')

      hexchess.applyMove('g9g10q')
      expect(hexchess.get('g10')).toBe('Q')

      hexchess.applyMove('g2g1q')
      expect(hexchess.get('g1')).toBe('q')

      hexchess.applyMove('i7i8n')
      expect(hexchess.get('i8')).toBe('N')

      hexchess.applyMove('i2i1n')
      expect(hexchess.get('i1')).toBe('n')
    })

    test('errors on illegal move', () => {
      const hexchess = Hexchess.init()

      expect(() => hexchess.applyMove('a4a5')).toThrow()
    })
  })

  describe('applySequence', () => {
    test('applying a sequence of moves', () => {
      const hexchess = Hexchess.init().applySequence('g4g6 f7g6 f5f7 g6f6')

      expect(hexchess.toString()).toBe('b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3')
    })

    test('apply sequence with invalid san', () => {
      expect(() => Hexchess.init().applySequence('whoops')).toThrow()
    })

    test('apply sequence with illegal move', () => {
      expect(() => Hexchess.init().applySequence('g4g5 a6a5')).toThrow()
    })
  })

  describe('applyUnsafe', () => {
    test('errors on empty positions', () => {
      const hexchess = Hexchess.init()

      expect(() => hexchess.applyMoveUnsafe('a4a5')).toThrow()
    })
  })

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

  test('currentMoves', () => {
    const hexchess = Hexchess.init()
    const result = hexchess.currentMoves().map(san => san.toString())

    expect(result.length).toBe(51)
    expect(result[0]).toBe('f5f6')
    expect(result[1]).toBe('e4e5')
    expect(result[2]).toBe('e4e6')
    expect(result[3]).toBe('g4g5')
    expect(result[4]).toBe('g4g6')
    expect(result[5]).toBe('d3d4')
    expect(result[6]).toBe('d3d5')
    expect(result[7]).toBe('f3h2')
    expect(result[8]).toBe('f3d2')
    expect(result[9]).toBe('h3h4')
    expect(result[10]).toBe('h3h5')
    expect(result[11]).toBe('c2c3')
    expect(result[12]).toBe('c2c4')
    expect(result[13]).toBe('f2g3')
    expect(result[14]).toBe('f2h4')
    expect(result[15]).toBe('f2i5')
    expect(result[16]).toBe('f2k6')
    expect(result[17]).toBe('f2e3')
    expect(result[18]).toBe('f2d4')
    expect(result[19]).toBe('f2c5')
    expect(result[20]).toBe('f2b6')
    expect(result[21]).toBe('i2i3')
    expect(result[22]).toBe('i2i4')
    expect(result[23]).toBe('b1b2')
    expect(result[24]).toBe('b1b3')
    expect(result[25]).toBe('c1d2')
    expect(result[26]).toBe('c1e3')
    expect(result[27]).toBe('c1f4')
    expect(result[28]).toBe('d1f4')
    expect(result[29]).toBe('d1g2')
    expect(result[30]).toBe('d1b2')
    expect(result[31]).toBe('d1c3')
    expect(result[32]).toBe('e1e2')
    expect(result[33]).toBe('e1e3')
    expect(result[34]).toBe('e1d2')
    expect(result[35]).toBe('e1c3')
    expect(result[36]).toBe('e1b4')
    expect(result[37]).toBe('e1a5')
    expect(result[38]).toBe('f1g2')
    expect(result[39]).toBe('f1e2')
    expect(result[40]).toBe('g1g2')
    expect(result[41]).toBe('g1h2')
    expect(result[42]).toBe('h1i3')
    expect(result[43]).toBe('h1k2')
    expect(result[44]).toBe('h1e2')
    expect(result[45]).toBe('h1f4')
    expect(result[46]).toBe('i1h2')
    expect(result[47]).toBe('i1g3')
    expect(result[48]).toBe('i1f4')
    expect(result[49]).toBe('k1k2')
    expect(result[50]).toBe('k1k3')
  })

  test('findKing', () => {
    const hexchess = Hexchess.init()

    expect(hexchess.findKing('b')).toBe(index('g10'))
    expect(hexchess.findKing('w')).toBe(index('g1'))
  })

  test('get', () => {
    const hexchess = Hexchess.init()

    expect(hexchess.get('g10')).toBe('k')
    expect(hexchess.get('g1')).toBe('K')
    expect(hexchess.get('a4')).toBe(null)
    // @ts-expect-error - invalid position
    expect(hexchess.get('whoops')).toBe(null)
  })

  test('getColor', () => {
    const hexchess = Hexchess.init()
    const results = hexchess.getColor('b')

    expect(results.length).toBe(18)
    expect(results[0]).toBe(index('f11'))
    expect(results[1]).toBe(index('e10'))
    expect(results[2]).toBe(index('f10'))
    expect(results[3]).toBe(index('g10'))
    expect(results[4]).toBe(index('d9'))
    expect(results[5]).toBe(index('f9'))
    expect(results[6]).toBe(index('h9'))
    expect(results[7]).toBe(index('c8'))
    expect(results[8]).toBe(index('i8'))
    expect(results[9]).toBe(index('b7'))
    expect(results[10]).toBe(index('c7'))
    expect(results[11]).toBe(index('d7'))
    expect(results[12]).toBe(index('e7'))
    expect(results[13]).toBe(index('f7'))
    expect(results[14]).toBe(index('g7'))
    expect(results[15]).toBe(index('h7'))
    expect(results[16]).toBe(index('i7'))
    expect(results[17]).toBe(index('k7'))
  })

  describe('isLegal', () => {
    test('legal move', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isLegal('g4g5')).toBe(true)
    })

    test('illegal move', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isLegal('b1b4')).toBe(false)
    })

    test('illegal move out of turn', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.isLegal('g7g6')).toBe(false)

      hexchess.turn = 'b'

      expect(hexchess.isLegal('g7g6')).toBe(true)
    })

    test('white cannot promote on black positions', () => {
      const hexchess = Hexchess.parse('1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr w - 0 1')

      expect(hexchess.isLegal(new San({ from: 'b1', to: 'b2' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'b1', to: 'b2', promotion: 'q' }))).toBe(false)

      expect(hexchess.isLegal(new San({ from: 'k1', to: 'l1' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'k1', to: 'l1', promotion: 'q' }))).toBe(false)
    })

    test('black cannot promote on white positions', () => {
      const hexchess = Hexchess.parse('1/3/5/7/p7p/R9R/11/11/11/11/rP7Pr b - 0 1')

      expect(hexchess.isLegal(new San({ from: 'b7', to: 'a6' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'b7', to: 'a6', promotion: 'q' }))).toBe(false)

      expect(hexchess.isLegal(new San({ from: 'k7', to: 'l6' }))).toBe(true)
      expect(hexchess.isLegal(new San({ from: 'k7', to: 'l6', promotion: 'q' }))).toBe(false)
    })

    test('pawn must promote on final rank', () => {
      const hexchess = Hexchess.parse('1/1P1/5/7/9/11/11/11/11/5p5/11 w - 0 1')

      expect(hexchess.isLegal(new San({ from: 'f10', to: 'f11' }))).toBe(false)
      expect(hexchess.isLegal(new San({ from: 'f10', to: 'f11', promotion: 'q' }))).toBe(true)

      hexchess.turn = 'b'

      expect(hexchess.isLegal(new San({ from: 'f2', to: 'f1' }))).toBe(false)
      expect(hexchess.isLegal(new San({ from: 'f2', to: 'f1', promotion: 'q' }))).toBe(true)
    })
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

//     #[test]
//     fn test_get() {
//         let hexchess = Hexchess::init();

//         assert_eq!(hexchess.get("g10"), Some(Piece::BlackKing));
//         assert_eq!(hexchess.get("g1"), Some(Piece::WhiteKing));
//         assert_eq!(hexchess.get("a4"), None);
//         assert_eq!(hexchess.get("whoops"), None);
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
