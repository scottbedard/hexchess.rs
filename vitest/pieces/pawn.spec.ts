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

  test('black blocked friendly', () => {
    const f7 = Hexchess.parse('1/3/5/7/4p4/5p5/11/11/11/11/11 b - 0 1').movesFrom(index('f7'))
    expect(f7.length).toEqual(0)
  })

  test('black blocked friendly double', () => {
    const f7 = Hexchess.parse('1/3/5/7/4p4/11/5P5/11/11/11/11 b - 0 1').movesFrom(index('f7'))
    expect(f7).toEqual([
      { from: index('f7'), promotion: null, to: index('f6') },
    ])
  })

  test('black blocked hostile', () => {
    const f7 = Hexchess.parse('1/3/5/7/4p4/5P5/11/11/11/11/11 b - 0 1').movesFrom(index('f7'))
    expect(f7.length).toEqual(0)
  })

  test('black blocked hostile double', () => {
    const f7 = Hexchess.parse('1/3/5/7/4p4/11/5P5/11/11/11/11 b - 0 1').movesFrom(index('f7'))
    expect(f7).toEqual([
      { from: index('f7'), promotion: null, to: index('f6') },
    ])
  })

  test('white starting pawns', () => {
    const f5 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('f5'))
    expect(f5).toEqual([
      { from: index('f5'), promotion: null, to: index('f6') },
      { from: index('f5'), promotion: null, to: index('f7') },
    ])

    const e4 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('e4'))
    expect(e4).toEqual([
      { from: index('e4'), promotion: null, to: index('e5') },
      { from: index('e4'), promotion: null, to: index('e6') },
    ])

    const g4 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('g4'))
    expect(g4).toEqual([
      { from: index('g4'), promotion: null, to: index('g5') },
      { from: index('g4'), promotion: null, to: index('g6') },
    ])

    const d3 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('d3'))
    expect(d3).toEqual([
      { from: index('d3'), promotion: null, to: index('d4') },
      { from: index('d3'), promotion: null, to: index('d5') },
    ])

    const h3 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('h3'))
    expect(h3).toEqual([
      { from: index('h3'), promotion: null, to: index('h4') },
      { from: index('h3'), promotion: null, to: index('h5') },
    ])

    const c2 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('c2'))
    expect(c2).toEqual([
      { from: index('c2'), promotion: null, to: index('c3') },
      { from: index('c2'), promotion: null, to: index('c4') },
    ])

    const i2 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('i2'))
    expect(i2).toEqual([
      { from: index('i2'), promotion: null, to: index('i3') },
      { from: index('i2'), promotion: null, to: index('i4') },
    ])

    const b1 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('b1'))
    expect(b1).toEqual([
      { from: index('b1'), promotion: null, to: index('b2') },
      { from: index('b1'), promotion: null, to: index('b3') },
    ])

    const k1 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('k1'))
    expect(k1).toEqual([
      { from: index('k1'), promotion: null, to: index('k2') },
      { from: index('k1'), promotion: null, to: index('k3') },
    ])
  })

  test('white blocked friendly', () => {
    const f5 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('f5'))
    expect(f5).toEqual([
      { from: index('f5'), promotion: null, to: index('f6') },
      { from: index('f5'), promotion: null, to: index('f7') },
    ])

    const e4 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('e4'))
    expect(e4).toEqual([
      { from: index('e4'), promotion: null, to: index('e5') },
      { from: index('e4'), promotion: null, to: index('e6') },
    ])

    const g4 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('g4'))
    expect(g4).toEqual([
      { from: index('g4'), promotion: null, to: index('g5') },
      { from: index('g4'), promotion: null, to: index('g6') },
    ])

    const d3 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('d3'))
    expect(d3).toEqual([
      { from: index('d3'), promotion: null, to: index('d4') },
      { from: index('d3'), promotion: null, to: index('d5') },
    ])

    const h3 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('h3'))
    expect(h3).toEqual([
      { from: index('h3'), promotion: null, to: index('h4') },
      { from: index('h3'), promotion: null, to: index('h5') },
    ])

    const c2 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('c2'))
    expect(c2).toEqual([
      { from: index('c2'), promotion: null, to: index('c3') },
      { from: index('c2'), promotion: null, to: index('c4') },
    ])

    const i2 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('i2'))
    expect(i2).toEqual([
      { from: index('i2'), promotion: null, to: index('i3') },
      { from: index('i2'), promotion: null, to: index('i4') },
    ])

    const b1 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('b1'))
    expect(b1).toEqual([
      { from: index('b1'), promotion: null, to: index('b2') },
      { from: index('b1'), promotion: null, to: index('b3') },
    ])

    const k1 = Hexchess.parse('1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1').movesFrom(index('k1'))
    expect(k1).toEqual([
      { from: index('k1'), promotion: null, to: index('k2') },
      { from: index('k1'), promotion: null, to: index('k3') },
    ])
  })

  test('white blocked friendly', () => {
    const result = Hexchess.parse('1/3/5/7/9/5P5/5P5/11/11/11/11 w - 0 1').movesFrom('f5')
    expect(result.length).toEqual(0)
  })

  test('white blocked friendly double', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/11/5P5/11/11/11/11 w - 0 1').movesFrom('f5')
    expect(result).toEqual([
      { from: index('f5'), promotion: null, to: index('f6') },
    ])
  })

  test('white blocked hostile', () => {
    const result = Hexchess.parse('1/3/5/7/9/5p5/5P5/11/11/11/11 w - 0 1').movesFrom('f5')
    expect(result.length).toEqual(0)
  })

  test('white blocked hostile double', () => {
    const result = Hexchess.parse('1/3/5/7/4p4/11/5P5/11/11/11/11 w - 0 1').movesFrom('f5')
    expect(result).toEqual([
      { from: index('f5'), promotion: null, to: index('f6') },
    ])
  })

  test('black capture', () => {
    const result = Hexchess.parse('1/3/5/7/9/5p5/4P1P4/11/11/11/11 b - 0 1').movesFrom('f6')
    expect(result).toEqual([
      { from: index('f6'), promotion: null, to: index('f5') },
      { from: index('f6'), promotion: null, to: index('g5') },
      { from: index('f6'), promotion: null, to: index('e5') },
    ])
  })

  test('black capture blocked', () => {
    const result = Hexchess.parse('1/3/5/7/9/5p5/4p1p4/11/11/11/11 b - 0 1').movesFrom('f6')
    expect(result).toEqual([
      { from: index('f6'), promotion: null, to: index('f5') },
    ])
  })

  test('white capture', () => {
    const result = Hexchess.parse('1/3/5/7/9/4pPp4/11/11/11/11/11 w - 0 1').movesFrom('f6')
    expect(result).toEqual([
      { from: index('f6'), promotion: null, to: index('f7') },
      { from: index('f6'), promotion: null, to: index('e6') },
      { from: index('f6'), promotion: null, to: index('g6') },
    ])
  })

  test('white capture blocked', () => {
    const result = Hexchess.parse('1/3/5/7/9/4PPP4/11/11/11/11/11 w - 0 1').movesFrom('f6')
    expect(result).toEqual([
      { from: index('f6'), promotion: null, to: index('f7') },
    ])
  })

  test('capture near edge of board', () => {
    const hexchess = Hexchess.parse('1/3/5/7/9/11/Rr9/P8Rp/10r/11/11 w - 0 1')

    const a4 = hexchess.movesFrom('a4')
    expect(a4).toEqual([
      { from: index('a4'), promotion: null, to: index('b5') },
    ])

    const l4 = hexchess.movesFrom('l4')
    expect(l4).toEqual([
      { from: index('l4'), promotion: null, to: index('k4') },
    ])
  })

  test('black en passant portside', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/4p6/11/11/11/11/11 b f6 0 1').movesFrom('e6')
    expect(result).toEqual([
      { from: index('e6'), promotion: null, to: index('e5') },
      { from: index('e6'), promotion: null, to: index('f6') },
    ])
  })

  test('black en passant portside out of turn', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/4p6/11/11/11/11/11 w f6 0 1').movesFrom('e6')
    expect(result).toEqual([
      { from: index('e6'), promotion: null, to: index('e5') },
      // f6 is out of turn
    ])
  })

  test('black en passant starboard', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/6p4/11/11/11/11/11 b f6 0 1').movesFrom('g6')
    expect(result).toEqual([
      { from: index('g6'), promotion: null, to: index('g5') },
      { from: index('g6'), promotion: null, to: index('f6') },
    ])
  })

  test('black en passant starboard out of turn', () => {
    const result = Hexchess.parse('1/3/5/7/4P4/6p4/11/11/11/11/11 w f6 0 1').movesFrom('g6')
    expect(result).toEqual([
      { from: index('g6'), promotion: null, to: index('g5') },
      // f6 is out of turn
    ])
  })

  test('black promote forward', () => {
    const result = Hexchess.parse('1/3/5/7/9/11/11/11/11/5p5/11 b - 0 1').movesFrom('f2')
    expect(result).toEqual([
      { from: index('f2'), promotion: 'b', to: index('f1') },
      { from: index('f2'), promotion: 'n', to: index('f1') },
      { from: index('f2'), promotion: 'q', to: index('f1') },
      { from: index('f2'), promotion: 'r', to: index('f1') },
    ])
  })

  test('black promote capture portside', () => {
    const result = Hexchess.parse('1/3/5/7/9/11/11/11/11/5p5/4rrK4 w - 0 1').movesFrom('f2')
    expect(result).toEqual([
      { from: index('f2'), promotion: 'b', to: index('g1') },
      { from: index('f2'), promotion: 'n', to: index('g1') },
      { from: index('f2'), promotion: 'q', to: index('g1') },
      { from: index('f2'), promotion: 'r', to: index('g1') },
    ])
  })

  test('black promote capture starboard', () => {
    const result = Hexchess.parse('1/3/5/7/9/11/11/11/11/5p5/4Krr4 w - 0 1').movesFrom('f2')
    expect(result).toEqual([
      { from: index('f2'), promotion: 'b', to: index('e1') },
      { from: index('f2'), promotion: 'n', to: index('e1') },
      { from: index('f2'), promotion: 'q', to: index('e1') },
      { from: index('f2'), promotion: 'r', to: index('e1') },
    ])
  })

  test('white promote forward', () => {
    const result = Hexchess.parse('1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')
    expect(result).toEqual([
      { from: index('f10'), promotion: 'b', to: index('f11') },
      { from: index('f10'), promotion: 'n', to: index('f11') },
      { from: index('f10'), promotion: 'q', to: index('f11') },
      { from: index('f10'), promotion: 'r', to: index('f11') },
    ])
  })

  test('white promote capture portside', () => {
    const result = Hexchess.parse('R/kPR/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')
    expect(result).toEqual([
      { from: index('f10'), promotion: 'b', to: index('e10') },
      { from: index('f10'), promotion: 'n', to: index('e10') },
      { from: index('f10'), promotion: 'q', to: index('e10') },
      { from: index('f10'), promotion: 'r', to: index('e10') },
    ])
  })

  test('white promote capture starboard', () => {
    const result = Hexchess.parse('R/RPk/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')
    expect(result).toEqual([
      { from: index('f10'), promotion: 'b', to: index('g10') },
      { from: index('f10'), promotion: 'n', to: index('g10') },
      { from: index('f10'), promotion: 'q', to: index('g10') },
      { from: index('f10'), promotion: 'r', to: index('g10') },
    ])
  })
})
