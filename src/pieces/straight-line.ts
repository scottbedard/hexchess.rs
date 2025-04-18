import { Hexchess } from '../hexchess'
import { San } from '../san'
import { walk } from '../utils'
import type { Color, Direction } from '../types'

export function straightLineMovesUnsafe(
  hexchess: Hexchess,
  from: number,
  color: Color,
  directions: Direction[]
): San[] {
  const result: San[] = []

  for (const direction of directions) {
    const path = walk(hexchess, from, direction, color)

    for (const to of path) {
      result.push(new San({ from, to }))
    }
  }

  return result
}
