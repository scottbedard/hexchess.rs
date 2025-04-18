import { describe, expect, test } from 'vitest'
import { Hexchess, index } from '../../src'

describe('king', () => {
  test('white', () => {
    const result = Hexchess.parse('1/3/5/7/3P5/5K5/11/6p4/11/11/11 w - 0 1').movesFrom('f6')

    expect(result[0]).toEqual({ from: index('f6'), promotion: null, to: index('f7') })
    expect(result[1]).toEqual({ from: index('f6'), promotion: null, to: index('g7') })
    expect(result[2]).toEqual({ from: index('f6'), promotion: null, to: index('g6') })
    expect(result[3]).toEqual({ from: index('f6'), promotion: null, to: index('h5') })
    expect(result[4]).toEqual({ from: index('f6'), promotion: null, to: index('g5') })
    expect(result[5]).toEqual({ from: index('f6'), promotion: null, to: index('g4') }) // <- g4 is hostile
    expect(result[6]).toEqual({ from: index('f6'), promotion: null, to: index('f5') })
    expect(result[7]).toEqual({ from: index('f6'), promotion: null, to: index('e4') })
    expect(result[8]).toEqual({ from: index('f6'), promotion: null, to: index('e5') })
    expect(result[9]).toEqual({ from: index('f6'), promotion: null, to: index('d5') })
    expect(result[10]).toEqual({ from: index('f6'), promotion: null, to: index('e6') })
    // e7 is friendly
  })

  test('black', () => {
    const result = Hexchess.parse('1/3/5/7/3P5/5k5/11/6p4/11/11/11 b - 0 1').movesFrom('f6')

    expect(result[0]).toEqual({ from: index('f6'), promotion: null, to: index('f7') })
    expect(result[1]).toEqual({ from: index('f6'), promotion: null, to: index('g7') })
    expect(result[2]).toEqual({ from: index('f6'), promotion: null, to: index('g6') })
    expect(result[3]).toEqual({ from: index('f6'), promotion: null, to: index('h5') })
    expect(result[4]).toEqual({ from: index('f6'), promotion: null, to: index('g5') })
    // g4 is friendly
    expect(result[5]).toEqual({ from: index('f6'), promotion: null, to: index('f5') })
    expect(result[6]).toEqual({ from: index('f6'), promotion: null, to: index('e4') })
    expect(result[7]).toEqual({ from: index('f6'), promotion: null, to: index('e5') })
    expect(result[8]).toEqual({ from: index('f6'), promotion: null, to: index('d5') })
    expect(result[9]).toEqual({ from: index('f6'), promotion: null, to: index('e6') })
    expect(result[10]).toEqual({ from: index('f6'), promotion: null, to: index('e7') }) // <- e7 is hostile
  })
})
