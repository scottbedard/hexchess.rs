import { expect, test } from 'vitest'
import { createHexchess } from '../pkg'

test('createHexchess', () => {
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
      null,
    ],
    ep: null,
    fullmove: 1,
    halfmove: 0,
    turn: 'w',
  })
})
