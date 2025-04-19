import { expect, test } from 'vitest'
import {
  index,
  San
} from '../src'

import {
  applyMove,
  createHexchess,
  currentMoves,
  get,
  initHexchess,
  isCheck,
  isCheckmate,
  isStalemate,
  movesFrom,
  movesFromUnsafe,
  parseHexchess,
  parseSan,
  stringifyHexchess,
  stringifySan
} from '../dist/wasm?init'

test('applyMove', () => {
  const hexchess = applyMove(initHexchess(), {
    from: index('g4'),
    to: index('g5'),
    promotion: null,
  })

  expect(hexchess.board[index('g5')]).toBe('P')
})

test('createHexchess', () => {
  const hexchess = createHexchess()

  expect(hexchess).toEqual({
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

test('currentMoves', () => {
  const hexchess = initHexchess()

  expect(currentMoves(hexchess).map(obj => new San(obj).toString())).toEqual([
    'f5f6', 'e4e5', 'e4e6', 'g4g5', 'g4g6',
    'd3d4', 'd3d5', 'f3h2', 'f3d2', 'h3h4',
    'h3h5', 'c2c3', 'c2c4', 'f2g3', 'f2h4',
    'f2i5', 'f2k6', 'f2e3', 'f2d4', 'f2c5',
    'f2b6', 'i2i3', 'i2i4', 'b1b2', 'b1b3',
    'c1d2', 'c1e3', 'c1f4', 'd1f4', 'd1g2',
    'd1b2', 'd1c3', 'e1e2', 'e1e3', 'e1d2',
    'e1c3', 'e1b4', 'e1a5', 'f1g2', 'f1e2',
    'g1g2', 'g1h2', 'h1i3', 'h1k2', 'h1e2',
    'h1f4', 'i1h2', 'i1g3', 'i1f4', 'k1k2',
    'k1k3',
  ])
})

test('initHexchess', () => {
  const hexchess = initHexchess()

  expect(hexchess).toEqual({
    board: [
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
    ],
    ep: null,
    fullmove: 1,
    halfmove: 0,
    turn: 'w',
  })
})

test('get', () => {
  const hexchess = initHexchess()

  expect(get(hexchess, 'g4')).toEqual('P')
  expect(get(hexchess, 'g5')).toEqual(null)
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

test('movesFrom', () => {
  const hexchess = initHexchess()

  expect(movesFrom(hexchess, 'g4')).toEqual([
    { from: 53, promotion: null, to: 42 }, // why are these from 0?
    { from: 53, promotion: null, to: 31 },
  ])
})

test('movesFromUnsafe', () => {
  const hexchess = initHexchess()

  expect(movesFromUnsafe(hexchess, 'a1')).toEqual([])
})

test('parseHexchess', () => {
  const hexchess = initHexchess()
})

test('parseSan', () => {

})

test('stringifyHexchess', () => {

})

test('stringifySan', () => {

})
