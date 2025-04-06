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

/// Parse `Hexchess` object from Forsyth–Edwards Notation.
#[wasm_bindgen(js_name = parseHexchess)]
pub fn parse_hexchess(source: String) -> Hexchess {
    match Hexchess::parse(source.as_str()) {
        Ok(hexchess) => hexchess,
        Err(err) => panic!("hexchess error: {:?}", err),
    }
}

/// Convert `Hexchess` object to string using Forsyth–Edwards Notation.
#[wasm_bindgen(js_name = stringifyHexchess)]
pub fn stringify_hexchess(hexchess: Hexchess) -> String {
    hexchess.to_string()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
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
]
"#;
