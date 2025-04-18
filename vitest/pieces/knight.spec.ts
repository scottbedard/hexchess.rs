import { describe, test, expect } from 'vitest'
import { Hexchess, index } from '../../src'
describe('knight', () => {
  test('white', () => {
    const result = Hexchess.parse('1/3/5/2P1p2/9/5N5/11/11/11/11/11 w - 0 1').movesFrom('f6')

    expect(result.length).toEqual(11)
    expect(result[0]).toEqual({ from: index('f6'), promotion: null, to: index('g8') }) // <- g8 is hostile
    expect(result[1]).toEqual({ from: index('f6'), promotion: null, to: index('h7') })
    expect(result[2]).toEqual({ from: index('f6'), promotion: null, to: index('i5') })
    expect(result[3]).toEqual({ from: index('f6'), promotion: null, to: index('i4') })
    expect(result[4]).toEqual({ from: index('f6'), promotion: null, to: index('h3') })
    expect(result[5]).toEqual({ from: index('f6'), promotion: null, to: index('g3') })
    expect(result[6]).toEqual({ from: index('f6'), promotion: null, to: index('e3') })
    expect(result[7]).toEqual({ from: index('f6'), promotion: null, to: index('d3') })
    expect(result[8]).toEqual({ from: index('f6'), promotion: null, to: index('c4') })
    expect(result[9]).toEqual({ from: index('f6'), promotion: null, to: index('c5') })
    expect(result[10]).toEqual({ from: index('f6'), promotion: null, to: index('d7') })
    // <- e8 is friendly
  })

  test('black', () => {
    const result = Hexchess.parse('1/3/5/2P1p2/9/5n5/11/11/11/11/11 b - 0 1').movesFrom('f6')

    expect(result.length).toEqual(11)
    expect(result[0]).toEqual({ from: index('f6'), promotion: null, to: index('h7') })
    expect(result[1]).toEqual({ from: index('f6'), promotion: null, to: index('i5') })
    expect(result[2]).toEqual({ from: index('f6'), promotion: null, to: index('i4') })
    expect(result[3]).toEqual({ from: index('f6'), promotion: null, to: index('h3') })
    expect(result[4]).toEqual({ from: index('f6'), promotion: null, to: index('g3') })
    expect(result[5]).toEqual({ from: index('f6'), promotion: null, to: index('e3') })
    expect(result[6]).toEqual({ from: index('f6'), promotion: null, to: index('d3') })
    expect(result[7]).toEqual({ from: index('f6'), promotion: null, to: index('c4') })
    expect(result[8]).toEqual({ from: index('f6'), promotion: null, to: index('c5') })
    expect(result[9]).toEqual({ from: index('f6'), promotion: null, to: index('d7') })
    expect(result[10]).toEqual({ from: index('f6'), promotion: null, to: index('e8') }) // <- e8 is hostile
  })

  test('near edge of board', () => {
    const result = Hexchess.parse('1/1N1/5/7/9/11/11/11/11/11/11 w - 0 1').movesFrom('f10')

    expect(result.length).toEqual(6)
    expect(result[0]).toEqual({ from: index('f10'), promotion: null, to: index('i8') })
    expect(result[1]).toEqual({ from: index('f10'), promotion: null, to: index('h7') })
    expect(result[2]).toEqual({ from: index('f10'), promotion: null, to: index('g7') })
    expect(result[3]).toEqual({ from: index('f10'), promotion: null, to: index('e7') })
    expect(result[4]).toEqual({ from: index('f10'), promotion: null, to: index('d7') })
    expect(result[5]).toEqual({ from: index('f10'), promotion: null, to: index('c8') })
  })
})
