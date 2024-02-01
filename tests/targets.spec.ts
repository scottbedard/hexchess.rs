import { parse, targets } from '@bedard/hexchess'
import { describe, expect, it } from 'vitest'

// https://github.com/scottbedard/hexchess/tree/f98a8e6e18fcd67f32c38adf086ba22094fcf6ef

describe('targets', () => {
  it('targets', () => {
    const hexchess = parse('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')

    const result = targets(hexchess, 'g4')

    expect(result).toEqual([
      { from: 'g4', promotion: null, to: 'g5' },
      { from: 'g4', promotion: null, to: 'g6' }
    ])
  })
})
