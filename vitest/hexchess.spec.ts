import { describe, expect, test } from 'vitest'
import { initialPosition, Hexchess, San } from '../pkg'

describe('Hexchess', () => {
  describe('applyMove', () => {
    test('san', () => {
      const san = San.parse('g4g5')
      const hexchess = Hexchess.init().applyMove(san)
      expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1')
    })

    test('string', () => {
      const hexchess = Hexchess.init().applyMove('g4g5')
      expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1')
    })
  })

  test('applySequence', () => {
    const hexchess = Hexchess.init().applySequence('g4g6 f7g6 f5f7 g6f6')

    expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3')
  })

  test('constructor', () => {
    const hexchess = new Hexchess()

    expect(hexchess).toEqual({
      board: [
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null, null,
        null,
      ],
      ep: null,
      fullmove: 1,
      halfmove: 0,
      turn: 'w',
    })
  })

  test('currentMoves', () => {
    const sans = Hexchess.init().currentMoves().map(String)

    expect(sans.length).toBe(51)
    expect(sans[0]).toBe('f5f6')
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

  describe('movesFrom', () => {
    test('index', () => {
      const sans = Hexchess.init().movesFrom(53).map(String)

      expect(sans.length).toBe(2)
      expect(sans[0]).toBe('g4g5')
      expect(sans[1]).toBe('g4g6')
    })

    test('string', () => {
      const sans = Hexchess.init().movesFrom('g4').map(String)

      expect(sans.length).toBe(2)
      expect(sans[0]).toBe('g4g5')
      expect(sans[1]).toBe('g4g6')
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
