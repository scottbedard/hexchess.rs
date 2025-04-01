pub mod constants;
pub mod hexchess;
pub mod macros;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseHexchess)]
pub fn from(source: String) {
    let _hexchess = hexchess::hexchess::Hexchess::from(&source);
}
