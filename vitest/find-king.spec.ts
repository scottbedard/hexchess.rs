import {
  findKing,
  parseHexchess,
} from '@bedard/hexchess'

import { describe, expect, it } from 'vitest'

describe('isThreatened', () => {
  it('found', () => {
    const hexchess = parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')!
    
    expect(findKing(hexchess, 'w')).toBe('g1')
    expect(findKing(hexchess, 'b')).toBe('g10')
  })

  it('not found', () => {
    const hexchess = parseHexchess('1/3/5/7/9/11/11/11/11/11/11 w - 0 1')!
    
    expect(findKing(hexchess, 'w')).toBe(undefined)
    expect(findKing(hexchess, 'b')).toBe(undefined)
  })
})
