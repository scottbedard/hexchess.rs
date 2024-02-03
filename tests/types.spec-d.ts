import { assertType, describe, it } from 'vitest'
import { Hexchess, Notation, notation, parse } from '@bedard/hexchess'

describe('types', () => {
  it('Hexchess', () => {
    assertType<Hexchess>(parse('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1'))
  })

  it('Notation', () => {
    assertType<Notation>(notation('e4e5'))
  })
})