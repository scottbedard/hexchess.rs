import { expect, test } from 'vitest'
import { positions } from '../dist'
import { applyMove, initHexchess } from '../dist/wasm?init

test('applyMove', () => {
  const hexchess = applyMove(initHexchess(), {
    from: positions.indexOf('g4'),
    to: positions.indexOf('g5'),
    promotion: null,
  })

  expect(hexchess.board[positions.indexOf('g5')]).toBe('P')
})
