import {
  applySequence,
  createHexchessInitial,
  stringifyHexchess,
} from '@bedard/hexchess'

import { describe, expect, it } from 'vitest'

describe('applySequence', () => {
  it('executes a sequence of moves', () => {
    const hexchess = applySequence(createHexchessInitial(), 'g4g6 f7g6 f5f7 g6f6')
    
    expect(stringifyHexchess(hexchess)).toBe('b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3')
  })
})
