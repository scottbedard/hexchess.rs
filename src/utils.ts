import type {
  Color,
  Piece,
  Position
} from './types'

import { positions } from './constants'

/** throw a hexchess error */
export function error(message: string): never {
  throw new Error(`[hexchess error] ${message}`)
}

/** get the color of a piece */
export function getColor(piece: Piece): Color {
  return piece === 'k' || piece === 'q' || piece === 'r' || piece === 'b' || piece === 'n' || piece === 'p'
    ? 'b'
    : 'w'
}

/** test if string is a position name */
export function isPosition(source: string): source is Position {
  return (positions as readonly string[]).includes(source)
}
