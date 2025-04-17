import { expect, test } from 'vitest'
import { index } from '../dist'
import { applyMove, initHexchess } from '../dist/wasm?init'

test('applyMove', () => {
  const hexchess = applyMove(initHexchess(), {
    from: index('g4'),
    to: index('g5'),
    promotion: null,
  })

  expect(hexchess.board[index('g5')]).toBe('P')
})
