import { expect, test } from 'vitest'
import { initHexchess, initialPosition, stringifyHexchess } from '../pkg'

test('stringifyHexchess', () => {
  const hexchess = initHexchess()

  expect(stringifyHexchess(hexchess)).toEqual(initialPosition)
})
