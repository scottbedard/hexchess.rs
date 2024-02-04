import * as wasm from "./hexchess_bg.wasm";
import { __wbg_set_wasm } from "./hexchess_bg.js";
__wbg_set_wasm(wasm);
export * from "./hexchess_bg.js";
