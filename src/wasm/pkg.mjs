import * as bg from './hexchess.js'
export * from './hexchess.js'

// ^^^ @delete-header - everything above this line is excluded from

/**
 * Initial game position
 */
export const initialPosition = 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'

/**
 * Position names
 */
export const positions = [
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
  'l1',
]

/**
 * https://github.com/scottbedard/hexchess
 */
export const version = 'x.y.z'

function updateHexchess(hexchess, data = {}) {
  if (data.board) hexchess.board.splice(0, 91, ...data.board)
  if (data.ep) hexchess.ep = data.ep
  if (data.turn) hexchess.turn = data.turn
  if (data.halfmove) hexchess.halfmove = data.halfmove
  if (data.fullmove) hexchess.fullmove = data.fullmove
  return hexchess
}

function updateSan(san, data = {}) {
  if (data.from !== undefined) san.from = data.from
  if (data.promotion !== undefined) san.promotion = data.promotion
  if (data.to !== undefined) san.to = data.to
  return san
}

export class Hexchess {
  constructor() {
    this.board = new Array(91).fill(null)
    this.ep = null
    this.turn = 'w'
    this.halfmove = 0
    this.fullmove = 1
  }

  apply(source) {
    return this.applySequence(typeof source === 'object' ? bg.stringifySan(source) : source)
  }

  applySequence(source) {
    return updateHexchess(this, bg.applySequence(this, source))
  }

  currentMoves() {
    return bg.currentMoves(this).map(obj => new San(obj))
  }

  static init() {
    return updateHexchess(new Hexchess, bg.initHexchess())
  }

  movesFrom(from) {
    const data = typeof from === 'string' ? bg.toIndex(from) : from
    return bg.movesFrom(this, data).map(obj => new San(obj))
  }

  movesFromUnsafe(from) {
    const data = typeof from === 'string' ? bg.toIndex(from) : from
    return bg.movesFromUnsafe(this, data).map(obj => new San(obj))
  }

  static parse(source) {
    return updateHexchess(new Hexchess, bg.parseHexchess(source))
  }

  toString() {
    return bg.stringifyHexchess(this)
  }
}

export class San {
  constructor(obj = {}) {
    this.from = obj.from ?? 0
    this.promotion = obj.promotion ?? null
    this.to = obj.to ?? 0
  }

  static parse(source) {
    return updateSan(new San, bg.parseSan(source))
  }

  toString() {
    return bg.stringifySan(this)
  }
}
