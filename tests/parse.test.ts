import { parse } from '@bedard/hexchess'

import { describe, it } from 'vitest'

describe('parse', () => {
  it('initial game state', () => {
    const obj = parse('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')

    console.log ({ obj })
  })
})
