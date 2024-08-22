import { describe, expect, it } from 'vitest'

import {
  parseHexchess,
  parseNotation,
  stringifyHexchess,
} from '@bedard/hexchess'

describe('serialization', () => {
  it('parse / stringify hexchess', () => {
    const str = 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'

    const hexchess = parseHexchess(str)

    expect(hexchess).toEqual({
      board: {
        a1: null,
        a2: null,
        a3: null,
        a4: null,
        a5: null,
        a6: null,
        b1: 'P',
        b2: null,
        b3: null,
        b4: null,
        b5: null,
        b6: null,
        b7: 'p',
        c1: 'R',
        c2: 'P',
        c3: null,
        c4: null,
        c5: null,
        c6: null,
        c7: 'p',
        c8: 'r',
        d1: 'N',
        d2: null,
        d3: 'P',
        d4: null,
        d5: null,
        d6: null,
        d7: 'p',
        d8: null,
        d9: 'n',
        e1: 'Q',
        e2: null,
        e3: null,
        e4: 'P',
        e5: null,
        e6: null,
        e7: 'p',
        e8: null,
        e9: null,
        e10: 'q',
        f1: 'B',
        f2: 'B',
        f3: 'B',
        f4: null,
        f5: 'P',
        f6: null,
        f7: 'p',
        f8: null,
        f9: 'b',
        f10: 'b',
        f11: 'b',
        g1: 'K',
        g2: null,
        g3: null,
        g4: 'P',
        g5: null,
        g6: null,
        g7: 'p',
        g8: null,
        g9: null,
        g10: 'k',
        h1: 'N',
        h2: null,
        h3: 'P',
        h4: null,
        h5: null,
        h6: null,
        h7: 'p',
        h8: null,
        h9: 'n',
        i1: 'R',
        i2: 'P',
        i3: null,
        i4: null,
        i5: null,
        i6: null,
        i7: 'p',
        i8: 'r',
        k1: 'P',
        k2: null,
        k3: null,
        k4: null,
        k5: null,
        k6: null,
        k7: 'p',
        l1: null,
        l2: null,
        l3: null,
        l4: null,
        l5: null,
        l6: null
      },
      enPassant: null,
      fullmove: 1,
      halfmove: 0,
      turn: 'w'
    })

    expect(stringifyHexchess(hexchess!)).toBe(str)
  })

  it('parse notation', () => {
    expect(parseNotation('g4g5')).toEqual({
      from: 'g4',
      promotion: null,
      to: 'g5',
    })
  })
})
