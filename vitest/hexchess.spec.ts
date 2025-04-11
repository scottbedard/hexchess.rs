import { describe, expect, test } from 'vitest'
import { initialPosition, Hexchess, San } from '../pkg'

describe('Hexchess', () => {
  describe('apply', () => {
    test('san', () => {
      const san = San.parse('g4g5')
      const hexchess = Hexchess.init().apply(san)
      expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1')
    })

    test('string', () => {
      const hexchess = Hexchess.init().apply('g4g5')
      expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1')
    })

    test('string sequence', () => {
      const hexchess = Hexchess.init().apply('g4g6 f7g6 f5f7 g6f6')

      expect(hexchess.toString()).toEqual('b/qbk/n1b1n/r5r/pppp1pppp/5p5/11/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 3')
    })
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

  describe('movesFromUnsafe', () => {
    test('index', () => {
      const sans = Hexchess.parse('1/3/5/7/4r4/5K5/11/11/11/11/11 w - 0 1')
        .movesFromUnsafe(30)
        .map(String)

      expect(sans).toEqual([
        'f6f7', 'f6g7',
        'f6g6', 'f6h5',
        'f6g5', 'f6g4',
        'f6f5', 'f6e4',
        'f6e5', 'f6d5',
        'f6e6', 'f6e7',
      ])
    })

    test('string', () => {
      const sans = Hexchess.parse('1/3/5/7/4r4/5K5/11/11/11/11/11 w - 0 1')
        .movesFromUnsafe('f6')
        .map(String)

      expect(sans).toEqual([
        'f6f7', 'f6g7',
        'f6g6', 'f6h5',
        'f6g5', 'f6g4',
        'f6f5', 'f6e4',
        'f6e5', 'f6d5',
        'f6e6', 'f6e7',
      ])
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
