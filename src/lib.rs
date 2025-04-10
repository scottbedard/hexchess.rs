pub mod constants;
pub mod hexchess;
pub mod macros;

use hexchess::hexchess::Hexchess;
use hexchess::san::San;
use wasm_bindgen::prelude::*;

// pub fn set_panic_hook() {
//     // When the `console_error_panic_hook` feature is enabled, we can call the
//     // `set_panic_hook` function at least once during initialization, and then
//     // we will get better error messages if our code ever panics.
//     //
//     // For more details see
//     // https://github.com/rustwasm/console_error_panic_hook#readme
//     #[cfg(feature = "console_error_panic_hook")]
//     console_error_panic_hook::set_once();
// }

/// Apply `San` object to a `Hexchess` object.
#[wasm_bindgen(js_name = applyMove)]
pub fn apply_move(hexchess: Hexchess, san: San) -> Hexchess {
    let mut clone = hexchess.clone();

    match clone.apply_move(&san) {
        Ok(_) => clone,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Apply a whitespace separated sequence of move to `Hexchess` object.
#[wasm_bindgen(js_name = applySequence)]
pub fn apply_sequence(hexchess: Hexchess, sequence: String) -> Hexchess {
    let mut clone = hexchess.clone();

    match clone.apply_sequence(sequence.as_str()) {
        Ok(_) => clone,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Create a blank `Hexchess` object.
#[wasm_bindgen(js_name = createHexchess)]
pub fn create_hexchess() -> Hexchess {
    Hexchess::new()
}

/// Get current legal moves.
#[wasm_bindgen(js_name = currentMoves)]
pub fn current_moves(hexchess: Hexchess) -> Vec<San> {
    hexchess.current_moves()
}

/// Create `Hexchess` object at the initial position.
#[wasm_bindgen(js_name = initHexchess)]
pub fn init_hexchess() -> Hexchess {
    Hexchess::init()
}

/// Get legal moves from a position index.
#[wasm_bindgen(js_name = movesFrom)]
pub fn moves_from(hexchess: Hexchess, index: u8) -> Vec<San> {
    hexchess.moves_from(index)
}

/// Parse `Hexchess` object from Forsyth–Edwards Notation.
#[wasm_bindgen(js_name = parseHexchess)]
pub fn parse_hexchess(source: String) -> Hexchess {
    match Hexchess::parse(source.as_str()) {
        Ok(hexchess) => hexchess,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Parse `San` object from string.
#[wasm_bindgen(js_name = parseSan)]
pub fn parse_san(source: String) -> San {
    match San::from(source.as_str()) {
        Ok(san) => san,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Convert `Hexchess` object to string using Forsyth–Edwards Notation.
#[wasm_bindgen(js_name = stringifyHexchess)]
pub fn stringify_hexchess(hexchess: Hexchess) -> String {
    hexchess.to_string()
}

/// Convert `San` object to string.
#[wasm_bindgen(js_name = stringifySan)]
pub fn stringify_san(san: San) -> String {
    san.to_string()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export const initialPosition = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1";

/**
 * Hexchess
 */
export class Hexchess {
  board: BoardArray;
  ep: Position | null;
  turn: Color;
  halfmove: number;
  fullmove: number;

  applySequence(source: string): Hexchess;
  currentMoves(): San[];
  static init(): Hexchess;
  static parse(source: string): Hexchess;
  toString(): string;
}

/**
 * San
 */
export class San {
  from: number;
  to: number;
  promotion: PromotionPiece;
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
  Piece | null,
];

/**
 * Position name
 */
export type Position =
  | "f11"
  | "e10"
  | "f10"
  | "g10"
  | "d9"
  | "e9"
  | "f9"
  | "g9"
  | "h9"
  | "c8"
  | "d8"
  | "e8"
  | "f8"
  | "g8"
  | "h8"
  | "i8"
  | "b7"
  | "c7"
  | "d7"
  | "e7"
  | "f7"
  | "g7"
  | "h7"
  | "i7"
  | "k7"
  | "a6"
  | "b6"
  | "c6"
  | "d6"
  | "e6"
  | "f6"
  | "g6"
  | "h6"
  | "i6"
  | "k6"
  | "l6"
  | "a5"
  | "b5"
  | "c5"
  | "d5"
  | "e5"
  | "f5"
  | "g5"
  | "h5"
  | "i5"
  | "k5"
  | "l5"
  | "a4"
  | "b4"
  | "c4"
  | "d4"
  | "e4"
  | "f4"
  | "g4"
  | "h4"
  | "i4"
  | "k4"
  | "l4"
  | "a3"
  | "b3"
  | "c3"
  | "d3"
  | "e3"
  | "f3"
  | "g3"
  | "h3"
  | "i3"
  | "k3"
  | "l3"
  | "a2"
  | "b2"
  | "c2"
  | "d2"
  | "e2"
  | "f2"
  | "g2"
  | "h2"
  | "i2"
  | "k2"
  | "l2"
  | "a1"
  | "b1"
  | "c1"
  | "d1"
  | "e1"
  | "f1"
  | "g1"
  | "h1"
  | "i1"
  | "k1"
  | "l1";

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
  | 90;
"#;
