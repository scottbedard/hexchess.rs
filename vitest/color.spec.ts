import {
  createHexchessInitial,
  getPieceColor,
  getPositionColor,
} from '@bedard/hexchess'

import { describe, expect, it } from 'vitest'

describe('color', () => {
  it('getColor', () => {
    expect(getPieceColor('p')).toBe('b')
    expect(getPieceColor('P')).toBe('w')
    expect(getPieceColor('?')).toBe(null)
  })

  it('getPositionColor', () => {
    const hexchess = createHexchessInitial()

    expect(getPositionColor(hexchess, 'f5')).toBe('w')
    expect(getPositionColor(hexchess, 'f6')).toBe(null)
    expect(getPositionColor(hexchess, 'f7')).toBe('b')
  })
})
