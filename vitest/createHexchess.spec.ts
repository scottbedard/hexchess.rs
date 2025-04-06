import { describe, it, expect } from 'vitest'
import { createHexchess } from '../pkg/hexchess'

describe('createHexchess', () => {
  it('creates a blank hexchess object', () => {
    expect(createHexchess()).toEqual({
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
        null
      ],
      ep: null,
      fullmove: 1,
      halfmove: 0,
      turn: 'w'
    })
  })
})