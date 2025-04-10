import * as bg from "./hexchess_bg.js";
// @wasm-bindgen

/**
 * Initial game position
 */
export const initialPosition = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1";

/**
 * https://github.com/scottbedard/hexchess.rs
 */
export const version = "x.y.z";

function update(hexchess, data = {}) {
  if (data.board) hexchess.board.splice(0, 91, ...data.board);
  if (data.ep) hexchess.ep = data.ep;
  if (data.turn) hexchess.turn = data.turn;
  if (data.halfmove) hexchess.halfmove = data.halfmove;
  if (data.fullmove) hexchess.fullmove = data.fullmove;
  return hexchess;
}

export class Hexchess {
  constructor() {
    this.board = new Array(91).fill(null);
    this.ep = null;
    this.turn = "w";
    this.halfmove = 0;
    this.fullmove = 1;
  }

  applySequence(source) {
    const data = bg.applySequence(this, source);
    return update(this, data);
  }

  currentMoves() {
    return bg.currentMoves(this);
  }

  static init() {
    return update(new Hexchess, bg.initHexchess());
  }

  static parse(source) {
    return update(new Hexchess, bg.parseHexchess(source));
  }

  toString() {
    return bg.stringifyHexchess(this);
  }
}
