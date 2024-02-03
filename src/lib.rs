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

/// Execute hexchess notation
#[wasm_bindgen]
pub fn apply(hexchess: Hexchess, notation: Notation) -> Hexchess {
    set_panic_hook();

    let mut output = hexchess.clone();
    
    let _ = output.apply(notation);

    output
}

/// Parse algebraic hexchess notation
#[wasm_bindgen]
pub fn notation(str: String) -> Notation {
    set_panic_hook();

    Notation::from(&str).unwrap()
}

/// Parse hexchess FEN string
#[wasm_bindgen]
pub fn parse(fen: String) -> Hexchess {
    set_panic_hook();

    let hexchess = Hexchess::from(&fen).unwrap();

    hexchess
}

/// Stringify hexchess object
#[wasm_bindgen]
pub fn stringify(hexchess: Hexchess) -> String {
    set_panic_hook();

    hexchess.to_string()
}

/// Find target moves from a position, regardless of turn color
#[wasm_bindgen(skip_typescript)]
pub fn targets(hexchess: Hexchess, position: Position) -> JsValue {
    set_panic_hook();

    let targets = hexchess.targets(position);

    JsValue::from_serde(&targets).unwrap()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export function targets(hexchess: Hexchess, position: Position): Notation[];
"#;