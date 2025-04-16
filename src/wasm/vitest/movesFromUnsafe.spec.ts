import { expect, test } from 'vitest'
import { parseHexchess, movesFromUnsafe } from '../pkg'

test('movesFromUnsafe', () => {
  const hexchess = parseHexchess('1/3/5/7/4r4/5K5/11/11/11/11/11 w - 0 1')

  expect(movesFromUnsafe(hexchess, 30)).toEqual([
    { from: 30, promotion: null, to: 20 },
    { from: 30, promotion: null, to: 21 },
    { from: 30, promotion: null, to: 31 },
    { from: 30, promotion: null, to: 43 },
    { from: 30, promotion: null, to: 42 },
    { from: 30, promotion: null, to: 53 },
    { from: 30, promotion: null, to: 41 },
    { from: 30, promotion: null, to: 51 },
    { from: 30, promotion: null, to: 40 },
    { from: 30, promotion: null, to: 39 },
    { from: 30, promotion: null, to: 29 },
    { from: 30, promotion: null, to: 19 },
  ])
})

