import { describe, expect, it } from 'vitest'
import { parseHexchess } from '@bedard/hexchess'

describe('parseHexchess', () => {
  it('parse initial ', () => {
    let hexchess = parseHexchess('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1')

    expect(hexchess?.board.f5).toBe('P')
  })

  it('error returns undefined', () => {
    let hexchess = parseHexchess('whoops')

    expect(hexchess).toBeUndefined()
  })
})