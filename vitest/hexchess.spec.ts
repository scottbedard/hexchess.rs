import { describe, expect, test } from 'vitest'
import { Hexchess, initialPosition } from '../src'

describe('Hexchess', () => {
  test('parse', () => {
    const hexchess = Hexchess.parse(initialPosition)

    expect(hexchess.board).toEqual([
      'b', 'q', 'b', 'k', 'n', null, 'b', null, 'n', 'r',
      null, null, null, null, null, 'r', 'p', 'p', 'p', 'p',
      'p', 'p', 'p', 'p', 'p', null, null, null, null, null,
      null, null, null, null, null, null, null, null, null, null,
      null, 'P', null, null, null, null, null, null, null, null,
      null, 'P', null, 'P', null, null, null, null, null, null,
      null, 'P', null, 'B', null, 'P', null, null, null, null,
      null, 'P', null, null, 'B', null, null, 'P', null, null,
      null, 'P', 'R', 'N', 'Q', 'B', 'K', 'N', 'R', 'P',
      null,
    ])

    expect(hexchess.ep).toBeNull()

    expect(hexchess.turn).toBe('w')

    expect(hexchess.halfmove).toBe(0)

    expect(hexchess.fullmove).toBe(1)
  })

  describe('toString', () => {
    test('empty', () => {
      const hexchess = new Hexchess()

      expect(hexchess.toString()).toBe('1/3/5/7/9/11/11/11/11/11/11 w - 0 1')
    })

    test('initial position', () => {
      const hexchess = Hexchess.init()

      expect(hexchess.toString()).toBe('b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1')
    })
  })
})
