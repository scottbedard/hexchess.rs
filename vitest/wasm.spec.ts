import { expect, test } from 'vitest'
import { index } from '../dist'

import {
  applyMove,
  initHexchess,
  isCheck,
  isCheckmate,
  isStalemate
} from '../dist/wasm?init'

test('applyMove', () => {
  const hexchess = applyMove(initHexchess(), {
    from: index('g4'),
    to: index('g5'),
    promotion: null,
  })

  expect(hexchess.board[index('g5')]).toBe('P')
})

test('isCheck', () => {
  const hexchess = initHexchess()

  expect(isCheck(hexchess)).toBe(false)
})

test('isCheckmate', () => {
  const hexchess = initHexchess()

  expect(isCheckmate(hexchess)).toBe(false)
})

test('isStalemate', () => {
  const hexchess = initHexchess()

  expect(isStalemate(hexchess)).toBe(false)
})
