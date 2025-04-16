import type { positions } from './constants'

/** board state in fen order */
export type Board = [
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null,
  Piece | null
]

/** piece color */
export type Color = 'b' | 'w'

/** step direction */
export type Direction = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11

/** piece character */
export type Piece = 'p' | 'r' | 'n' | 'b' | 'q' | 'k' | 'P' | 'R' | 'N' | 'B' | 'Q' | 'K'

/** promotion piece character */
export type PromotionPiece = 'q' | 'r' | 'b' | 'n'

/** position names */
export type Position = typeof positions[number]

