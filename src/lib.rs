pub mod commands;
pub mod constants;
pub mod game;

use crate::game::board::Position;
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::{Color, Piece};
use tsify::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Execute notation on hexchess object
#[wasm_bindgen(js_name = applyNotation)]
pub fn apply_notation(hexchess: Hexchess, notation: Notation) -> Hexchess {
    set_panic_hook();
    let mut output = hexchess.clone();
    let _ = output.apply(notation);
    output
}

/// Execute a sequence of moves
#[wasm_bindgen(js_name = applySequence)]
pub fn apply_sequence(hexchess: Hexchess, sequence: &str) -> JsValue {
    let mut output = hexchess.clone();
    let result = output.apply_sequence(sequence);

    match result {
        Ok(_) => JsValue::from_serde(&output).unwrap(),
        Err(message) => panic!("{}", message),
    }
}

/// Create empty hexchess object
#[wasm_bindgen(js_name = createHexchess)]
pub fn create_hexchess() -> Hexchess {
    set_panic_hook();
    Hexchess::new()
}

/// Create hexchess object with initial position
#[wasm_bindgen(js_name = createHexchessInitial)]
pub fn create_hexchess_initial() -> Hexchess {
    set_panic_hook();
    Hexchess::initial()
}

/// Test if board is in checkmate
#[wasm_bindgen(js_name = findKing, skip_typescript)]
pub fn find_king(hexchess: Hexchess, color: Color) -> JsValue {
    set_panic_hook();
    JsValue::from_serde(&hexchess.find_king(color)).unwrap()
}

/// Find piece color at board position
#[wasm_bindgen(js_name = getPositionColor, skip_typescript)]
pub fn get_position_color(hexchess: Hexchess, position: Position) -> JsValue {
    set_panic_hook();
    JsValue::from_serde(&hexchess.color(position)).unwrap()
}

/// Find target moves from a position, regardless of turn color
#[wasm_bindgen(js_name = getTargets, skip_typescript)]
pub fn get_targets(hexchess: Hexchess, position: Position) -> JsValue {
    set_panic_hook();
    JsValue::from_serde(&hexchess.targets(position)).unwrap()
}

/// Test if board is in checkmate
#[wasm_bindgen(js_name = isCheckmate)]
pub fn is_checkmate(hexchess: Hexchess) -> bool {
    set_panic_hook();
    hexchess.is_checkmate()
}

/// Test if a position is threatened
#[wasm_bindgen(js_name = isThreatened)]
pub fn is_threatened(hexchess: Hexchess, position: Position) -> bool {
    set_panic_hook();
    hexchess.is_threatened(position)
}

/// Create hexchess object from string
#[wasm_bindgen(js_name = parseHexchess)]
pub fn parse_hexchess(fen: String) -> Option<Hexchess> {
    set_panic_hook();
    match Hexchess::from(&fen) {
        Ok(hexchess) => Some(hexchess),
        Err(_) => None
    }
}

/// Create hexchess notation object from string
#[wasm_bindgen(js_name = parseNotation)]
pub fn parse_notation(str: String) -> Option<Notation> {
    set_panic_hook();
    match Notation::from(&str) {
        Ok(notation) => Some(notation),
        Err(_) => None
    }
}

/// Stringify hexchess object
#[wasm_bindgen(js_name = stringifyHexchess)]
pub fn stringify_hexchess(hexchess: Hexchess) -> String {
    set_panic_hook();
    hexchess.to_string()
}

/// Get piece color
#[wasm_bindgen(js_name = getPieceColor)]
pub fn get_piece_color(val: char) -> JsValue {
    set_panic_hook();

    match Piece::from_char(val) {
        Ok(piece) => JsValue::from_serde(&piece.color()).unwrap(),
        Err(_) => JsValue::NULL,
    }
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
* Test if board is in checkmate
* @param {Hexchess} hexchess
* @param {Color} color
* @returns {Position | null}
*/
export function findKing(hexchess: Hexchess, color: Color): Position | null;

/**
* Find piece color at board position
* @param {Hexchess} hexchess
* @param {Position} position
* @returns {Color | null}
*/
export function getPositionColor(hexchess: Hexchess, position: Position): Color | null;

/**
 * Find the legal moves from a position
* @param {Hexchess} hexchess
* @param {Position} position
* @returns {Notation[]}
*/
export function getTargets(hexchess: Hexchess, position: Position): Notation[];
"#;