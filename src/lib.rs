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

#[wasm_bindgen]
pub fn targets(hexchess: Hexchess, position: Position) -> JsValue {
    let targets = hexchess.targets(position);

    JsValue::from_serde(&targets).unwrap()
}


#[wasm_bindgen]
pub fn parse(fen: String) -> Hexchess {
    let hexchess = Hexchess::from(&fen).unwrap();

    hexchess
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export function targets(hexchess: Hexchess, position: Position): Notation[];
"#;