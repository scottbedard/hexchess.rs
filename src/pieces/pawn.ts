import { getColor, isPromotionPosition, step } from '../utils'
import { Hexchess } from '../hexchess'
import { San } from '../san'
import type { Color, Direction } from '../types'

export function pawnMovesUnsafe(
  hexchess: Hexchess,
  from: number,
  color: Color
): San[] {
  const result: San[] = []

  const [forward, portside, starboard] = color === 'w'
    ? [0, 10, 2] as const
    : [6, 4, 8] as const

  // advance forward one position
  const advance1 = advance(hexchess, from, from, forward)

  if (advance1) {
    pushMoves(result, advance1, color)

    // if starting position, advance forward another position
    if (isStartingPosition(from, color)) {
      const advance2 = advance(hexchess, from, advance1.to, forward)

      if (advance2) {
        result.push(advance2)
      }
    }
  }

  // capture portside
  const capturePortside = capture(hexchess, from, portside, color)

  if (capturePortside) {
    pushMoves(result, capturePortside, color)
  }

  // capture starboard
  const captureStarboard = capture(hexchess, from, starboard, color)

  if (captureStarboard) {
    pushMoves(result, captureStarboard, color)
  }

  return result
}

/** advance forward if possible */
const advance = (
  hexchess: Hexchess,
  start: number,
  from: number,
  forward: Direction
): San | null => {
  // we don't need to verify the step exists, because pawns cannot exist
  // on the final rank without promoting. there will always be one more step.
  const to = step(from, forward)!

  return hexchess.board[to] === null
    ? new San({ from: start, to, promotion: null })
    : null
}

/** capture if possible */
const capture = (
  hexchess: Hexchess,
  from: number,
  direction: Direction,
  friendly: Color
): San | null => {
  const to = step(from, direction)

  if (typeof to !== 'number') {
    return null
  }

  const target = hexchess.board[to]

  if (target) {
    if (getColor(target) !== friendly) {
      return new San({ from, to, promotion: null })
    }
  } else if (hexchess.ep === to && hexchess.turn === friendly) {
    return new San({ from, to, promotion: null })
  }

  return null
}

/** check if position is a starting position */
const isStartingPosition = (position: number, color: Color) => color === 'b'
  ? [
    16, // b7
    17, // c7
    18, // d7
    19, // e7
    20, // f7
    21, // g7
    22, // h7
    23, // i7
    24, // k7
  ].includes(position)
  : [
    81, // b1
    71, // c2
    61, // d3
    51, // e4
    41, // f5
    53, // g4
    65, // h3
    77, // i2
    89, // k1
  ].includes(position)

/** push move and expand promotions */
const pushMoves = (
  arr: San[],
  san: San,
  color: Color
) => {
  if (isPromotionPosition(san.to, color)) {
    arr.push(
      new San({ from: san.from, promotion: 'b', to: san.to }),
      new San({ from: san.from, promotion: 'n', to: san.to }),
      new San({ from: san.from, promotion: 'q', to: san.to }),
      new San({ from: san.from, promotion: 'r', to: san.to })
    )
  } else {
    arr.push(san)
  }
}
