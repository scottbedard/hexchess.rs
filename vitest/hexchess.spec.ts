import { describe, expect, test } from 'vitest'
import { initialPosition, Hexchess } from '../pkg'

describe('Hexchess', () => {
  test('applySequence', () => {
    const hexchess = Hexchess.init().applySequence('g4g6 f7g6 f5f7 g6f6')

    expect(hexchess.toString()).toBe('b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3')
  })

  test('current moves', () => {
    const sans = Hexchess.init().currentMoves()

    expect(sans.length).toBe(51)
    expect(sans[0]).toEqual({ from: 41, promotion: null, to: 30 })
  })

  test('init', () => {
    const hexchess = Hexchess.init()

    expect(hexchess).toEqual({
      board: [
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
      ],
      ep: null,
      fullmove: 1,
      halfmove: 0,
      turn: 'w',
    })
  })

  test('parse', () => {
    const hexchess = Hexchess.parse(initialPosition)

    expect(hexchess).toEqual({
      board: [
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
      ],
      ep: null,
      fullmove: 1,
      halfmove: 0,
      turn: 'w',
    })
  })

  test('toString', () => {
    const hexchess = Hexchess.parse(initialPosition)

    expect(hexchess.toString()).toEqual(initialPosition)
  })
})
