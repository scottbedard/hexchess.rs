import type {
  Color,
  Direction,
  Piece,
  Position
} from './types'

import type { Hexchess } from './hexchess'
import { graph, positions } from './constants'

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

/** get the index of a position */
export function index(position: Position): number {
  return positions.indexOf(position)
}

/** test if string is a position name */
export function isPosition(source: string): source is Position {
  return (positions as readonly string[]).includes(source)
}

/** test if position is unnocupied or hostile */
export function isTarget(hexchess: Hexchess, position: number | undefined, color: Color): position is number {
  return typeof position === 'number' && (
    !hexchess.board[position] || getColor(hexchess.board[position]) !== color
  )
}

/** step along the hexboard graph */
export function step(from: number, direction: Direction): number | undefined {
  return graph[from][direction]
}

/** walk along the hexboard graph */
export function walk(hexchess: Hexchess, from: number, direction: Direction, color: Color): number[] {
  const path: number[] = []

  let position = from

  while (true) {
    const next = step(position, direction)

    if (next === undefined) {
      return path // <- end of board
    }

    position = next

    const piece = hexchess.board[position]

    if (piece === null) {
      path.push(position) // <- unoccupied position
      continue
    }

    if (getColor(piece) === color) {
      return path // <- shop short of friendly piece
    }

    path.push(position) // <- and captury enemy piece

    return path
  }
}
