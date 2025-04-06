use crate::h;
use serde::{Deserialize, Serialize};
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

/// Initial game position
pub const INITIAL_POSITION: &str = "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1";

/// This graph represents the positions on a hexboard, and their relationship
/// to one another. Each child in the graph represents a position, with it's
/// neighboring positions listed clockwise starting from the position directly
/// above it.
/// 
/// Think of it like the hands of a clock, with 12 o'clock being index 0...
/// For example, to find the position directly below f6, we'd first go to that
/// position in the fen (index 30), then look at the 6th index of that array,
/// which is 41. The 41st fen index is f5.
pub const HEXBOARD_GRAPH: [[Option<u8>; 12]; 91] = [
    [ 
        /* f11 */
        None, 
        None, 
        None,
        None, 
        Some(h!("g10")),
        Some(h!("g9")),
        Some(h!("f10")),
        Some(h!("e9")),
        Some(h!("e10")),
        None, 
        None, 
        None
    ],
    [ 
        /* e10 */
        None, 
        None, 
        Some(h!("f11")),
        Some(h!("g10")),
        Some(h!("f10")),
        Some(h!("f9")),
        Some(h!("e9")),
        Some(h!("d8")),
        Some(h!("d9")),
        None, 
        None, 
        None
    ],
    [ 
        /* f10 */
        Some(h!("f11")),
        None, 
        Some(h!("g10")),
        Some(h!("h9")),
        Some(h!("g9")),
        Some(h!("g8")),
        Some(h!("f9")),
        Some(h!("e8")),
        Some(h!("e9")),
        Some(h!("d9")),
        Some(h!("e10")),
        None,
    ],
    [ 
        /* g10 */
        None, 
        None, 
        None,
        None, 
        Some(h!("h9")),
        Some(h!("h8")),
        Some(h!("g9")),
        Some(h!("f9")),
        Some(h!("f10")),
        Some(h!("e10")),
        Some(h!("f11")),
        None,
    ],
    [ 
        /* d9 */
        None, 
        None,
        Some(h!("e10")),
        Some(h!("f10")),
        Some(h!("e9")),
        Some(h!("e8")),
        Some(h!("d8")),
        Some(h!("c7")),
        Some(h!("c8")),
        None, 
        None,
        None
    ],
    [ 
        /* e9 */
        Some(h!("e10")),
        Some(h!("f11")),
        Some(h!("f10")),
        Some(h!("g9")),
        Some(h!("f9")),
        Some(h!("f8")),
        Some(h!("e8")),
        Some(h!("d7")),
        Some(h!("d8")),
        Some(h!("c8")),
        Some(h!("d9")), 
        None
    ],
    [ 
        /* f9 */
        Some(h!("f10")),
        Some(h!("g10")),
        Some(h!("g9")),
        Some(h!("h8")),
        Some(h!("g8")),
        Some(h!("g7")),
        Some(h!("f8")),
        Some(h!("e7")),
        Some(h!("e8")),
        Some(h!("d8")),
        Some(h!("e9")),
        Some(h!("e10"))
    ],
    [ 
        /* g9 */
        Some(h!("g10")),
        None, 
        Some(h!("h9")),
        Some(h!("i8")),
        Some(h!("h8")),
        Some(h!("h7")),
        Some(h!("g8")),
        Some(h!("f8")),
        Some(h!("f9")),
        Some(h!("e9")),
        Some(h!("f10")),
        Some(h!("f11"))
    ],
    [ 
        /* h9 */
        None, 
        None, 
        None,
        None, 
        Some(h!("i8")),
        Some(h!("i7")),
        Some(h!("h8")),
        Some(h!("g8")),
        Some(h!("g9")),
        Some(h!("f10")),
        Some(h!("g10")),
        None,
    ],
    [ 
        /* c8 */
        None,
        None,
        Some(h!("d9")),
        Some(h!("e9")),
        Some(h!("d8")),
        Some(h!("d7")),
        Some(h!("c7")),
        Some(h!("b6")),
        Some(h!("b7")),
        None,
        None,
        None
    ],
    [ 
        /* d8 */
        Some(h!("d9")),
        Some(h!("e10")),
        Some(h!("e9")),
        Some(h!("f9")),
        Some(h!("e8")),
        Some(h!("e7")),
        Some(h!("d7")),
        Some(h!("c6")),
        Some(h!("c7")),
        Some(h!("b7")),
        Some(h!("c8")), 
        None
    ],
    [ 
        /* e8 */
        Some(h!("e9")),
        Some(h!("f10")),
        Some(h!("f9")),
        Some(h!("g8")),
        Some(h!("f8")),
        Some(h!("f7")),
        Some(h!("e7")),
        Some(h!("d6")),
        Some(h!("d7")),
        Some(h!("c7")),
        Some(h!("d8")),
        Some(h!("d9"))
    ],
    [ 
        /* f8 */
        Some(h!("f9")),
        Some(h!("g9")),
        Some(h!("g8")),
        Some(h!("h7")),
        Some(h!("g7")),
        Some(h!("g6")),
        Some(h!("f7")),
        Some(h!("e6")),
        Some(h!("e7")),
        Some(h!("d7")),
        Some(h!("e8")),
        Some(h!("e9"))
    ],
    [ 
        /* g8 */
        Some(h!("g9")),
        Some(h!("h9")),
        Some(h!("h8")),
        Some(h!("i7")),
        Some(h!("h7")),
        Some(h!("h6")),
        Some(h!("g7")),
        Some(h!("f7")),
        Some(h!("f8")),
        Some(h!("e8")),
        Some(h!("f9")),
        Some(h!("f10"))
    ],
    [ 
        /* h8 */
        Some(h!("h9")),
        None,
        Some(h!("i8")),
        Some(h!("k7")),
        Some(h!("i7")),
        Some(h!("i6")),
        Some(h!("h7")),
        Some(h!("g7")),
        Some(h!("g8")),
        Some(h!("f9")),
        Some(h!("g9")),
        Some(h!("g10"))
    ],
    [ 
        /* i8 */
        None,
        None,
        None,
        None,
        Some(h!("k7")),
        Some(h!("k6")),
        Some(h!("i7")),
        Some(h!("h7")),
        Some(h!("h8")),
        Some(h!("g9")),
        Some(h!("h9")),
        None,
    ],
    [ 
        /* b7 */
        None,
        None,
        Some(h!("c8")),
        Some(h!("d8")),
        Some(h!("c7")),
        Some(h!("c6")),
        Some(h!("b6")),
        Some(h!("a5")),
        Some(h!("a6")),
        None,
        None,
        None
    ],
    [ 
        /* c7 */
        Some(h!("c8")),
        Some(h!("d9")),
        Some(h!("d8")),
        Some(h!("e8")),
        Some(h!("d7")),
        Some(h!("d6")),
        Some(h!("c6")),
        Some(h!("b5")),
        Some(h!("b6")),
        Some(h!("a6")),
        Some(h!("b7")),
        None,
    ],
    [ 
        /* d7 */
        Some(h!("d8")),
        Some(h!("e9")),
        Some(h!("e8")),
        Some(h!("f8")),
        Some(h!("e7")),
        Some(h!("e6")),
        Some(h!("d6")),
        Some(h!("c5")),
        Some(h!("c6")),
        Some(h!("b6")),
        Some(h!("c7")),
        Some(h!("c8"))
    ],
    [ 
        /* e7 */
        Some(h!("e8")),
        Some(h!("f9")),
        Some(h!("f8")),
        Some(h!("g7")),
        Some(h!("f7")),
        Some(h!("f6")),
        Some(h!("e6")),
        Some(h!("d5")),
        Some(h!("d6")),
        Some(h!("c6")),
        Some(h!("d7")),
        Some(h!("d8"))
    ],
    [ 
        /* f7 */
        Some(h!("f8")),
        Some(h!("g8")),
        Some(h!("g7")),
        Some(h!("h6")),
        Some(h!("g6")),
        Some(h!("g5")),
        Some(h!("f6")),
        Some(h!("e5")),
        Some(h!("e6")),
        Some(h!("d6")),
        Some(h!("e7")),
        Some(h!("e8"))
    ],
    [ 
        /* g7 */
        Some(h!("g8")),
        Some(h!("h8")),
        Some(h!("h7")),
        Some(h!("i6")),
        Some(h!("h6")),
        Some(h!("h5")),
        Some(h!("g6")),
        Some(h!("f6")),
        Some(h!("f7")),
        Some(h!("e7")),
        Some(h!("f8")),
        Some(h!("f9"))
    ],
    [ 
        /* h7 */
        Some(h!("h8")),
        Some(h!("i8")),
        Some(h!("i7")),
        Some(h!("k6")),
        Some(h!("i6")),
        Some(h!("i5")),
        Some(h!("h6")),
        Some(h!("g6")),
        Some(h!("g7")),
        Some(h!("f8")),
        Some(h!("g8")),
        Some(h!("g9"))
    ],
    [ 
        /* i7 */
        Some(h!("i8")),
        None,
        Some(h!("k7")),
        Some(h!("l6")),
        Some(h!("k6")),
        Some(h!("k5")),
        Some(h!("i6")),
        Some(h!("h6")),
        Some(h!("h7")),
        Some(h!("g8")),
        Some(h!("h8")),
        Some(h!("h9"))
    ],
    [ 
        /* k7 */
        None,
        None,
        None,
        None,
        Some(h!("l6")),
        Some(h!("l5")),
        Some(h!("k6")),
        Some(h!("i6")),
        Some(h!("i7")),
        Some(h!("h8")),
        Some(h!("i8")),
        None,
    ],
    [ 
        /* a6 */
        None,
        None,
        Some(h!("b7")),
        Some(h!("c7")),
        Some(h!("b6")),
        Some(h!("b5")),
        Some(h!("a5")),
        None,
        None,
        None,
        None,
        None
    ],
    [ 
        /* b6 */
        Some(h!("b7")),
        Some(h!("c8")),
        Some(h!("c7")),
        Some(h!("d7")),
        Some(h!("c6")),
        Some(h!("c5")),
        Some(h!("b5")),
        Some(h!("a4")),
        Some(h!("a5")),
        None,
        Some(h!("a6")),
        None,
    ],
    [ 
        /* c6 */
        Some(h!("c7")),
        Some(h!("d8")),
        Some(h!("d7")),
        Some(h!("e7")),
        Some(h!("d6")),
        Some(h!("d5")),
        Some(h!("c5")),
        Some(h!("b4")),
        Some(h!("b5")),
        Some(h!("a5")),
        Some(h!("b6")),
        Some(h!("b7"))
    ],
    [ 
        /* d6 */
        Some(h!("d7")),
        Some(h!("e8")),
        Some(h!("e7")),
        Some(h!("f7")),
        Some(h!("e6")),
        Some(h!("e5")),
        Some(h!("d5")),
        Some(h!("c4")),
        Some(h!("c5")),
        Some(h!("b5")),
        Some(h!("c6")),
        Some(h!("c7"))
    ],
    [ 
        /* e6 */
        Some(h!("e7")),
        Some(h!("f8")),
        Some(h!("f7")),
        Some(h!("g6")),
        Some(h!("f6")),
        Some(h!("f5")),
        Some(h!("e5")),
        Some(h!("d4")),
        Some(h!("d5")),
        Some(h!("c5")),
        Some(h!("d6")),
        Some(h!("d7"))
    ],
    [ 
        /* f6 */
        Some(h!("f7")),
        Some(h!("g7")),
        Some(h!("g6")),
        Some(h!("h5")),
        Some(h!("g5")),
        Some(h!("g4")),
        Some(h!("f5")),
        Some(h!("e4")),
        Some(h!("e5")),
        Some(h!("d5")),
        Some(h!("e6")),
        Some(h!("e7"))
    ],
    [ 
        /* g6 */
        Some(h!("g7")),
        Some(h!("h7")),
        Some(h!("h6")),
        Some(h!("i5")),
        Some(h!("h5")),
        Some(h!("h4")),
        Some(h!("g5")),
        Some(h!("f5")),
        Some(h!("f6")),
        Some(h!("e6")),
        Some(h!("f7")),
        Some(h!("f8"))
    ],
    [ 
        /* h6 */
        Some(h!("h7")),
        Some(h!("i7")),
        Some(h!("i6")),
        Some(h!("k5")),
        Some(h!("i5")),
        Some(h!("i4")),
        Some(h!("h5")),
        Some(h!("g5")),
        Some(h!("g6")),
        Some(h!("f7")),
        Some(h!("g7")),
        Some(h!("g8"))
    ],
    [ 
        /* i6 */
        Some(h!("i7")),
        Some(h!("k7")),
        Some(h!("k6")),
        Some(h!("l5")),
        Some(h!("k5")),
        Some(h!("k4")),
        Some(h!("i5")),
        Some(h!("h5")),
        Some(h!("h6")),
        Some(h!("g7")),
        Some(h!("h7")),
        Some(h!("h8"))
    ],
    [ 
        /* k6 */
        Some(h!("k7")),
        None,
        Some(h!("l6")),
        None,
        Some(h!("l5")),
        Some(h!("l4")),
        Some(h!("k5")),
        Some(h!("i5")),
        Some(h!("i6")),
        Some(h!("h7")),
        Some(h!("i7")),
        Some(h!("i8"))
    ],
    [ 
        /* l6 */
        None,
        None,
        None,
        None,
        None,
        None,
        Some(h!("l5")),
        Some(h!("k5")),
        Some(h!("k6")),
        Some(h!("i7")),
        Some(h!("k7")),
        None,
    ],
    [ 
        /* a5 */
        Some(h!("a6")),
        Some(h!("b7")),
        Some(h!("b6")),
        Some(h!("c6")),
        Some(h!("b5")),
        Some(h!("b4")),
        Some(h!("a4")),
        None,
        None,
        None,
        None,
        None
    ],
    [ 
        /* b5 */
        Some(h!("b6")),
        Some(h!("c7")),
        Some(h!("c6")),
        Some(h!("d6")),
        Some(h!("c5")),
        Some(h!("c4")),
        Some(h!("b4")),
        Some(h!("a3")),
        Some(h!("a4")),
        None,
        Some(h!("a5")),
        Some(h!("a6"))
    ],
    [ 
        /* c5 */
        Some(h!("c6")),
        Some(h!("d7")),
        Some(h!("d6")),
        Some(h!("e6")),
        Some(h!("d5")),
        Some(h!("d4")),
        Some(h!("c4")),
        Some(h!("b3")),
        Some(h!("b4")),
        Some(h!("a4")),
        Some(h!("b5")),
        Some(h!("b6"))
    ],
    [ 
        /* d5 */
        Some(h!("d6")),
        Some(h!("e7")),
        Some(h!("e6")),
        Some(h!("f6")),
        Some(h!("e5")),
        Some(h!("e4")),
        Some(h!("d4")),
        Some(h!("c3")),
        Some(h!("c4")),
        Some(h!("b4")),
        Some(h!("c5")),
        Some(h!("c6"))
    ],
    [ 
        /* e5 */
        Some(h!("e6")),
        Some(h!("f7")),
        Some(h!("f6")),
        Some(h!("g5")),
        Some(h!("f5")),
        Some(h!("f4")),
        Some(h!("e4")),
        Some(h!("d3")),
        Some(h!("d4")),
        Some(h!("c4")),
        Some(h!("d5")),
        Some(h!("d6"))
    ],
    [ 
        /* f5 */
        Some(h!("f6")),
        Some(h!("g6")),
        Some(h!("g5")),
        Some(h!("h4")),
        Some(h!("g4")),
        Some(h!("g3")),
        Some(h!("f4")),
        Some(h!("e3")),
        Some(h!("e4")),
        Some(h!("d4")),
        Some(h!("e5")),
        Some(h!("e6"))
    ],
    [ 
        /* g5 */
        Some(h!("g6")),
        Some(h!("h6")),
        Some(h!("h5")),
        Some(h!("i4")),
        Some(h!("h4")),
        Some(h!("h3")),
        Some(h!("g4")),
        Some(h!("f4")),
        Some(h!("f5")),
        Some(h!("e5")),
        Some(h!("f6")),
        Some(h!("f7"))
    ],
    [ 
        /* h5 */
        Some(h!("h6")),
        Some(h!("i6")),
        Some(h!("i5")),
        Some(h!("k4")),
        Some(h!("i4")),
        Some(h!("i3")),
        Some(h!("h4")),
        Some(h!("g4")),
        Some(h!("g5")),
        Some(h!("f6")),
        Some(h!("g6")),
        Some(h!("g7"))
    ],
    [ 
        /* i5 */
        Some(h!("i6")),
        Some(h!("k6")),
        Some(h!("k5")),
        Some(h!("l4")),
        Some(h!("k4")),
        Some(h!("k3")),
        Some(h!("i4")),
        Some(h!("h4")),
        Some(h!("h5")),
        Some(h!("g6")),
        Some(h!("h6")),
        Some(h!("h7"))
    ],
    [ 
        /* k5 */
        Some(h!("k6")),
        Some(h!("l6")),
        Some(h!("l5")),
        None,
        Some(h!("l4")),
        Some(h!("l3")),
        Some(h!("k4")),
        Some(h!("i4")),
        Some(h!("i5")),
        Some(h!("h6")),
        Some(h!("i6")),
        Some(h!("i7"))
    ],
    [ 
        /* l5 */
        Some(h!("l6")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l4")),
        Some(h!("k4")),
        Some(h!("k5")),
        Some(h!("i6")),
        Some(h!("k6")),
        Some(h!("k7"))
    ],
    [ 
        /* a4 */
        Some(h!("a5")),
        Some(h!("b6")),
        Some(h!("b5")),
        Some(h!("c5")),
        Some(h!("b4")),
        Some(h!("b3")),
        Some(h!("a3")),
        None,
        None,
        None,
        None,
        None
    ],
    [ 
        /* b4 */
        Some(h!("b5")),
        Some(h!("c6")),
        Some(h!("c5")),
        Some(h!("d5")),
        Some(h!("c4")),
        Some(h!("c3")),
        Some(h!("b3")),
        Some(h!("a2")),
        Some(h!("a3")),
        None,
        Some(h!("a4")),
        Some(h!("a5"))
    ],
    [ 
        /* c4 */
        Some(h!("c5")),
        Some(h!("d6")),
        Some(h!("d5")),
        Some(h!("e5")),
        Some(h!("d4")),
        Some(h!("d3")),
        Some(h!("c3")),
        Some(h!("b2")),
        Some(h!("b3")),
        Some(h!("a3")),
        Some(h!("b4")),
        Some(h!("b5"))
    ],
    [ 
        /* d4 */
        Some(h!("d5")),
        Some(h!("e6")),
        Some(h!("e5")),
        Some(h!("f5")),
        Some(h!("e4")),
        Some(h!("e3")),
        Some(h!("d3")),
        Some(h!("c2")),
        Some(h!("c3")),
        Some(h!("b3")),
        Some(h!("c4")),
        Some(h!("c5"))
    ],
    [ 
        /* e4 */
        Some(h!("e5")),
        Some(h!("f6")),
        Some(h!("f5")),
        Some(h!("g4")),
        Some(h!("f4")),
        Some(h!("f3")),
        Some(h!("e3")),
        Some(h!("d2")),
        Some(h!("d3")),
        Some(h!("c3")),
        Some(h!("d4")),
        Some(h!("d5"))
    ],
    [ 
        /* f4 */
        Some(h!("f5")),
        Some(h!("g5")),
        Some(h!("g4")),
        Some(h!("h3")),
        Some(h!("g3")),
        Some(h!("g2")),
        Some(h!("f3")),
        Some(h!("e2")),
        Some(h!("e3")),
        Some(h!("d3")),
        Some(h!("e4")),
        Some(h!("e5"))
    ],
    [ 
        /* g4 */
        Some(h!("g5")),
        Some(h!("h5")),
        Some(h!("h4")),
        Some(h!("i3")),
        Some(h!("h3")),
        Some(h!("h2")),
        Some(h!("g3")),
        Some(h!("f3")),
        Some(h!("f4")),
        Some(h!("e4")),
        Some(h!("f5")),
        Some(h!("f6"))
    ],
    [ 
        /* h4 */
        Some(h!("h5")),
        Some(h!("i5")),
        Some(h!("i4")),
        Some(h!("k3")),
        Some(h!("i3")),
        Some(h!("i2")),
        Some(h!("h3")),
        Some(h!("g3")),
        Some(h!("g4")),
        Some(h!("f5")),
        Some(h!("g5")),
        Some(h!("g6"))
    ],
    [ 
        /* i4 */
        Some(h!("i5")),
        Some(h!("k5")),
        Some(h!("k4")),
        Some(h!("l3")),
        Some(h!("k3")),
        Some(h!("k2")),
        Some(h!("i3")),
        Some(h!("h3")),
        Some(h!("h4")),
        Some(h!("g5")),
        Some(h!("h5")),
        Some(h!("h6"))
    ],
    [ 
        /* k4 */
        Some(h!("k5")),
        Some(h!("l5")),
        Some(h!("l4")),
        None,
        Some(h!("l3")),
        Some(h!("l2")),
        Some(h!("k3")),
        Some(h!("i3")),
        Some(h!("i4")),
        Some(h!("h5")),
        Some(h!("i5")),
        Some(h!("i6"))
    ],
    [ 
        /* l4 */
        Some(h!("l5")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l3")),
        Some(h!("k3")),
        Some(h!("k4")),
        Some(h!("i5")),
        Some(h!("k5")),
        Some(h!("k6"))
    ],
    [ 
        /* a3 */
        Some(h!("a4")),
        Some(h!("b5")),
        Some(h!("b4")),
        Some(h!("c4")),
        Some(h!("b3")),
        Some(h!("b2")),
        Some(h!("a2")),
        None,
        None,
        None,
        None,
        None
    ],
    [ 
        /* b3 */
        Some(h!("b4")),
        Some(h!("c5")),
        Some(h!("c4")),
        Some(h!("d4")),
        Some(h!("c3")),
        Some(h!("c2")),
        Some(h!("b2")),
        Some(h!("a1")),
        Some(h!("a2")),
        None,
        Some(h!("a3")),
        Some(h!("a4"))
    ],
    [ 
        /* c3 */
        Some(h!("c4")),
        Some(h!("d5")),
        Some(h!("d4")),
        Some(h!("e4")),
        Some(h!("d3")),
        Some(h!("d2")),
        Some(h!("c2")),
        Some(h!("b1")),
        Some(h!("b2")),
        Some(h!("a2")),
        Some(h!("b3")),
        Some(h!("b4"))
    ],
    [ 
        /* d3 */
        Some(h!("d4")),
        Some(h!("e5")),
        Some(h!("e4")),
        Some(h!("f4")),
        Some(h!("e3")),
        Some(h!("e2")),
        Some(h!("d2")),
        Some(h!("c1")),
        Some(h!("c2")),
        Some(h!("b2")),
        Some(h!("c3")),
        Some(h!("c4"))
    ],
    [ 
        /* e3 */
        Some(h!("e4")),
        Some(h!("f5")),
        Some(h!("f4")),
        Some(h!("g3")),
        Some(h!("f3")),
        Some(h!("f2")),
        Some(h!("e2")),
        Some(h!("d1")),
        Some(h!("d2")),
        Some(h!("c2")),
        Some(h!("d3")),
        Some(h!("d4"))
    ],
    [ 
        /* f3 */
        Some(h!("f4")),
        Some(h!("g4")),
        Some(h!("g3")),
        Some(h!("h2")),
        Some(h!("g2")),
        Some(h!("g1")),
        Some(h!("f2")),
        Some(h!("e1")),
        Some(h!("e2")),
        Some(h!("d2")),
        Some(h!("e3")),
        Some(h!("e4"))
    ],
    [ 
        /* g3 */
        Some(h!("g4")),
        Some(h!("h4")),
        Some(h!("h3")),
        Some(h!("i2")),
        Some(h!("h2")),
        Some(h!("h1")),
        Some(h!("g2")),
        Some(h!("f2")),
        Some(h!("f3")),
        Some(h!("e3")),
        Some(h!("f4")),
        Some(h!("f5"))
    ],
    [ 
        /* h3 */
        Some(h!("h4")),
        Some(h!("i4")),
        Some(h!("i3")),
        Some(h!("k2")),
        Some(h!("i2")),
        Some(h!("i1")),
        Some(h!("h2")),
        Some(h!("g2")),
        Some(h!("g3")),
        Some(h!("f4")),
        Some(h!("g4")),
        Some(h!("g5"))
    ],
    [ 
        /* i3 */
        Some(h!("i4")),
        Some(h!("k4")),
        Some(h!("k3")),
        Some(h!("l2")),
        Some(h!("k2")),
        Some(h!("k1")),
        Some(h!("i2")),
        Some(h!("h2")),
        Some(h!("h3")),
        Some(h!("g4")),
        Some(h!("h4")),
        Some(h!("h5"))
    ],
    [ 
        /* k3 */
        Some(h!("k4")),
        Some(h!("l4")),
        Some(h!("l3")),
        None,
        Some(h!("l2")),
        Some(h!("l1")),
        Some(h!("k2")),
        Some(h!("i2")),
        Some(h!("i3")),
        Some(h!("h4")),
        Some(h!("i4")),
        Some(h!("i5"))
    ],
    [ 
        /* l3 */
        Some(h!("l4")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l2")),
        Some(h!("k2")),
        Some(h!("k3")),
        Some(h!("i4")),
        Some(h!("k4")),
        Some(h!("k5"))
    ],
    [ 
        /* a2 */
        Some(h!("a3")),
        Some(h!("b4")),
        Some(h!("b3")),
        Some(h!("c3")),
        Some(h!("b2")),
        Some(h!("b1")),
        Some(h!("a1")),
        None,
        None,
        None,
        None,
        None
    ],
    [ 
        /* b2 */
        Some(h!("b3")),
        Some(h!("c4")),
        Some(h!("c3")),
        Some(h!("d3")),
        Some(h!("c2")),
        Some(h!("c1")),
        Some(h!("b1")),
        None,
        Some(h!("a1")),
        None,
        Some(h!("a2")),
        Some(h!("a3"))
    ],
    [ 
        /* c2 */
        Some(h!("c3")),
        Some(h!("d4")),
        Some(h!("d3")),
        Some(h!("e3")),
        Some(h!("d2")),
        Some(h!("d1")),
        Some(h!("c1")),
        None,
        Some(h!("b1")),
        Some(h!("a1")),
        Some(h!("b2")),
        Some(h!("b3"))
    ],
    [ 
        /* d2 */
        Some(h!("d3")),
        Some(h!("e4")),
        Some(h!("e3")),
        Some(h!("f3")),
        Some(h!("e2")),
        Some(h!("e1")),
        Some(h!("d1")),
        None,
        Some(h!("c1")),
        Some(h!("b1")),
        Some(h!("c2")),
        Some(h!("c3"))
    ],
    [ 
        /* e2 */
        Some(h!("e3")),
        Some(h!("f4")),
        Some(h!("f3")),
        Some(h!("g2")),
        Some(h!("f2")),
        Some(h!("f1")),
        Some(h!("e1")),
        None,
        Some(h!("d1")),
        Some(h!("c1")),
        Some(h!("d2")),
        Some(h!("d3"))
    ],
    [ 
        /* f2 */
        Some(h!("f3")),
        Some(h!("g3")),
        Some(h!("g2")),
        Some(h!("h1")),
        Some(h!("g1")),
        None,
        Some(h!("f1")),
        None,
        Some(h!("e1")),
        Some(h!("d1")),
        Some(h!("e2")),
        Some(h!("e3"))
    ],
    [ 
        /* g2 */
        Some(h!("g3")),
        Some(h!("h3")),
        Some(h!("h2")),
        Some(h!("i1")),
        Some(h!("h1")),
        None,
        Some(h!("g1")),
        Some(h!("f1")),
        Some(h!("f2")),
        Some(h!("e2")),
        Some(h!("f3")),
        Some(h!("f4"))
    ],
    [ 
        /* h2 */
        Some(h!("h3")),
        Some(h!("i3")),
        Some(h!("i2")),
        Some(h!("k1")),
        Some(h!("i1")),
        None,
        Some(h!("h1")),
        Some(h!("g1")),
        Some(h!("g2")),
        Some(h!("f3")),
        Some(h!("g3")),
        Some(h!("g4"))
    ],
    [ 
        /* i2 */
        Some(h!("i3")),
        Some(h!("k3")),
        Some(h!("k2")),
        Some(h!("l1")),
        Some(h!("k1")),
        None,
        Some(h!("i1")),
        Some(h!("h1")),
        Some(h!("h2")),
        Some(h!("g3")),
        Some(h!("h3")),
        Some(h!("h4"))
    ],
    [ 
        /* k2 */
        Some(h!("k3")),
        Some(h!("l3")),
        Some(h!("l2")),
        None,
        Some(h!("l1")),
        None,
        Some(h!("k1")),
        Some(h!("i1")),
        Some(h!("i2")),
        Some(h!("h3")),
        Some(h!("i3")),
        Some(h!("i4"))
    ],
    [ 
        /* l2 */
        Some(h!("l3")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("l1")),
        Some(h!("k1")),
        Some(h!("k2")),
        Some(h!("i3")),
        Some(h!("k3")),
        Some(h!("k4"))
    ],
    [ 
        /* a1 */
        Some(h!("a2")),
        Some(h!("b3")),
        Some(h!("b2")),
        Some(h!("c2")),
        Some(h!("b1")),
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ],
    [ 
        /* b1 */
        Some(h!("b2")),
        Some(h!("c3")),
        Some(h!("c2")),
        Some(h!("d2")),
        Some(h!("c1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("a1")),
        Some(h!("a2"))
    ],
    [ 
        /* c1 */
        Some(h!("c2")),
        Some(h!("d3")),
        Some(h!("d2")),
        Some(h!("e2")),
        Some(h!("d1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("b1")),
        Some(h!("b2"))
    ],
    [ 
        /* d1 */
        Some(h!("d2")),
        Some(h!("e3")),
        Some(h!("e2")),
        Some(h!("f2")),
        Some(h!("e1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("c1")),
        Some(h!("c2"))
    ],
    [ 
        /* e1 */
        Some(h!("e2")),
        Some(h!("f3")),
        Some(h!("f2")),
        Some(h!("g1")),
        Some(h!("f1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("d1")),
        Some(h!("d2"))
    ],
    [ 
        /* f1 */
        Some(h!("f2")),
        Some(h!("g2")),
        Some(h!("g1")),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(h!("e1")),
        Some(h!("e2"))
    ],
    [ 
        /* g1 */
        Some(h!("g2")),
        Some(h!("h2")),
        Some(h!("h1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("f1")),
        Some(h!("e1")),
        Some(h!("f2")),
        Some(h!("f3"))
    ],
    [ 
        /* h1 */
        Some(h!("h2")),
        Some(h!("i2")),
        Some(h!("i1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("g1")),
        Some(h!("f2")),
        Some(h!("g2")),
        Some(h!("g3"))
    ],
    [ 
        /* i1 */
        Some(h!("i2")),
        Some(h!("k2")),
        Some(h!("k1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("h1")),
        Some(h!("g2")),
        Some(h!("h2")),
        Some(h!("h3"))
    ],
    [ 
        /* k1 */
        Some(h!("k2")),
        Some(h!("l2")),
        Some(h!("l1")),
        None,
        None,
        None,
        None,
        None,
        Some(h!("i1")),
        Some(h!("h2")),
        Some(h!("i2")),
        Some(h!("i3"))
    ],
    [ 
        /* l1 */
        Some(h!("l2")),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(h!("k1")),
        Some(h!("i2")),
        Some(h!("k2")),
        Some(h!("k3"))
    ]
];

/// Piece color
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Color {
    #[serde(rename(deserialize = "b", serialize = "b"))]
    Black,
    #[serde(rename(deserialize = "w", serialize = "w"))]
    White,
}

/// Piece symbols
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Piece {
    #[serde(rename(deserialize = "P", serialize = "P"))]
    WhitePawn,

    #[serde(rename(deserialize = "N", serialize = "N"))]
    WhiteKnight,

    #[serde(rename(deserialize = "B", serialize = "B"))]
    WhiteBishop,

    #[serde(rename(deserialize = "R", serialize = "R"))]
    WhiteRook,

    #[serde(rename(deserialize = "Q", serialize = "Q"))]
    WhiteQueen,

    #[serde(rename(deserialize = "K", serialize = "K"))]
    WhiteKing,

    #[serde(rename(deserialize = "p", serialize = "p"))]
    BlackPawn,

    #[serde(rename(deserialize = "n", serialize = "n"))]
    BlackKnight,

    #[serde(rename(deserialize = "b", serialize = "b"))]
    BlackBishop,

    #[serde(rename(deserialize = "r", serialize = "r"))]
    BlackRook,

    #[serde(rename(deserialize = "q", serialize = "q"))]
    BlackQueen,

    #[serde(rename(deserialize = "k", serialize = "k"))]
    BlackKing,
}

/// Promotion pieces
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PromotionPiece {
    Bishop,
    Knight,
    Queen,
    Rook,
}
