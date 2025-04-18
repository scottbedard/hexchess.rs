import { Hexchess } from '../hexchess'
import { isTarget, step } from '../utils'
import { San } from '../san'
import type { Color } from '../types'

// [diagonal, orthogonal one, orthogonal two]
const targets = [
  [1, 0, 2],
  [3, 2, 4],
  [5, 4, 6],
  [7, 6, 8],
  [9, 8, 10],
  [11, 10, 0],
] as const

/** get unsafe knight moves from position */
export function knightMovesUnsafe(
  hexchess: Hexchess,
  from: number,
  color: Color
): San[] {
  const result: San[] = []

  for (const [diagonal, orthogonal1, orthagonal2] of targets) {
    const intermediate = step(from, diagonal)

    if (typeof intermediate !== 'number') {
      continue
    }

    const first = step(intermediate, orthogonal1)
    const second = step(intermediate, orthagonal2)

    if (isTarget(hexchess, first, color)) {
      result.push(new San({ from, to: first }))
    }

    if (isTarget(hexchess, second, color)) {
      result.push(new San({ from, to: second }))
    }
  }

  return result
}
