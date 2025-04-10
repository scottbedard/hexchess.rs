import { describe, expect, test } from 'vitest'
import { initialPosition, Hexchess } from '../pkg'

describe.only('Hexchess', () => {
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
})
