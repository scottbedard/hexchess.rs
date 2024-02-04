pub mod commands;
pub mod constants;
pub mod game;

use crate::game::board::Position;
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Execute notation on hexchess object
#[wasm_bindgen(js_name = applyNotation)]
pub fn apply_notation(hexchess: Hexchess, notation: Notation) -> Hexchess {
    set_panic_hook();
    let mut output = hexchess.clone();
    let _ = output.apply(notation);
    output
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

/// Create hexchess object from string
#[wasm_bindgen(js_name = parseHexchess)]
pub fn parse_hexchess(fen: String) -> Hexchess {
    set_panic_hook();
    Hexchess::from(&fen).unwrap()
}

/// Create hexchess notation object from string
#[wasm_bindgen(js_name = parseNotation)]
pub fn parse_notation(str: String) -> Notation {
    set_panic_hook();
    Notation::from(&str).unwrap()
}

/// Stringify hexchess object
#[wasm_bindgen(js_name = stringifyHexchess)]
pub fn stringify_hexchess(hexchess: Hexchess) -> String {
    set_panic_hook();
    hexchess.to_string()
}

/// Find target moves from a position, regardless of turn color
#[wasm_bindgen(js_name = getTargets, skip_typescript)]
pub fn get_targets(hexchess: Hexchess, position: Position) -> JsValue {
    set_panic_hook();
    JsValue::from_serde(&hexchess.targets(position)).unwrap()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export function getTargets(hexchess: Hexchess, position: Position): Notation[];
"#;