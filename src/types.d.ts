import {
  Color,
  Piece,
  PromotionPiece,
  SanStruct
} from '../pkg'

// @append-types-start

/**
 * Initial game position
 */
export type initialPosition = 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'

/**
 * Position names
 */
export type positions = [
  'f11',
  'e10',
  'f10',
  'g10',
  'd9',
  'e9',
  'f9',
  'g9',
  'h9',
  'c8',
  'd8',
  'e8',
  'f8',
  'g8',
  'h8',
  'i8',
  'b7',
  'c7',
  'd7',
  'e7',
  'f7',
  'g7',
  'h7',
  'i7',
  'k7',
  'a6',
  'b6',
  'c6',
  'd6',
  'e6',
  'f6',
  'g6',
  'h6',
  'i6',
  'k6',
  'l6',
  'a5',
  'b5',
  'c5',
  'd5',
  'e5',
  'f5',
  'g5',
  'h5',
  'i5',
  'k5',
  'l5',
  'a4',
  'b4',
  'c4',
  'd4',
  'e4',
  'f4',
  'g4',
  'h4',
  'i4',
  'k4',
  'l4',
  'a3',
  'b3',
  'c3',
  'd3',
  'e3',
  'f3',
  'g3',
  'h3',
  'i3',
  'k3',
  'l3',
  'a2',
  'b2',
  'c2',
  'd2',
  'e2',
  'f2',
  'g2',
  'h2',
  'i2',
  'k2',
  'l2',
  'a1',
  'b1',
  'c1',
  'd1',
  'e1',
  'f1',
  'g1',
  'h1',
  'i1',
  'k1',
  'l1'
]

/**
 * https://github.com/scottbedard/hexchess
 */
export const version = 'x.y.z'

/**
 * Hexchess
 */
export class Hexchess {
  board: BoardArray

  ep: Position | null

  turn: Color

  halfmove: number

  fullmove: number

  /** Apply a whitespace separated sequence of moves, or a single `San` object. */
  apply(source: San | string): Hexchess

  /** Get all current legal moves. */
  currentMoves(): San[]

  /** Get all legal moves from a position. */
  movesFrom(from: Position | number): San[]

  /** Get all moves from a position, including ones that result in self-check. */
  movesFromUnsafe(from: Position | number): San[]

  /** Create `Hexchess` object at the initial position. */
  static init(): Hexchess

  /** Parse `Hexchess` object from Forsyth-Edwards Notation. */
  static parse(source: string): Hexchess

  /** Convert `Hexchess` object to string using Forsyth-Edwards Notation. */
  toString(): string
}

/**
 * San
 */
export class San {
  from: number
  to: number
  promotion: PromotionPiece | null

  constructor(source: Partial<SanStruct> | undefined)
  static parse(source: string): San
  toString(): string
}

/**
 * Board array
 */
export type BoardArray = [
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

/**
 * Position name
 */
export type Position =
  | 'f11'
  | 'e10'
  | 'f10'
  | 'g10'
  | 'd9'
  | 'e9'
  | 'f9'
  | 'g9'
  | 'h9'
  | 'c8'
  | 'd8'
  | 'e8'
  | 'f8'
  | 'g8'
  | 'h8'
  | 'i8'
  | 'b7'
  | 'c7'
  | 'd7'
  | 'e7'
  | 'f7'
  | 'g7'
  | 'h7'
  | 'i7'
  | 'k7'
  | 'a6'
  | 'b6'
  | 'c6'
  | 'd6'
  | 'e6'
  | 'f6'
  | 'g6'
  | 'h6'
  | 'i6'
  | 'k6'
  | 'l6'
  | 'a5'
  | 'b5'
  | 'c5'
  | 'd5'
  | 'e5'
  | 'f5'
  | 'g5'
  | 'h5'
  | 'i5'
  | 'k5'
  | 'l5'
  | 'a4'
  | 'b4'
  | 'c4'
  | 'd4'
  | 'e4'
  | 'f4'
  | 'g4'
  | 'h4'
  | 'i4'
  | 'k4'
  | 'l4'
  | 'a3'
  | 'b3'
  | 'c3'
  | 'd3'
  | 'e3'
  | 'f3'
  | 'g3'
  | 'h3'
  | 'i3'
  | 'k3'
  | 'l3'
  | 'a2'
  | 'b2'
  | 'c2'
  | 'd2'
  | 'e2'
  | 'f2'
  | 'g2'
  | 'h2'
  | 'i2'
  | 'k2'
  | 'l2'
  | 'a1'
  | 'b1'
  | 'c1'
  | 'd1'
  | 'e1'
  | 'f1'
  | 'g1'
  | 'h1'
  | 'i1'
  | 'k1'
  | 'l1'

/**
 * Position index
 */
export type PositionIndex =
  | 0
  | 1
  | 2
  | 3
  | 4
  | 5
  | 6
  | 7
  | 8
  | 9
  | 10
  | 11
  | 12
  | 13
  | 14
  | 15
  | 16
  | 17
  | 18
  | 19
  | 20
  | 21
  | 22
  | 23
  | 24
  | 25
  | 26
  | 27
  | 28
  | 29
  | 30
  | 31
  | 32
  | 33
  | 34
  | 35
  | 36
  | 37
  | 38
  | 39
  | 40
  | 41
  | 42
  | 43
  | 44
  | 45
  | 46
  | 47
  | 48
  | 49
  | 50
  | 51
  | 52
  | 53
  | 54
  | 55
  | 56
  | 57
  | 58
  | 59
  | 60
  | 61
  | 62
  | 63
  | 64
  | 65
  | 66
  | 67
  | 68
  | 69
  | 70
  | 71
  | 72
  | 73
  | 74
  | 75
  | 76
  | 77
  | 78
  | 79
  | 80
  | 81
  | 82
  | 83
  | 84
  | 85
  | 86
  | 87
  | 88
  | 89
  | 90
