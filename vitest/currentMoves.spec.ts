import { expect, test } from 'vitest'
import { currentMoves, initHexchess } from '../pkg/hexchess'

test('currentMoves', () => {
  const sans = currentMoves(initHexchess())
  
  expect(sans.length).toBe(51)
  expect(sans[0]).toEqual({ from: 41, promotion: null, to: 30 })
})