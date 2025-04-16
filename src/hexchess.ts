import type {
  Board,
  Position
} from './types'

import { initialPosition } from './constants'

import {
  error,
  isPosition
} from './utils'

export class Hexchess {
  board: Board = createBoard()

  /** index eligible for en passant capture */
  ep: Position | null = null

  /** current turn color */
  turn: 'w' | 'b' = 'w'

  /** number of moves since last pawn advance or capture */
  halfmove: number = 0

  /**  full moves, starting at 1 and incremented after black moves */
  fullmove: number = 1

  constructor(fen: string) {
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
      this.ep = ep
    } else {
      error('parse failed: invalid en passant')
    }

    this.halfmove = Math.max(0, parseInt(halfmove, 10))

    this.fullmove = Math.max(1, parseInt(fullmove, 10))
  }

  /** initialize hexchess from starting position */
  static init(): Hexchess {
    return new Hexchess(initialPosition)
  }

  /** parse hexchess from fen string */
  static parse(source: string): Hexchess {
    return new Hexchess(source)
  }
}

function createBoard(): Board {
  return new Array(91).fill(null) as Board
}

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
