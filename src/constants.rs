use crate::hex;

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
pub const GRAPH: [[Option<u8>; 12]; 91] = [
    [ 
        /* f11 */
        None,              None,              None,
        None,              Some(hex!("g10")), Some(hex!("g9")),
        Some(hex!("f10")), Some(hex!("e9")),  Some(hex!("e10")),
        None,              None,              None
    ],
    [ 
        /* e10 */
        None,              None,              Some(hex!("f11")),
        Some(hex!("g10")), Some(hex!("f10")), Some(hex!("f9")),
        Some(hex!("e9")),  Some(hex!("d8")),  Some(hex!("d9")),
        None,              None,              None
    ],
    [ 
        /* f10 */
        Some(hex!("f11")), None,              Some(hex!("g10")),
        Some(hex!("h9")),  Some(hex!("g9")),  Some(hex!("g8")),
        Some(hex!("f9")),  Some(hex!("e8")),  Some(hex!("e9")),
        Some(hex!("d9")),  Some(hex!("e10")), None
    ],
    [ 
        /* g10 */
        None,              None,              None,
        None,              Some(hex!("h9")),  Some(hex!("h8")),
        Some(hex!("g9")),  Some(hex!("f9")),  Some(hex!("f10")),
        Some(hex!("e10")), Some(hex!("f11")), None
    ],
    [ 
        /* d9 */
        None,              None,             Some(hex!("e10")),
        Some(hex!("f10")), Some(hex!("e9")), Some(hex!("e8")),
        Some(hex!("d8")),  Some(hex!("c7")), Some(hex!("c8")),
        None,              None,             None
    ],
    [ 
        /* e9 */
        Some(hex!("e10")), Some(hex!("f11")), Some(hex!("f10")),
        Some(hex!("g9")),  Some(hex!("f9")),  Some(hex!("f8")),
        Some(hex!("e8")),  Some(hex!("d7")),  Some(hex!("d8")),
        Some(hex!("c8")),  Some(hex!("d9")),              None
    ],
    [ 
        /* f9 */
        Some(hex!("f10")), Some(hex!("g10")), Some(hex!("g9")),
        Some(hex!("h8")),  Some(hex!("g8")),  Some(hex!("g7")),
        Some(hex!("f8")),  Some(hex!("e7")),  Some(hex!("e8")),
        Some(hex!("d8")),  Some(hex!("e9")),  Some(hex!("e10"))
    ],
    [ 
        /* g9 */
        Some(hex!("g10")), None,              Some(hex!("h9")),
        Some(hex!("i8")),  Some(hex!("h8")),  Some(hex!("h7")),
        Some(hex!("g8")),  Some(hex!("f8")),  Some(hex!("f9")),
        Some(hex!("e9")),  Some(hex!("f10")), Some(hex!("f11"))
    ],
    [ 
        /* h9 */
        None,              None,              None,
        None,              Some(hex!("i8")),  Some(hex!("i7")),
        Some(hex!("h8")),  Some(hex!("g8")),  Some(hex!("g9")),
        Some(hex!("f10")), Some(hex!("g10")), None
    ],
    [ 
        /* c8 */
        None,             None,             Some(hex!("d9")),
        Some(hex!("e9")), Some(hex!("d8")), Some(hex!("d7")),
        Some(hex!("c7")), Some(hex!("b6")), Some(hex!("b7")),
        None,             None,             None
    ],
    [ 
        /* d8 */
        Some(hex!("d9")), Some(hex!("e10")), Some(hex!("e9")),
        Some(hex!("f9")), Some(hex!("e8")),  Some(hex!("e7")),
        Some(hex!("d7")), Some(hex!("c6")),  Some(hex!("c7")),
        Some(hex!("b7")), Some(hex!("c8")),              None
    ],
    [ 
        /* e8 */
        Some(hex!("e9")), Some(hex!("f10")), Some(hex!("f9")),
        Some(hex!("g8")), Some(hex!("f8")),  Some(hex!("f7")),
        Some(hex!("e7")), Some(hex!("d6")),  Some(hex!("d7")),
        Some(hex!("c7")), Some(hex!("d8")),  Some(hex!("d9"))
    ],
    [ 
        /* f8 */
        Some(hex!("f9")), Some(hex!("g9")), Some(hex!("g8")),
        Some(hex!("h7")), Some(hex!("g7")), Some(hex!("g6")),
        Some(hex!("f7")), Some(hex!("e6")), Some(hex!("e7")),
        Some(hex!("d7")), Some(hex!("e8")), Some(hex!("e9"))
    ],
    [ 
        /* g8 */
        Some(hex!("g9")), Some(hex!("h9")), Some(hex!("h8")),
        Some(hex!("i7")), Some(hex!("h7")), Some(hex!("h6")),
        Some(hex!("g7")), Some(hex!("f7")), Some(hex!("f8")),
        Some(hex!("e8")), Some(hex!("f9")), Some(hex!("f10"))
    ],
    [ 
        /* h8 */
        Some(hex!("h9")), None,             Some(hex!("i8")),
        Some(hex!("k7")), Some(hex!("i7")), Some(hex!("i6")),
        Some(hex!("h7")), Some(hex!("g7")), Some(hex!("g8")),
        Some(hex!("f9")), Some(hex!("g9")), Some(hex!("g10"))
    ],
    [ 
        /* i8 */
        None,             None,             None,
        None,             Some(hex!("k7")), Some(hex!("k6")),
        Some(hex!("i7")), Some(hex!("h7")), Some(hex!("h8")),
        Some(hex!("g9")), Some(hex!("h9")), None
    ],
    [ 
        /* b7 */
        None,             None,             Some(hex!("c8")),
        Some(hex!("d8")), Some(hex!("c7")), Some(hex!("c6")),
        Some(hex!("b6")), Some(hex!("a5")), Some(hex!("a6")),
        None,             None,             None
    ],
    [ 
        /* c7 */
        Some(hex!("c8")), Some(hex!("d9")), Some(hex!("d8")),
        Some(hex!("e8")), Some(hex!("d7")), Some(hex!("d6")),
        Some(hex!("c6")), Some(hex!("b5")), Some(hex!("b6")),
        Some(hex!("a6")), Some(hex!("b7")), None
    ],
    [ 
        /* d7 */
        Some(hex!("d8")), Some(hex!("e9")), Some(hex!("e8")),
        Some(hex!("f8")), Some(hex!("e7")), Some(hex!("e6")),
        Some(hex!("d6")), Some(hex!("c5")), Some(hex!("c6")),
        Some(hex!("b6")), Some(hex!("c7")), Some(hex!("c8"))
    ],
    [ 
        /* e7 */
        Some(hex!("e8")), Some(hex!("f9")), Some(hex!("f8")),
        Some(hex!("g7")), Some(hex!("f7")), Some(hex!("f6")),
        Some(hex!("e6")), Some(hex!("d5")), Some(hex!("d6")),
        Some(hex!("c6")), Some(hex!("d7")), Some(hex!("d8"))
    ],
    [ 
        /* f7 */
        Some(hex!("f8")), Some(hex!("g8")), Some(hex!("g7")),
        Some(hex!("h6")), Some(hex!("g6")), Some(hex!("g5")),
        Some(hex!("f6")), Some(hex!("e5")), Some(hex!("e6")),
        Some(hex!("d6")), Some(hex!("e7")), Some(hex!("e8"))
    ],
    [ 
        /* g7 */
        Some(hex!("g8")), Some(hex!("h8")), Some(hex!("h7")),
        Some(hex!("i6")), Some(hex!("h6")), Some(hex!("h5")),
        Some(hex!("g6")), Some(hex!("f6")), Some(hex!("f7")),
        Some(hex!("e7")), Some(hex!("f8")), Some(hex!("f9"))
    ],
    [ 
        /* h7 */
        Some(hex!("h8")), Some(hex!("i8")), Some(hex!("i7")),
        Some(hex!("k6")), Some(hex!("i6")), Some(hex!("i5")),
        Some(hex!("h6")), Some(hex!("g6")), Some(hex!("g7")),
        Some(hex!("f8")), Some(hex!("g8")), Some(hex!("g9"))
    ],
    [ 
        /* i7 */
        Some(hex!("i8")), None,             Some(hex!("k7")),
        Some(hex!("l6")), Some(hex!("k6")), Some(hex!("k5")),
        Some(hex!("i6")), Some(hex!("h6")), Some(hex!("h7")),
        Some(hex!("g8")), Some(hex!("h8")), Some(hex!("h9"))
    ],
    [ 
        /* k7 */
        None,             None,             None,
        None,             Some(hex!("l6")), Some(hex!("l5")),
        Some(hex!("k6")), Some(hex!("i6")), Some(hex!("i7")),
        Some(hex!("h8")), Some(hex!("i8")), None
    ],
    [ 
        /* a6 */
        None,             None,             Some(hex!("b7")),
        Some(hex!("c7")), Some(hex!("b6")), Some(hex!("b5")),
        Some(hex!("a5")), None,             None,
        None,             None,             None
    ],
    [ 
        /* b6 */
        Some(hex!("b7")), Some(hex!("c8")), Some(hex!("c7")),
        Some(hex!("d7")), Some(hex!("c6")), Some(hex!("c5")),
        Some(hex!("b5")), Some(hex!("a4")), Some(hex!("a5")),
        None,             Some(hex!("a6")), None
    ],
    [ 
        /* c6 */
        Some(hex!("c7")), Some(hex!("d8")), Some(hex!("d7")),
        Some(hex!("e7")), Some(hex!("d6")), Some(hex!("d5")),
        Some(hex!("c5")), Some(hex!("b4")), Some(hex!("b5")),
        Some(hex!("a5")), Some(hex!("b6")), Some(hex!("b7"))
    ],
    [ 
        /* d6 */
        Some(hex!("d7")), Some(hex!("e8")), Some(hex!("e7")),
        Some(hex!("f7")), Some(hex!("e6")), Some(hex!("e5")),
        Some(hex!("d5")), Some(hex!("c4")), Some(hex!("c5")),
        Some(hex!("b5")), Some(hex!("c6")), Some(hex!("c7"))
    ],
    [ 
        /* e6 */
        Some(hex!("e7")), Some(hex!("f8")), Some(hex!("f7")),
        Some(hex!("g6")), Some(hex!("f6")), Some(hex!("f5")),
        Some(hex!("e5")), Some(hex!("d4")), Some(hex!("d5")),
        Some(hex!("c5")), Some(hex!("d6")), Some(hex!("d7"))
    ],
    [ 
        /* f6 */
        Some(hex!("f7")), Some(hex!("g7")), Some(hex!("g6")),
        Some(hex!("h5")), Some(hex!("g5")), Some(hex!("g4")),
        Some(hex!("f5")), Some(hex!("e4")), Some(hex!("e5")),
        Some(hex!("d5")), Some(hex!("e6")), Some(hex!("e7"))
    ],
    [ 
        /* g6 */
        Some(hex!("g7")), Some(hex!("h7")), Some(hex!("h6")),
        Some(hex!("i5")), Some(hex!("h5")), Some(hex!("h4")),
        Some(hex!("g5")), Some(hex!("f5")), Some(hex!("f6")),
        Some(hex!("e6")), Some(hex!("f7")), Some(hex!("f8"))
    ],
    [ 
        /* h6 */
        Some(hex!("h7")), Some(hex!("i7")), Some(hex!("i6")),
        Some(hex!("k5")), Some(hex!("i5")), Some(hex!("i4")),
        Some(hex!("h5")), Some(hex!("g5")), Some(hex!("g6")),
        Some(hex!("f7")), Some(hex!("g7")), Some(hex!("g8"))
    ],
    [ 
        /* i6 */
        Some(hex!("i7")), Some(hex!("k7")), Some(hex!("k6")),
        Some(hex!("l5")), Some(hex!("k5")), Some(hex!("k4")),
        Some(hex!("i5")), Some(hex!("h5")), Some(hex!("h6")),
        Some(hex!("g7")), Some(hex!("h7")), Some(hex!("h8"))
    ],
    [ 
        /* k6 */
        Some(hex!("k7")), None,             Some(hex!("l6")),
        None,             Some(hex!("l5")), Some(hex!("l4")),
        Some(hex!("k5")), Some(hex!("i5")), Some(hex!("i6")),
        Some(hex!("h7")), Some(hex!("i7")), Some(hex!("i8"))
    ],
    [ 
        /* l6 */
        None,             None,             None,
        None,             None,             None,
        Some(hex!("l5")), Some(hex!("k5")), Some(hex!("k6")),
        Some(hex!("i7")), Some(hex!("k7")), None
    ],
    [ 
        /* a5 */
        Some(hex!("a6")), Some(hex!("b7")), Some(hex!("b6")),
        Some(hex!("c6")), Some(hex!("b5")), Some(hex!("b4")),
        Some(hex!("a4")), None,             None,
        None,             None,             None
    ],
    [ 
        /* b5 */
        Some(hex!("b6")), Some(hex!("c7")), Some(hex!("c6")),
        Some(hex!("d6")), Some(hex!("c5")), Some(hex!("c4")),
        Some(hex!("b4")), Some(hex!("a3")), Some(hex!("a4")),
        None,             Some(hex!("a5")), Some(hex!("a6"))
    ],
    [ 
        /* c5 */
        Some(hex!("c6")), Some(hex!("d7")), Some(hex!("d6")),
        Some(hex!("e6")), Some(hex!("d5")), Some(hex!("d4")),
        Some(hex!("c4")), Some(hex!("b3")), Some(hex!("b4")),
        Some(hex!("a4")), Some(hex!("b5")), Some(hex!("b6"))
    ],
    [ 
        /* d5 */
        Some(hex!("d6")), Some(hex!("e7")), Some(hex!("e6")),
        Some(hex!("f6")), Some(hex!("e5")), Some(hex!("e4")),
        Some(hex!("d4")), Some(hex!("c3")), Some(hex!("c4")),
        Some(hex!("b4")), Some(hex!("c5")), Some(hex!("c6"))
    ],
    [ 
        /* e5 */
        Some(hex!("e6")), Some(hex!("f7")), Some(hex!("f6")),
        Some(hex!("g5")), Some(hex!("f5")), Some(hex!("f4")),
        Some(hex!("e4")), Some(hex!("d3")), Some(hex!("d4")),
        Some(hex!("c4")), Some(hex!("d5")), Some(hex!("d6"))
    ],
    [ 
        /* f5 */
        Some(hex!("f6")), Some(hex!("g6")), Some(hex!("g5")),
        Some(hex!("h4")), Some(hex!("g4")), Some(hex!("g3")),
        Some(hex!("f4")), Some(hex!("e3")), Some(hex!("e4")),
        Some(hex!("d4")), Some(hex!("e5")), Some(hex!("e6"))
    ],
    [ 
        /* g5 */
        Some(hex!("g6")), Some(hex!("h6")), Some(hex!("h5")),
        Some(hex!("i4")), Some(hex!("h4")), Some(hex!("h3")),
        Some(hex!("g4")), Some(hex!("f4")), Some(hex!("f5")),
        Some(hex!("e5")), Some(hex!("f6")), Some(hex!("f7"))
    ],
    [ 
        /* h5 */
        Some(hex!("h6")), Some(hex!("i6")), Some(hex!("i5")),
        Some(hex!("k4")), Some(hex!("i4")), Some(hex!("i3")),
        Some(hex!("h4")), Some(hex!("g4")), Some(hex!("g5")),
        Some(hex!("f6")), Some(hex!("g6")), Some(hex!("g7"))
    ],
    [ 
        /* i5 */
        Some(hex!("i6")), Some(hex!("k6")), Some(hex!("k5")),
        Some(hex!("l4")), Some(hex!("k4")), Some(hex!("k3")),
        Some(hex!("i4")), Some(hex!("h4")), Some(hex!("h5")),
        Some(hex!("g6")), Some(hex!("h6")), Some(hex!("h7"))
    ],
    [ 
        /* k5 */
        Some(hex!("k6")), Some(hex!("l6")), Some(hex!("l5")),
        None,             Some(hex!("l4")), Some(hex!("l3")),
        Some(hex!("k4")), Some(hex!("i4")), Some(hex!("i5")),
        Some(hex!("h6")), Some(hex!("i6")), Some(hex!("i7"))
    ],
    [ 
        /* l5 */
        Some(hex!("l6")), None,             None,
        None,             None,             None,
        Some(hex!("l4")), Some(hex!("k4")), Some(hex!("k5")),
        Some(hex!("i6")), Some(hex!("k6")), Some(hex!("k7"))
    ],
    [ 
        /* a4 */
        Some(hex!("a5")), Some(hex!("b6")), Some(hex!("b5")),
        Some(hex!("c5")), Some(hex!("b4")), Some(hex!("b3")),
        Some(hex!("a3")), None,             None,
        None,             None,             None
    ],
    [ 
        /* b4 */
        Some(hex!("b5")), Some(hex!("c6")), Some(hex!("c5")),
        Some(hex!("d5")), Some(hex!("c4")), Some(hex!("c3")),
        Some(hex!("b3")), Some(hex!("a2")), Some(hex!("a3")),
        None,             Some(hex!("a4")), Some(hex!("a5"))
    ],
    [ 
        /* c4 */
        Some(hex!("c5")), Some(hex!("d6")), Some(hex!("d5")),
        Some(hex!("e5")), Some(hex!("d4")), Some(hex!("d3")),
        Some(hex!("c3")), Some(hex!("b2")), Some(hex!("b3")),
        Some(hex!("a3")), Some(hex!("b4")), Some(hex!("b5"))
    ],
    [ 
        /* d4 */
        Some(hex!("d5")), Some(hex!("e6")), Some(hex!("e5")),
        Some(hex!("f5")), Some(hex!("e4")), Some(hex!("e3")),
        Some(hex!("d3")), Some(hex!("c2")), Some(hex!("c3")),
        Some(hex!("b3")), Some(hex!("c4")), Some(hex!("c5"))
    ],
    [ 
        /* e4 */
        Some(hex!("e5")), Some(hex!("f6")), Some(hex!("f5")),
        Some(hex!("g4")), Some(hex!("f4")), Some(hex!("f3")),
        Some(hex!("e3")), Some(hex!("d2")), Some(hex!("d3")),
        Some(hex!("c3")), Some(hex!("d4")), Some(hex!("d5"))
    ],
    [ 
        /* f4 */
        Some(hex!("f5")), Some(hex!("g5")), Some(hex!("g4")),
        Some(hex!("h3")), Some(hex!("g3")), Some(hex!("g2")),
        Some(hex!("f3")), Some(hex!("e2")), Some(hex!("e3")),
        Some(hex!("d3")), Some(hex!("e4")), Some(hex!("e5"))
    ],
    [ 
        /* g4 */
        Some(hex!("g5")), Some(hex!("h5")), Some(hex!("h4")),
        Some(hex!("i3")), Some(hex!("h3")), Some(hex!("h2")),
        Some(hex!("g3")), Some(hex!("f3")), Some(hex!("f4")),
        Some(hex!("e4")), Some(hex!("f5")), Some(hex!("f6"))
    ],
    [ 
        /* h4 */
        Some(hex!("h5")), Some(hex!("i5")), Some(hex!("i4")),
        Some(hex!("k3")), Some(hex!("i3")), Some(hex!("i2")),
        Some(hex!("h3")), Some(hex!("g3")), Some(hex!("g4")),
        Some(hex!("f5")), Some(hex!("g5")), Some(hex!("g6"))
    ],
    [ 
        /* i4 */
        Some(hex!("i5")), Some(hex!("k5")), Some(hex!("k4")),
        Some(hex!("l3")), Some(hex!("k3")), Some(hex!("k2")),
        Some(hex!("i3")), Some(hex!("h3")), Some(hex!("h4")),
        Some(hex!("g5")), Some(hex!("h5")), Some(hex!("h6"))
    ],
    [ 
        /* k4 */
        Some(hex!("k5")), Some(hex!("l5")), Some(hex!("l4")),
        None,             Some(hex!("l3")), Some(hex!("l2")),
        Some(hex!("k3")), Some(hex!("i3")), Some(hex!("i4")),
        Some(hex!("h5")), Some(hex!("i5")), Some(hex!("i6"))
    ],
    [ 
        /* l4 */
        Some(hex!("l5")), None,             None,
        None,             None,             None,
        Some(hex!("l3")), Some(hex!("k3")), Some(hex!("k4")),
        Some(hex!("i5")), Some(hex!("k5")), Some(hex!("k6"))
    ],
    [ 
        /* a3 */
        Some(hex!("a4")), Some(hex!("b5")), Some(hex!("b4")),
        Some(hex!("c4")), Some(hex!("b3")), Some(hex!("b2")),
        Some(hex!("a2")), None,             None,
        None,             None,             None
    ],
    [ 
        /* b3 */
        Some(hex!("b4")), Some(hex!("c5")), Some(hex!("c4")),
        Some(hex!("d4")), Some(hex!("c3")), Some(hex!("c2")),
        Some(hex!("b2")), Some(hex!("a1")), Some(hex!("a2")),
        None,             Some(hex!("a3")), Some(hex!("a4"))
    ],
    [ 
        /* c3 */
        Some(hex!("c4")), Some(hex!("d5")), Some(hex!("d4")),
        Some(hex!("e4")), Some(hex!("d3")), Some(hex!("d2")),
        Some(hex!("c2")), Some(hex!("b1")), Some(hex!("b2")),
        Some(hex!("a2")), Some(hex!("b3")), Some(hex!("b4"))
    ],
    [ 
        /* d3 */
        Some(hex!("d4")), Some(hex!("e5")), Some(hex!("e4")),
        Some(hex!("f4")), Some(hex!("e3")), Some(hex!("e2")),
        Some(hex!("d2")), Some(hex!("c1")), Some(hex!("c2")),
        Some(hex!("b2")), Some(hex!("c3")), Some(hex!("c4"))
    ],
    [ 
        /* e3 */
        Some(hex!("e4")), Some(hex!("f5")), Some(hex!("f4")),
        Some(hex!("g3")), Some(hex!("f3")), Some(hex!("f2")),
        Some(hex!("e2")), Some(hex!("d1")), Some(hex!("d2")),
        Some(hex!("c2")), Some(hex!("d3")), Some(hex!("d4"))
    ],
    [ 
        /* f3 */
        Some(hex!("f4")), Some(hex!("g4")), Some(hex!("g3")),
        Some(hex!("h2")), Some(hex!("g2")), Some(hex!("g1")),
        Some(hex!("f2")), Some(hex!("e1")), Some(hex!("e2")),
        Some(hex!("d2")), Some(hex!("e3")), Some(hex!("e4"))
    ],
    [ 
        /* g3 */
        Some(hex!("g4")), Some(hex!("h4")), Some(hex!("h3")),
        Some(hex!("i2")), Some(hex!("h2")), Some(hex!("h1")),
        Some(hex!("g2")), Some(hex!("f2")), Some(hex!("f3")),
        Some(hex!("e3")), Some(hex!("f4")), Some(hex!("f5"))
    ],
    [ 
        /* h3 */
        Some(hex!("h4")), Some(hex!("i4")), Some(hex!("i3")),
        Some(hex!("k2")), Some(hex!("i2")), Some(hex!("i1")),
        Some(hex!("h2")), Some(hex!("g2")), Some(hex!("g3")),
        Some(hex!("f4")), Some(hex!("g4")), Some(hex!("g5"))
    ],
    [ 
        /* i3 */
        Some(hex!("i4")), Some(hex!("k4")), Some(hex!("k3")),
        Some(hex!("l2")), Some(hex!("k2")), Some(hex!("k1")),
        Some(hex!("i2")), Some(hex!("h2")), Some(hex!("h3")),
        Some(hex!("g4")), Some(hex!("h4")), Some(hex!("h5"))
    ],
    [ 
        /* k3 */
        Some(hex!("k4")), Some(hex!("l4")), Some(hex!("l3")),
        None,             Some(hex!("l2")), Some(hex!("l1")),
        Some(hex!("k2")), Some(hex!("i2")), Some(hex!("i3")),
        Some(hex!("h4")), Some(hex!("i4")), Some(hex!("i5"))
    ],
    [ 
        /* l3 */
        Some(hex!("l4")), None,             None,
        None,             None,             None,
        Some(hex!("l2")), Some(hex!("k2")), Some(hex!("k3")),
        Some(hex!("i4")), Some(hex!("k4")), Some(hex!("k5"))
    ],
    [ 
        /* a2 */
        Some(hex!("a3")), Some(hex!("b4")), Some(hex!("b3")),
        Some(hex!("c3")), Some(hex!("b2")), Some(hex!("b1")),
        Some(hex!("a1")), None,             None,
        None,             None,             None
    ],
    [ 
        /* b2 */
        Some(hex!("b3")), Some(hex!("c4")), Some(hex!("c3")),
        Some(hex!("d3")), Some(hex!("c2")), Some(hex!("c1")),
        Some(hex!("b1")), None,             Some(hex!("a1")),
        None,             Some(hex!("a2")), Some(hex!("a3"))
    ],
    [ 
        /* c2 */
        Some(hex!("c3")), Some(hex!("d4")), Some(hex!("d3")),
        Some(hex!("e3")), Some(hex!("d2")), Some(hex!("d1")),
        Some(hex!("c1")), None,             Some(hex!("b1")),
        Some(hex!("a1")), Some(hex!("b2")), Some(hex!("b3"))
    ],
    [ 
        /* d2 */
        Some(hex!("d3")), Some(hex!("e4")), Some(hex!("e3")),
        Some(hex!("f3")), Some(hex!("e2")), Some(hex!("e1")),
        Some(hex!("d1")), None,             Some(hex!("c1")),
        Some(hex!("b1")), Some(hex!("c2")), Some(hex!("c3"))
    ],
    [ 
        /* e2 */
        Some(hex!("e3")), Some(hex!("f4")), Some(hex!("f3")),
        Some(hex!("g2")), Some(hex!("f2")), Some(hex!("f1")),
        Some(hex!("e1")), None,             Some(hex!("d1")),
        Some(hex!("c1")), Some(hex!("d2")), Some(hex!("d3"))
    ],
    [ 
        /* f2 */
        Some(hex!("f3")), Some(hex!("g3")), Some(hex!("g2")),
        Some(hex!("h1")), Some(hex!("g1")), None,
        Some(hex!("f1")), None,             Some(hex!("e1")),
        Some(hex!("d1")), Some(hex!("e2")), Some(hex!("e3"))
    ],
    [ 
        /* g2 */
        Some(hex!("g3")), Some(hex!("h3")), Some(hex!("h2")),
        Some(hex!("i1")), Some(hex!("h1")), None,
        Some(hex!("g1")), Some(hex!("f1")), Some(hex!("f2")),
        Some(hex!("e2")), Some(hex!("f3")), Some(hex!("f4"))
    ],
    [ 
        /* h2 */
        Some(hex!("h3")), Some(hex!("i3")), Some(hex!("i2")),
        Some(hex!("k1")), Some(hex!("i1")), None,
        Some(hex!("h1")), Some(hex!("g1")), Some(hex!("g2")),
        Some(hex!("f3")), Some(hex!("g3")), Some(hex!("g4"))
    ],
    [ 
        /* i2 */
        Some(hex!("i3")), Some(hex!("k3")), Some(hex!("k2")),
        Some(hex!("l1")), Some(hex!("k1")), None,
        Some(hex!("i1")), Some(hex!("h1")), Some(hex!("h2")),
        Some(hex!("g3")), Some(hex!("h3")), Some(hex!("h4"))
    ],
    [ 
        /* k2 */
        Some(hex!("k3")), Some(hex!("l3")), Some(hex!("l2")),
        None,             Some(hex!("l1")), None,
        Some(hex!("k1")), Some(hex!("i1")), Some(hex!("i2")),
        Some(hex!("h3")), Some(hex!("i3")), Some(hex!("i4"))
    ],
    [ 
        /* l2 */
        Some(hex!("l3")), None,             None,
        None,             None,             None,
        Some(hex!("l1")), Some(hex!("k1")), Some(hex!("k2")),
        Some(hex!("i3")), Some(hex!("k3")), Some(hex!("k4"))
    ],
    [ 
        /* a1 */
        Some(hex!("a2")), Some(hex!("b3")), Some(hex!("b2")),
        Some(hex!("c2")), Some(hex!("b1")), None,
        None,             None,             None,
        None,             None,             None
    ],
    [ 
        /* b1 */
        Some(hex!("b2")), Some(hex!("c3")), Some(hex!("c2")),
        Some(hex!("d2")), Some(hex!("c1")), None,
        None,             None,             None,
        None,             Some(hex!("a1")), Some(hex!("a2"))
    ],
    [ 
        /* c1 */
        Some(hex!("c2")), Some(hex!("d3")), Some(hex!("d2")),
        Some(hex!("e2")), Some(hex!("d1")), None,
        None,             None,             None,
        None,             Some(hex!("b1")), Some(hex!("b2"))
    ],
    [ 
        /* d1 */
        Some(hex!("d2")), Some(hex!("e3")), Some(hex!("e2")),
        Some(hex!("f2")), Some(hex!("e1")), None,
        None,             None,             None,
        None,             Some(hex!("c1")), Some(hex!("c2"))
    ],
    [ 
        /* e1 */
        Some(hex!("e2")), Some(hex!("f3")), Some(hex!("f2")),
        Some(hex!("g1")), Some(hex!("f1")), None,
        None,             None,             None,
        None,             Some(hex!("d1")), Some(hex!("d2"))
    ],
    [ 
        /* f1 */
        Some(hex!("f2")), Some(hex!("g2")), Some(hex!("g1")),
        None,             None,             None,
        None,             None,             None,
        None,             Some(hex!("e1")), Some(hex!("e2"))
    ],
    [ 
        /* g1 */
        Some(hex!("g2")), Some(hex!("h2")), Some(hex!("h1")),
        None,             None,             None,
        None,             None,             Some(hex!("f1")),
        Some(hex!("e1")), Some(hex!("f2")), Some(hex!("f3"))
    ],
    [ 
        /* h1 */
        Some(hex!("h2")), Some(hex!("i2")), Some(hex!("i1")),
        None,             None,             None,
        None,             None,             Some(hex!("g1")),
        Some(hex!("f2")), Some(hex!("g2")), Some(hex!("g3"))
    ],
    [ 
        /* i1 */
        Some(hex!("i2")), Some(hex!("k2")), Some(hex!("k1")),
        None,             None,             None,
        None,             None,             Some(hex!("h1")),
        Some(hex!("g2")), Some(hex!("h2")), Some(hex!("h3"))
    ],
    [ 
        /* k1 */
        Some(hex!("k2")), Some(hex!("l2")), Some(hex!("l1")),
        None,             None,             None,
        None,             None,             Some(hex!("i1")),
        Some(hex!("h2")), Some(hex!("i2")), Some(hex!("i3"))
    ],
    [ 
        /* l1 */
        Some(hex!("l2")), None,             None,
        None,             None,             None,
        None,             None,             Some(hex!("k1")),
        Some(hex!("i2")), Some(hex!("k2")), Some(hex!("k3"))
    ]
];

/// Piece color
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

/// Piece symbols
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
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

/// parsed standard algebraic notation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct San {
    pub from: u8,
    pub promotion: Option<PromotionPiece>,
    pub to: u8,
}