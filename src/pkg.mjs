import * as bg from './hexchess.js'

/**
 * Initial game position
 */
export const initialPosition = 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'

/**
 * https://github.com/scottbedard/hexchess.rs
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

  applyMove(source) {
    const data = typeof source === 'string' ? San.parse(source) : source
    return updateHexchess(this, bg.applyMove(this, data))
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
