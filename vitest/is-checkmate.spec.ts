import {
  isCheckmate,
  parseHexchess,
} from '@bedard/hexchess'

import { describe, expect, it } from 'vitest'

describe('isCheckmate', () => {
  it('yes', () => {
    const hexchess = parseHexchess('K/3/1q1q1/7/9/11/11/11/11/11/11 w - 2 3')

    expect(isCheckmate(hexchess!)).toBe(true)
  })

  it('no', () => {
    const hexchess = parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')

    expect(isCheckmate(hexchess!)).toBe(false)
  })
})
