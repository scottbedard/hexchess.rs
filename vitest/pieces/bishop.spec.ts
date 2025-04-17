import { describe, expect, test } from 'vitest'
import { Hexchess, index } from '../../src'

describe('bishop', () => {
  test('white bishop', () => {
    const result = Hexchess.parse('1/3/5/1p3P1/9/5B5/11/11/11/11/11 w - 0 1').movesFrom('f6')

    expect(result[0]).toEqual({ from: index('f6'), promotion: null, to: index('g7') })
    // h8 is friendly
    expect(result[1]).toEqual({ from: index('f6'), promotion: null, to: index('h5') })
    expect(result[2]).toEqual({ from: index('f6'), promotion: null, to: index('k4') })
    expect(result[3]).toEqual({ from: index('f6'), promotion: null, to: index('g4') })
    expect(result[4]).toEqual({ from: index('f6'), promotion: null, to: index('h2') })
    expect(result[5]).toEqual({ from: index('f6'), promotion: null, to: index('e4') })
    expect(result[6]).toEqual({ from: index('f6'), promotion: null, to: index('d2') })
    expect(result[7]).toEqual({ from: index('f6'), promotion: null, to: index('d5') })
    expect(result[8]).toEqual({ from: index('f6'), promotion: null, to: index('b4') })
    expect(result[9]).toEqual({ from: index('f6'), promotion: null, to: index('e7') })
    expect(result[10]).toEqual({ from: index('f6'), promotion: null, to: index('d8') }) // <- d8 is hostile
  })

  test('black bishop', () => {
    const result = Hexchess.parse('1/3/5/1p3P1/9/5b5/11/11/11/11/11 b - 0 1').movesFrom('f6')

    expect(result[0]).toEqual({ from: index('f6'), promotion: null, to: index('g7') })
    expect(result[1]).toEqual({ from: index('f6'), promotion: null, to: index('h8') }) // <- h8 is hostile
    expect(result[2]).toEqual({ from: index('f6'), promotion: null, to: index('h5') })
    expect(result[3]).toEqual({ from: index('f6'), promotion: null, to: index('k4') })
    expect(result[4]).toEqual({ from: index('f6'), promotion: null, to: index('g4') })
    expect(result[5]).toEqual({ from: index('f6'), promotion: null, to: index('h2') })
    expect(result[6]).toEqual({ from: index('f6'), promotion: null, to: index('e4') })
    expect(result[7]).toEqual({ from: index('f6'), promotion: null, to: index('d2') })
    expect(result[8]).toEqual({ from: index('f6'), promotion: null, to: index('d5') })
    expect(result[9]).toEqual({ from: index('f6'), promotion: null, to: index('b4') })
    expect(result[10]).toEqual({ from: index('f6'), promotion: null, to: index('e7') })
    // d8 is friendly
  })
})
