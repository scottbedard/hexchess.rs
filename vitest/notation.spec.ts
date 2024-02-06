import {
  applyNotation,
  parseHexchess,
  parseNotation,
} from '@bedard/hexchess'

import { describe, expect, it } from 'vitest'

describe('apply', () => {
  it('executes turn notation', () => {
    let hexchess = parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1')

    hexchess = applyNotation(hexchess, parseNotation('g4g5'))

    expect(hexchess.board.g5).toBe('P')
  })
})
