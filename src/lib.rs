pub mod constants;
pub mod hexchess;
pub mod macros;
pub mod structs;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseHexchess)]
pub fn parse_hexchess(source: String) {
    hexchess::notation::parse_board(source);
}
