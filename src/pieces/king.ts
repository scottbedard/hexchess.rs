import { Hexchess } from '../hexchess'
import { isTarget, step } from '../utils'
import { San } from '../san'
import type { Color, Direction } from '../types'

export function kingMovesUnsafe(
  hexchess: Hexchess,
  from: number,
  color: Color
): San[] {
  const result: San[] = []

  for (let i = 0; i < 12; i++) {
    const to = step(from, i as Direction)

    if (isTarget(hexchess, to, color)) {
      result.push(new San({ from, to, promotion: null }))
    }
  }

  return result
}
