import type {
  Board,
  Piece,
  Position
} from './types'

import type { San } from './san'

import {
  error,
  index,
  isPosition
} from './utils'

import { initialPosition } from './constants'
import { kingMovesUnsafe } from './pieces/king'
import { knightMovesUnsafe } from './pieces/knight'
import { pawnMovesUnsafe } from './pieces/pawn'
import { straightLineMovesUnsafe } from './pieces/straight-line'

export interface HexchessStruct {
  board: Board
  ep: number | null
  turn: 'w' | 'b'
  halfmove: number
  fullmove: number
}

export class Hexchess implements HexchessStruct {
  board: Board = createBoard()

  /** index eligible for en passant capture */
  ep: number | null = null

  /** current turn color */
  turn: 'w' | 'b' = 'w'

  /** number of moves since last pawn advance or capture */
  halfmove: number = 0

  /**  full moves, starting at 1 and incremented after black moves */
  fullmove: number = 1

  /** create hexchess from fen */
  constructor(fen: string = '') {
    if (!fen) {
      return
    }

    const [
      board,
      turn = 'w',
      ep = '-',
      halfmove = '0',
      fullmove = '1',
    ] = fen
      .split(' ')
      .map(str => str.trim())
      .filter(str => str)

    this.board = parseBoard(board)

    if (turn === 'w' || turn === 'b') {
      this.turn = turn
    } else {
      error('parse failed: invalid turn color')
    }

    if (ep === '-') {
      this.ep = null
    } else if (isPosition(ep)) {
      this.ep = index(ep)
    } else {
      error('parse failed: invalid en passant')
    }

    this.halfmove = Math.max(0, parseInt(halfmove, 10))

    this.fullmove = Math.max(1, parseInt(fullmove, 10))
  }

  get(position: Position): Piece | null {
    return this.board[index(position)] ?? null
  }

  /** initialize hexchess from starting position */
  static init(): Hexchess {
    return new Hexchess(initialPosition)
  }

  /** get legal moves from a position */
  movesFrom(from: number | Position): San[] {
    return this.movesFromUnsafe(from)
  }

  /** get moves from a position, regardless of turn or legality */
  movesFromUnsafe(from: number | Position): San[] {
    const i = typeof from === 'string' ? index(from) : from
    const piece = this.board[i]

    switch (piece) {
      case 'b':
      case 'B': return straightLineMovesUnsafe(this, i, this.turn, [1, 3, 5, 7, 9, 11])
      case 'k':
      case 'K': return kingMovesUnsafe(this, i, this.turn)
      case 'n':
      case 'N': return knightMovesUnsafe(this, i, this.turn)
      case 'p':
      case 'P': return pawnMovesUnsafe(this, i, this.turn)
      case 'q':
      case 'Q': return straightLineMovesUnsafe(this, i, this.turn, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
      case 'r':
      case 'R': return straightLineMovesUnsafe(this, i, this.turn, [0, 2, 4, 6, 8, 10])
    }

    return []
  }

  /** create hexchess from fen */
  static parse(fen: string): Hexchess {
    return new Hexchess(fen)
  }

  /** format hexchess as fen */
  toString(): string {
    return `${stringifyBoard(this.board)} ${this.turn} ${this.ep ?? '-'} ${this.halfmove} ${this.fullmove}`
  }
}

/** create an empty board object */
function createBoard(): Board {
  return new Array(91).fill(null) as Board
}

/** parse the board section of a fen */
function parseBoard(source: string) {
  const board = createBoard()

  let black = false
  let white = false
  let j = 0

  for (let i = 0; i < source.length; i++) {
    const current = source[i]

    switch (current) {
      case '1':
        switch (source[i + 1]) {
          case '0':
            j += 10
            i++
            continue
          case '1':
            j += 11
            i++
            continue
          default:
            j++
            continue
        }
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9':
        j += parseInt(current, 10)
        continue
      case 'K':
        if (white) {
          error('parse failed: multiple white kings')
        }

        white = true
        board[j] = 'K'
        j++
        continue
      case 'k':
        if (black) {
          error('parse failed: multiple black kings')
        }

        black = true
        board[j] = 'k'
        j++
        continue
      case 'b':
      case 'B':
      case 'n':
      case 'N':
      case 'p':
      case 'P':
      case 'Q':
      case 'q':
      case 'r':
      case 'R':
        board[j] = current
        j++
        continue
      case '/':
        continue
    }

    error(`parse failed: invalid piece ${current}`)
  }

  if (j !== 91) {
    error('parse failed: invalid length')
  }

  return board
}

/** format the board section of a fen */
function stringifyBoard(board: Board): string {
  let blank = 0
  let index = 0
  let result = ''

  for (const piece of board) {
    if (piece === null) {
      blank += 1
    } else {
      if (blank > 0) {
        result += blank.toString()
        blank = 0
      }

      result += piece
    }

    if ([0, 3, 8, 15, 24, 35, 46, 57, 68, 79].includes(index)) {
      if (blank > 0) {
        result += blank.toString()
      }

      result += '/'
      blank = 0
    }

    index += 1
  }

  if (blank > 0) {
    result += blank.toString()
  }

  return result
}
