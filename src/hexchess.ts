import type {
  Board,
  Color,
  Piece,
  Position
} from './types'

import {
  error,
  getColor,
  index,
  isPosition,
  step
} from './utils'

import { San } from './san'
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

  /** apply legal move */
  applyMove(san: San | string) {
    if (!this.isLegal(san)) {
      error(`illegal move: ${san}`)
    }

    this.applyMoveUnsafe(san)

    return this
  }

  /** apply move, regardless of turn or legality */
  applyMoveUnsafe(san: San | string) {
    const { from, to, promotion } = typeof san === 'string' ? San.from(san) : san
    const piece = this.board[from]

    if (piece === null) {
      error(`cannot apply move from empty position: ${from}`)
    }

    // update halfmove
    if (this.board[to] !== null || piece === 'p' || piece === 'P') {
      this.halfmove = 0
    } else {
      this.halfmove += 1
    }

    // update fullmove and turn color
    const color = getColor(piece)

    if (color === 'b') {
      this.fullmove += 1
      this.turn = 'w'
    } else {
      this.turn = 'b'
    }

    // set from positions
    this.board[from] = null

    // set to position
    if (promotion) {
      this.board[to] = color === 'b' ? promotion : promotion.toUpperCase() as Piece
    } else {
      this.board[to] = piece
    }

    // clear captured en passant
    if (to === this.ep) {
      const captured = piece === 'p'
        ? step(to, 0)
        : piece === 'P'
          ? step(to, 6)
          : null

      if (typeof captured === 'number') {
        this.board[captured] = null
      }
    }

    // set en passsant
    if (piece === 'p') {
      if (from === 17 && to === 38) this.ep = 27 // c7 -> c5, c6
      else if (from === 18 && to === 39) this.ep = 28 // d7 -> d5, d6
      else if (from === 19 && to === 40) this.ep = 29 // e7 -> e5, e6
      else if (from === 20 && to === 41) this.ep = 30 // f7 -> f5, f6
      else if (from === 21 && to === 42) this.ep = 31 // g7 -> g5, g6
      else if (from === 22 && to === 43) this.ep = 32 // h7 -> h5, h6
      else if (from === 23 && to === 44) this.ep = 33 // i7 -> i5, i6
      else if (from === 24 && to === 45) this.ep = 34 // k7 -> k5, k6
      else this.ep = null
    } else if (piece === 'P') {
      if (from === 71 && to === 49) this.ep = 60 // c2 -> c4, c3
      else if (from === 61 && to === 39) this.ep = 50 // d3 -> d5, d4
      else if (from === 51 && to === 29) this.ep = 40 // e4 -> e6, e5
      else if (from === 41 && to === 20) this.ep = 30 // f5 -> f7, f6
      else if (from === 53 && to === 31) this.ep = 42 // g4 -> g6, g5
      else if (from === 65 && to === 43) this.ep = 54 // h3 -> h5, h4
      else if (from === 77 && to === 55) this.ep = 66 // i2 -> i4, i3
      else if (from === 89 && to === 67) this.ep = 78 // k1 -> k3, k2
      else this.ep = null
    } else {
      this.ep = null
    }

    return this
  }

  /** apply a whitespace separated sequence of moves */
  applySequence(sequence: string) {
    const clone = this.clone()

    sequence
      .split(' ')
      .map(str => str.trim())
      .filter(str => str)
      .forEach((part, i) => {
        try {
          const san = San.from(part)

          try {
            clone.applyMoveUnsafe(san)
          } catch {
            error(`illegal move at index ${i}: ${part}`)
          }
        } catch {
          error(`invalid san at index ${i}: ${part}`)
        }
      })

    this.board.splice(0, 91, ...clone.board)
    this.turn = clone.turn
    this.ep = clone.ep
    this.fullmove = clone.fullmove
    this.halfmove = clone.halfmove

    return this
  }

  /** clone hexchess */
  clone() {
    const clone = new Hexchess()
    clone.board.splice(0, 91, ...this.board)
    clone.ep = this.ep
    clone.turn = this.turn
    clone.halfmove = this.halfmove
    clone.fullmove = this.fullmove

    return clone
  }

  /** find king by color */
  findKing(color: Color): number | null {
    const result = this.board.indexOf(color === 'b' ? 'k' : 'K')

    return result >= 0 ? result : null
  }

  /** get piece at position */
  get(position: Position): Piece | null {
    return this.board[index(position)] ?? null
  }

  /** initialize hexchess from starting position */
  static init(): Hexchess {
    return new Hexchess(initialPosition)
  }

  /** test if move is legal */
  isLegal(san: San | string): boolean {
    const { from, promotion, to } = typeof san === 'string' ? San.from(san) : san
    const piece = this.board[from]

    if (!piece) {
      return false
    }

    if (getColor(piece) !== this.turn) {
      return false
    }

    return this.movesFrom(from).some(s => s.from === from && s.to === to && s.promotion === promotion)
  }

  // test if position is threatened
  isThreatened(position: Position | number): boolean {
    const p = typeof position === 'string' ? index(position) : position

    const threatenedPiece = this.board[p]

    if (!threatenedPiece) {
      return false
    }

    const color = getColor(threatenedPiece)

    for (let i = 0; i < 91; i++) {
      const piece = this.board[i]

      if (piece && color !== getColor(piece)) {
        for (const san of this.movesFromUnsafe(i)) {
          if (san.to === p) {
            return true
          }
        }
      }
    }

    return false
  }

  /** get legal moves from a position */
  movesFrom(from: number | Position): San[] {
    const i = typeof from === 'string' ? index(from) : from
    const piece = this.board[i]

    if (!piece) {
      return []
    }

    const color = getColor(piece)

    return this.movesFromUnsafe(i).filter(san => {
      const clone = this.clone().applyMoveUnsafe(san)
      const king = clone.findKing(color)

      return king
        ? !clone.isThreatened(king)
        : true
    })
  }

  /** get moves from a position, regardless of turn or legality */
  movesFromUnsafe(from: number | Position): San[] {
    const i = typeof from === 'string' ? index(from) : from
    const piece = this.board[i]

    if (piece === null) {
      return []
    }

    const color = getColor(piece)

    switch (piece) {
      case 'b':
      case 'B': return straightLineMovesUnsafe(this, i, color, [1, 3, 5, 7, 9, 11])
      case 'k':
      case 'K': return kingMovesUnsafe(this, i, color)
      case 'n':
      case 'N': return knightMovesUnsafe(this, i, color)
      case 'p':
      case 'P': return pawnMovesUnsafe(this, i, color)
      case 'q':
      case 'Q': return straightLineMovesUnsafe(this, i, color, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11])
      case 'r':
      case 'R': return straightLineMovesUnsafe(this, i, color, [0, 2, 4, 6, 8, 10])
    }
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
