import { expect, test } from 'vitest'
import { Hexchess, initialPosition } from '../src'

test('parse', () => {
  const hexchess = Hexchess.parse(initialPosition)

  expect(hexchess.board).toEqual([
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
  ])

  expect(hexchess.ep).toBeNull()

  expect(hexchess.turn).toBe('w')

  expect(hexchess.halfmove).toBe(0)

  expect(hexchess.fullmove).toBe(1)
})

