import { expect, test } from 'vitest'
import { movesFrom, initHexchess, stringifySan } from '../pkg/hexchess'

test('movesFrom', () => {
  const sans = movesFrom(initHexchess(), 53).map(stringifySan)

  expect(sans).toEqual(['g4g5', 'g4g6'])
})