import { describe, expect, test } from 'vitest'
import { Hexchess, index } from '../../src'

describe('pawn', () => {
  test('black starting pawns', () => {
    const b7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom('b7')
    expect(b7).toEqual([
      { from: index('b7'), promotion: null, to: index('b6') },
      { from: index('b7'), promotion: null, to: index('b5') },
    ])

    const c7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('c7'))
    expect(c7).toEqual([
      { from: index('c7'), promotion: null, to: index('c6') },
      { from: index('c7'), promotion: null, to: index('c5') },
    ])

    const d7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('d7'))
    expect(d7).toEqual([
      { from: index('d7'), promotion: null, to: index('d6') },
      { from: index('d7'), promotion: null, to: index('d5') },
    ])

    const e7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('e7'))
    expect(e7).toEqual([
      { from: index('e7'), promotion: null, to: index('e6') },
      { from: index('e7'), promotion: null, to: index('e5') },
    ])

    const f7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('f7'))
    expect(f7).toEqual([
      { from: index('f7'), promotion: null, to: index('f6') },
      { from: index('f7'), promotion: null, to: index('f5') },
    ])

    const g7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('g7'))
    expect(g7).toEqual([
      { from: index('g7'), promotion: null, to: index('g6') },
      { from: index('g7'), promotion: null, to: index('g5') },
    ])

    const h7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('h7'))
    expect(h7).toEqual([
      { from: index('h7'), promotion: null, to: index('h6') },
      { from: index('h7'), promotion: null, to: index('h5') },
    ])

    const i7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('i7'))
    expect(i7).toEqual([
      { from: index('i7'), promotion: null, to: index('i6') },
      { from: index('i7'), promotion: null, to: index('i5') },
    ])

    const k7 = Hexchess.parse('1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1').movesFrom(index('k7'))
    expect(k7).toEqual([
      { from: index('k7'), promotion: null, to: index('k6') },
      { from: index('k7'), promotion: null, to: index('k5') },
    ])
  })

  // #[test]
  // fn black_blocked_friendly() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4p4/5p5/11/11/11/11/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f7"));

  //     assert_eq!(result.len(), 0);
  // }

  // #[test]
  // fn black_blocked_friendly_double() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4p4/11/5p5/11/11/11/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f7"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("f7"), promotion: None, to: h!("f6") });
  // }

  // #[test]
  // fn black_blocked_hostile() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4p4/5P5/11/11/11/11/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f7"));

  //     assert_eq!(result.len(), 0);
  // }

  // #[test]
  // fn black_blocked_hostile_double() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4p4/11/5P5/11/11/11/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f7"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("f7"), promotion: None, to: h!("f6") });
  // }

  // #[test]
  // fn white_starting_pawns() {
  //     let f5: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("f5"));
  //     assert_eq!(f5[0], San { from: h!("f5"), promotion: None, to: h!("f6") });
  //     assert_eq!(f5[1], San { from: h!("f5"), promotion: None, to: h!("f7") });

  //     let e4: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("e4"));
  //     assert_eq!(e4[0], San { from: h!("e4"), promotion: None, to: h!("e5") });
  //     assert_eq!(e4[1], San { from: h!("e4"), promotion: None, to: h!("e6") });

  //     let g4: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("g4"));
  //     assert_eq!(g4[0], San { from: h!("g4"), promotion: None, to: h!("g5") });
  //     assert_eq!(g4[1], San { from: h!("g4"), promotion: None, to: h!("g6") });

  //     let d3: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("d3"));
  //     assert_eq!(d3[0], San { from: h!("d3"), promotion: None, to: h!("d4") });
  //     assert_eq!(d3[1], San { from: h!("d3"), promotion: None, to: h!("d5") });

  //     let h3: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("h3"));
  //     assert_eq!(h3[0], San { from: h!("h3"), promotion: None, to: h!("h4") });
  //     assert_eq!(h3[1], San { from: h!("h3"), promotion: None, to: h!("h5") });

  //     let c2: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("c2"));
  //     assert_eq!(c2[0], San { from: h!("c2"), promotion: None, to: h!("c3") });
  //     assert_eq!(c2[1], San { from: h!("c2"), promotion: None, to: h!("c4") });

  //     let i2: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("i2"));
  //     assert_eq!(i2[0], San { from: h!("i2"), promotion: None, to: h!("i3") });
  //     assert_eq!(i2[1], San { from: h!("i2"), promotion: None, to: h!("i4") });

  //     let b1: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("b1"));
  //     assert_eq!(b1[0], San { from: h!("b1"), promotion: None, to: h!("b2") });
  //     assert_eq!(b1[1], San { from: h!("b1"), promotion: None, to: h!("b3") });

  //     let k1: Vec<San> = Hexchess::parse("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(h!("k1"));
  //     assert_eq!(k1[0], San { from: h!("k1"), promotion: None, to: h!("k2") });
  //     assert_eq!(k1[1], San { from: h!("k1"), promotion: None, to: h!("k3") });
  // }

  // #[test]
  // fn white_blocked_friendly() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/9/5P5/5P5/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f5"));

  //     assert_eq!(result.len(), 0);
  // }

  // #[test]
  // fn white_blocked_friendly_double() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/11/5P5/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f5"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("f5"), promotion: None, to: h!("f6") });
  // }

  // #[test]
  // fn white_blocked_hostile() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/9/5p5/5P5/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f5"));

  //     assert_eq!(result.len(), 0);
  // }

  // #[test]
  // fn white_blocked_hostile_double() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4p4/11/5P5/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f5"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("f5"), promotion: None, to: h!("f6") });
  // }

  // #[test]
  // fn black_capture() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/9/5p5/4P1P4/11/11/11/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f6"));

  //     assert_eq!(result.len(), 3);
  //     assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f5") });
  //     assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("g5") });
  //     assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("e5") });
  // }

  // #[test]
  // fn black_capture_blocked() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/9/5p5/4p1p4/11/11/11/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f6"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f5") });
  // }

  // #[test]
  // fn white_capture() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/9/4pPp4/11/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f6"));

  //     assert_eq!(result.len(), 3);
  //     assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
  //     assert_eq!(result[1], San { from: h!("f6"), promotion: None, to: h!("e6") });
  //     assert_eq!(result[2], San { from: h!("f6"), promotion: None, to: h!("g6") });
  // }

  // #[test]
  // fn white_capture_blocked() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/9/4PPP4/11/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f6"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("f6"), promotion: None, to: h!("f7") });
  // }

  // #[test]
  // fn test_capture_near_edge_of_board() {
  //     let hexchess = Hexchess::parse("1/3/5/7/9/11/Rr9/P8Rp/10r/11/11 w - 0 1").unwrap();

  //     let a4 = hexchess.moves_from(h!("a4"));
  //     assert_eq!(a4.len(), 1);
  //     assert_eq!(a4[0], s!("a4b5"));

  //     let l4 = hexchess.moves_from(h!("l4"));
  //     assert_eq!(l4.len(), 1);
  //     assert_eq!(l4[0], s!("l4k4"));
  // }

  // #[test]
  // fn black_en_passant_portside() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/4p6/11/11/11/11/11 b f6 0 1")
  //         .unwrap()
  //         .moves_from(h!("e6"));

  //     assert_eq!(result.len(), 2);
  //     assert_eq!(result[0], San { from: h!("e6"), promotion: None, to: h!("e5") });
  //     assert_eq!(result[1], San { from: h!("e6"), promotion: None, to: h!("f6") });
  // }

  // #[test]
  // fn black_en_passant_portside_out_of_turn() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/4p6/11/11/11/11/11 w f6 0 1")
  //         .unwrap()
  //         .moves_from(h!("e6"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("e6"), promotion: None, to: h!("e5") });
  //     // f6 is out of turn
  // }

  // #[test]
  // fn black_en_passant_starboard() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/6p4/11/11/11/11/11 b f6 0 1")
  //         .unwrap()
  //         .moves_from(h!("g6"));

  //     assert_eq!(result.len(), 2);
  //     assert_eq!(result[0], San { from: h!("g6"), promotion: None, to: h!("g5") });
  //     assert_eq!(result[1], San { from: h!("g6"), promotion: None, to: h!("f6") });
  // }

  // #[test]
  // fn black_en_passant_starboard_out_of_turn() {
  //     let result: Vec<San> = Hexchess::parse("1/3/5/7/4P4/6p4/11/11/11/11/11 w f6 0 1")
  //         .unwrap()
  //         .moves_from(h!("g6"));

  //     assert_eq!(result.len(), 1);
  //     assert_eq!(result[0], San { from: h!("g6"), promotion: None, to: h!("g5") });
  //     // f6 is out of turn
  // }

  // #[test]
  // fn promote_black_forward() {
  //     let result = Hexchess::parse("1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f2"));

  //     assert_eq!(result.len(), 4);
  //     assert_eq!(result[0], San { from: h!("f2"), promotion: Some(PromotionPiece::Bishop), to: h!("f1") });
  //     assert_eq!(result[1], San { from: h!("f2"), promotion: Some(PromotionPiece::Knight), to: h!("f1") });
  //     assert_eq!(result[2], San { from: h!("f2"), promotion: Some(PromotionPiece::Queen), to: h!("f1") });
  //     assert_eq!(result[3], San { from: h!("f2"), promotion: Some(PromotionPiece::Rook), to: h!("f1") });
  // }

  // #[test]
  // fn promote_black_capture_portside() {
  //     let result = Hexchess::parse("1/3/5/7/9/11/11/11/11/5p5/4rrK4 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f2"));

  //     assert_eq!(result.len(), 4);
  //     assert_eq!(result[0], San { from: h!("f2"), promotion: Some(PromotionPiece::Bishop), to: h!("g1") });
  //     assert_eq!(result[1], San { from: h!("f2"), promotion: Some(PromotionPiece::Knight), to: h!("g1") });
  //     assert_eq!(result[2], San { from: h!("f2"), promotion: Some(PromotionPiece::Queen), to: h!("g1") });
  //     assert_eq!(result[3], San { from: h!("f2"), promotion: Some(PromotionPiece::Rook), to: h!("g1") });
  // }

  // #[test]
  // fn promote_black_capture_starboard() {
  //     let result = Hexchess::parse("1/3/5/7/9/11/11/11/11/5p5/4Krr4 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f2"));

  //     assert_eq!(result.len(), 4);
  //     assert_eq!(result[0], San { from: h!("f2"), promotion: Some(PromotionPiece::Bishop), to: h!("e1") });
  //     assert_eq!(result[1], San { from: h!("f2"), promotion: Some(PromotionPiece::Knight), to: h!("e1") });
  //     assert_eq!(result[2], San { from: h!("f2"), promotion: Some(PromotionPiece::Queen), to: h!("e1") });
  //     assert_eq!(result[3], San { from: h!("f2"), promotion: Some(PromotionPiece::Rook), to: h!("e1") });
  // }

  // #[test]
  // fn promote_white_forward() {
  //     let result = Hexchess::parse("1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f10"));

  //     assert_eq!(result.len(), 4);
  //     assert_eq!(result[0], San { from: h!("f10"), promotion: Some(PromotionPiece::Bishop), to: h!("f11") });
  //     assert_eq!(result[1], San { from: h!("f10"), promotion: Some(PromotionPiece::Knight), to: h!("f11") });
  //     assert_eq!(result[2], San { from: h!("f10"), promotion: Some(PromotionPiece::Queen), to: h!("f11") });
  //     assert_eq!(result[3], San { from: h!("f10"), promotion: Some(PromotionPiece::Rook), to: h!("f11") });
  // }

  // #[test]
  // fn promote_white_capture_portside() {
  //     let result = Hexchess::parse("R/kPR/5/7/9/11/11/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f10"));

  //     assert_eq!(result.len(), 4);
  //     assert_eq!(result[0], San { from: h!("f10"), promotion: Some(PromotionPiece::Bishop), to: h!("e10") });
  //     assert_eq!(result[1], San { from: h!("f10"), promotion: Some(PromotionPiece::Knight), to: h!("e10") });
  //     assert_eq!(result[2], San { from: h!("f10"), promotion: Some(PromotionPiece::Queen), to: h!("e10") });
  //     assert_eq!(result[3], San { from: h!("f10"), promotion: Some(PromotionPiece::Rook), to: h!("e10") });
  // }

  // #[test]
  // fn promote_white_capture_starboard() {
  //     let result = Hexchess::parse("R/RPk/5/7/9/11/11/11/11/11/11 w - 0 1")
  //         .unwrap()
  //         .moves_from(h!("f10"));

  //     assert_eq!(result.len(), 4);
  //     assert_eq!(result[0], San { from: h!("f10"), promotion: Some(PromotionPiece::Bishop), to: h!("g10") });
  //     assert_eq!(result[1], San { from: h!("f10"), promotion: Some(PromotionPiece::Knight), to: h!("g10") });
  //     assert_eq!(result[2], San { from: h!("f10"), promotion: Some(PromotionPiece::Queen), to: h!("g10") });
  //     assert_eq!(result[3], San { from: h!("f10"), promotion: Some(PromotionPiece::Rook), to: h!("g10") });
  // }
})
