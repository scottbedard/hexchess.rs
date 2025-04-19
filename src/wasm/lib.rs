pub mod constants;
pub mod hexchess;
pub mod macros;

use constants::Piece;
use hexchess::hexchess::{Hexchess};
use hexchess::san::San;
use hexchess::utils::index;
use serde::Serialize;
use wasm_bindgen::prelude::*;

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Apply a whitespace separated sequence of move to `Hexchess` object.
#[wasm_bindgen(js_name = apply)]
pub fn apply(hexchess: Hexchess, sequence: String) -> Hexchess {
    set_panic_hook();

    let mut clone = hexchess.clone();

    match clone.apply(sequence.as_str()) {
        Ok(_) => clone,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Apply `San` object to a `Hexchess` object.
#[wasm_bindgen(js_name = applyMove)]
pub fn apply_move(hexchess: Hexchess, san: San) -> Hexchess {
    set_panic_hook();

    let mut clone = hexchess.clone();

    match clone.apply_move(&san) {
        Ok(_) => clone,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Create a blank `Hexchess` object.
#[wasm_bindgen(js_name = createHexchess)]
pub fn create_hexchess() -> Hexchess {
    set_panic_hook();

    Hexchess::new()
}

/// Get current legal moves.
#[wasm_bindgen(js_name = currentMoves)]
pub fn current_moves(hexchess: Hexchess) -> Vec<San> {
    set_panic_hook();

    hexchess.current_moves()
}

/// Create `Hexchess` object at the initial position.
#[wasm_bindgen(js_name = initHexchess)]
pub fn init_hexchess() -> Hexchess {
    set_panic_hook();

    Hexchess::init()
}

/// Get piece at position
#[wasm_bindgen(js_name = get)]
pub fn get(hexchess: Hexchess, position: String) -> JsValue {
    set_panic_hook();

    match hexchess.get(position.as_str()) {
        Some(piece) => JsValue::from_str(&piece.to_string()),
        None => JsValue::NULL,
    }
}

/// Test if the board is in check.
#[wasm_bindgen(js_name = isCheck)]
pub fn is_check(hexchess: Hexchess) -> bool {
    set_panic_hook();

    hexchess.is_check()
}


/// Test if the board is in checkmate.
#[wasm_bindgen(js_name = isCheckmate)]
pub fn is_checkmate(hexchess: Hexchess) -> bool {
    set_panic_hook();

    hexchess.is_checkmate()
}

/// Test if the board is in stalemate.
#[wasm_bindgen(js_name = isStalemate)]
pub fn is_stalemate(hexchess: Hexchess) -> bool {
    set_panic_hook();

    hexchess.is_stalemate()
}

/// Get legal moves from a position index.
#[wasm_bindgen(js_name = movesFrom)]
pub fn moves_from(hexchess: Hexchess, position: &str) -> Vec<San> {
    set_panic_hook();

    match index(position) {
        Ok(n) => hexchess.moves_from(n),
        Err(_) => panic!("invalid position: {}", position),
    }
}

/// Get all possible moves, including ones that result in self-check.
#[wasm_bindgen(js_name = movesFromUnsafe)]
pub fn moves_from_unsafe(hexchess: Hexchess, position: &str) -> Vec<San> {
    set_panic_hook();

    match index(position) {
        Ok(n) => hexchess.moves_from_unsafe(n),
        Err(_) => panic!("invalid position: {}", position),
    }
}

/// Parse `Hexchess` object from Forsyth–Edwards Notation.
#[wasm_bindgen(js_name = parseHexchess)]
pub fn parse_hexchess(source: String) -> Hexchess {
    set_panic_hook();

    match Hexchess::parse(source.as_str()) {
        Ok(hexchess) => hexchess,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Parse `San` object from string.
#[wasm_bindgen(js_name = parseSan)]
pub fn parse_san(source: String) -> San {
    set_panic_hook();

    match San::from(source.as_str()) {
        Ok(san) => san,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Convert `Hexchess` object to string using Forsyth-Edwards Notation.
#[wasm_bindgen(js_name = stringifyHexchess)]
pub fn stringify_hexchess(hexchess: Hexchess) -> String {
    set_panic_hook();

    hexchess.to_string()
}

/// Convert `San` object to string.
#[wasm_bindgen(js_name = stringifySan)]
pub fn stringify_san(san: San) -> String {
    set_panic_hook();

    san.to_string()
}
