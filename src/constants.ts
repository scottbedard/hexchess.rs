/**
 * This graph represents the positions on a hexboard, and their relationship
 * to one another. Each child array represents a position by it's fen-order,
 * surrounded by it's neighbors starting from the position directly above it.
 *
 * Think of it like the hands of a clock, with 12 o'clock being index 0...
 * For example, to find the position directly below f6, we'd first go to that
 * position in a fen (index 30), then look at the 6th index of that array, 41.
 * The 41st fen index is f5.
 */
export const graph = [
  [
    /* f11 */
    ,
    ,
    ,
    ,
    3, // g10
    7, // g9
    2, // f10
    5, // e9
    1, // e10
    ,
    ,
    ,
  ],
  [
    /* e10 */
    ,
    ,
    0, // f11
    3, // g10
    2, // f10
    6, // f9
    5, // e9
    10, // d8
    4, // d9
    ,
    ,
    ,
  ],
  [
    /* f10 */
    0, // f11
    ,
    3, // g10
    8, // h9
    7, // g9
    13, // g8
    6, // f9
    11, // e8
    5, // e9
    4, // d9
    1, // e10
    ,
  ],
  [
    /* g10 */
    ,
    ,
    ,
    ,
    8, // h9
    14, // h8
    7, // g9
    6, // f9
    2, // f10
    1, // e10
    0, // f11
    ,
  ],
  [
    /* d9 */
    ,
    ,
    1, // e10
    2, // f10
    5, // e9
    11, // e8
    10, // d8
    17, // c7
    9, // c8
    ,
    ,
    ,
  ],
  [
    /* e9 */
    1, // e10
    0, // f11
    2, // f10
    7, // g9
    6, // f9
    12, // f8
    11, // e8
    18, // d7
    10, // d8
    9, // c8
    4, // d9
    ,
  ],
  [
    /* f9 */
    2, // f10
    3, // g10
    7, // g9
    14, // h8
    13, // g8
    21, // g7
    12, // f8
    19, // e7
    11, // e8
    10, // d8
    5, // e9
    1, // e10
  ],
  [
    /* g9 */
    3, // g10
    ,
    8, // h9
    15, // i8
    14, // h8
    22, // h7
    13, // g8
    12, // f8
    6, // f9
    5, // e9
    2, // f10
    0, // f11
  ],
  [
    /* h9 */
    ,
    ,
    ,
    ,
    15, // i8
    23, // i7
    14, // h8
    13, // g8
    7, // g9
    2, // f10
    3, // g10
    ,
  ],
  [
    /* c8 */
    ,
    ,
    4, // d9
    5, // e9
    10, // d8
    18, // d7
    17, // c7
    26, // b6
    16, // b7
    ,
    ,
    ,
  ],
  [
    /* d8 */
    4, // d9
    1, // e10
    5, // e9
    6, // f9
    11, // e8
    19, // e7
    18, // d7
    27, // c6
    17, // c7
    16, // b7
    9, // c8
    ,
  ],
  [
    /* e8 */
    5, // e9
    2, // f10
    6, // f9
    13, // g8
    12, // f8
    20, // f7
    19, // e7
    28, // d6
    18, // d7
    17, // c7
    10, // d8
    4, // d9
  ],
  [
    /* f8 */
    6, // f9
    7, // g9
    13, // g8
    22, // h7
    21, // g7
    31, // g6
    20, // f7
    29, // e6
    19, // e7
    18, // d7
    11, // e8
    5, // e9
  ],
  [
    /* g8 */
    7, // g9
    8, // h9
    14, // h8
    23, // i7
    22, // h7
    32, // h6
    21, // g7
    20, // f7
    12, // f8
    11, // e8
    6, // f9
    2, // f10
  ],
  [
    /* h8 */
    8, // h9
    ,
    15, // i8
    24, // k7
    23, // i7
    33, // i6
    22, // h7
    21, // g7
    13, // g8
    6, // f9
    7, // g9
    3, // g10
  ],
  [
    /* i8 */
    ,
    ,
    ,
    ,
    24, // k7
    34, // k6
    23, // i7
    22, // h7
    14, // h8
    7, // g9
    8, // h9
    ,
  ],
  [
    /* b7 */
    ,
    ,
    9, // c8
    10, // d8
    17, // c7
    27, // c6
    26, // b6
    36, // a5
    25, // a6
    ,
    ,
    ,
  ],
  [
    /* c7 */
    9, // c8
    4, // d9
    10, // d8
    11, // e8
    18, // d7
    28, // d6
    27, // c6
    37, // b5
    26, // b6
    25, // a6
    16, // b7
    ,
  ],
  [
    /* d7 */
    10, // d8
    5, // e9
    11, // e8
    12, // f8
    19, // e7
    29, // e6
    28, // d6
    38, // c5
    27, // c6
    26, // b6
    17, // c7
    9, // c8
  ],
  [
    /* e7 */
    11, // e8
    6, // f9
    12, // f8
    21, // g7
    20, // f7
    30, // f6
    29, // e6
    39, // d5
    28, // d6
    27, // c6
    18, // d7
    10, // d8
  ],
  [
    /* f7 */
    12, // f8
    13, // g8
    21, // g7
    32, // h6
    31, // g6
    42, // g5
    30, // f6
    40, // e5
    29, // e6
    28, // d6
    19, // e7
    11, // e8
  ],
  [
    /* g7 */
    13, // g8
    14, // h8
    22, // h7
    33, // i6
    32, // h6
    43, // h5
    31, // g6
    30, // f6
    20, // f7
    19, // e7
    12, // f8
    6, // f9
  ],
  [
    /* h7 */
    14, // h8
    15, // i8
    23, // i7
    34, // k6
    33, // i6
    44, // i5
    32, // h6
    31, // g6
    21, // g7
    12, // f8
    13, // g8
    7, // g9
  ],
  [
    /* i7 */
    15, // i8
    ,
    24, // k7
    35, // l6
    34, // k6
    45, // k5
    33, // i6
    32, // h6
    22, // h7
    13, // g8
    14, // h8
    8, // h9
  ],
  [
    /* k7 */
    ,
    ,
    ,
    ,
    35, // l6
    46, // l5
    34, // k6
    33, // i6
    23, // i7
    14, // h8
    15, // i8
    ,
  ],
  [
    /* a6 */
    ,
    ,
    16, // b7
    17, // c7
    26, // b6
    37, // b5
    36, // a5
    ,
    ,
    ,
    ,
    ,
  ],
  [
    /* b6 */
    16, // b7
    9, // c8
    17, // c7
    18, // d7
    27, // c6
    38, // c5
    37, // b5
    47, // a4
    36, // a5
    ,
    25, // a6
    ,
  ],
  [
    /* c6 */
    17, // c7
    10, // d8
    18, // d7
    19, // e7
    28, // d6
    39, // d5
    38, // c5
    48, // b4
    37, // b5
    36, // a5
    26, // b6
    16, // b7
  ],
  [
    /* d6 */
    18, // d7
    11, // e8
    19, // e7
    20, // f7
    29, // e6
    40, // e5
    39, // d5
    49, // c4
    38, // c5
    37, // b5
    27, // c6
    17, // c7
  ],
  [
    /* e6 */
    19, // e7
    12, // f8
    20, // f7
    31, // g6
    30, // f6
    41, // f5
    40, // e5
    50, // d4
    39, // d5
    38, // c5
    28, // d6
    18, // d7
  ],
  [
    /* f6 */
    20, // f7
    21, // g7
    31, // g6
    43, // h5
    42, // g5
    53, // g4
    41, // f5
    51, // e4
    40, // e5
    39, // d5
    29, // e6
    19, // e7
  ],
  [
    /* g6 */
    21, // g7
    22, // h7
    32, // h6
    44, // i5
    43, // h5
    54, // h4
    42, // g5
    41, // f5
    30, // f6
    29, // e6
    20, // f7
    12, // f8
  ],
  [
    /* h6 */
    22, // h7
    23, // i7
    33, // i6
    45, // k5
    44, // i5
    55, // i4
    43, // h5
    42, // g5
    31, // g6
    20, // f7
    21, // g7
    13, // g8
  ],
  [
    /* i6 */
    23, // i7
    24, // k7
    34, // k6
    46, // l5
    45, // k5
    56, // k4
    44, // i5
    43, // h5
    32, // h6
    21, // g7
    22, // h7
    14, // h8
  ],
  [
    /* k6 */
    24, // k7
    ,
    35, // l6
    ,
    46, // l5
    57, // l4
    45, // k5
    44, // i5
    33, // i6
    22, // h7
    23, // i7
    15, // i8
  ],
  [
    /* l6 */
    ,
    ,
    ,
    ,
    ,
    ,
    46, // l5
    45, // k5
    34, // k6
    23, // i7
    24, // k7
    ,
  ],
  [
    /* a5 */
    25, // a6
    16, // b7
    26, // b6
    27, // c6
    37, // b5
    48, // b4
    47, // a4
    ,
    ,
    ,
    ,
    ,
  ],
  [
    /* b5 */
    26, // b6
    17, // c7
    27, // c6
    28, // d6
    38, // c5
    49, // c4
    48, // b4
    58, // a3
    47, // a4
    ,
    36, // a5
    25, // a6
  ],
  [
    /* c5 */
    27, // c6
    18, // d7
    28, // d6
    29, // e6
    39, // d5
    50, // d4
    49, // c4
    59, // b3
    48, // b4
    47, // a4
    37, // b5
    26, // b6
  ],
  [
    /* d5 */
    28, // d6
    19, // e7
    29, // e6
    30, // f6
    40, // e5
    51, // e4
    50, // d4
    60, // c3
    49, // c4
    48, // b4
    38, // c5
    27, // c6
  ],
  [
    /* e5 */
    29, // e6
    20, // f7
    30, // f6
    42, // g5
    41, // f5
    52, // f4
    51, // e4
    61, // d3
    50, // d4
    49, // c4
    39, // d5
    28, // d6
  ],
  [
    /* f5 */
    30, // f6
    31, // g6
    42, // g5
    54, // h4
    53, // g4
    64, // g3
    52, // f4
    62, // e3
    51, // e4
    50, // d4
    40, // e5
    29, // e6
  ],
  [
    /* g5 */
    31, // g6
    32, // h6
    43, // h5
    55, // i4
    54, // h4
    65, // h3
    53, // g4
    52, // f4
    41, // f5
    40, // e5
    30, // f6
    20, // f7
  ],
  [
    /* h5 */
    32, // h6
    33, // i6
    44, // i5
    56, // k4
    55, // i4
    66, // i3
    54, // h4
    53, // g4
    42, // g5
    30, // f6
    31, // g6
    21, // g7
  ],
  [
    /* i5 */
    33, // i6
    34, // k6
    45, // k5
    57, // l4
    56, // k4
    67, // k3
    55, // i4
    54, // h4
    43, // h5
    31, // g6
    32, // h6
    22, // h7
  ],
  [
    /* k5 */
    34, // k6
    35, // l6
    46, // l5
    ,
    57, // l4
    68, // l3
    56, // k4
    55, // i4
    44, // i5
    32, // h6
    33, // i6
    23, // i7
  ],
  [
    /* l5 */
    35, // l6
    ,
    ,
    ,
    ,
    ,
    57, // l4
    56, // k4
    45, // k5
    33, // i6
    34, // k6
    24, // k7
  ],
  [
    /* a4 */
    36, // a5
    26, // b6
    37, // b5
    38, // c5
    48, // b4
    59, // b3
    58, // a3
    ,
    ,
    ,
    ,
    ,
  ],
  [
    /* b4 */
    37, // b5
    27, // c6
    38, // c5
    39, // d5
    49, // c4
    60, // c3
    59, // b3
    69, // a2
    58, // a3
    ,
    47, // a4
    36, // a5
  ],
  [
    /* c4 */
    38, // c5
    28, // d6
    39, // d5
    40, // e5
    50, // d4
    61, // d3
    60, // c3
    70, // b2
    59, // b3
    58, // a3
    48, // b4
    37, // b5
  ],
  [
    /* d4 */
    39, // d5
    29, // e6
    40, // e5
    41, // f5
    51, // e4
    62, // e3
    61, // d3
    71, // c2
    60, // c3
    59, // b3
    49, // c4
    38, // c5
  ],
  [
    /* e4 */
    40, // e5
    30, // f6
    41, // f5
    53, // g4
    52, // f4
    63, // f3
    62, // e3
    72, // d2
    61, // d3
    60, // c3
    50, // d4
    39, // d5
  ],
  [
    /* f4 */
    41, // f5
    42, // g5
    53, // g4
    65, // h3
    64, // g3
    75, // g2
    63, // f3
    73, // e2
    62, // e3
    61, // d3
    51, // e4
    40, // e5
  ],
  [
    /* g4 */
    42, // g5
    43, // h5
    54, // h4
    66, // i3
    65, // h3
    76, // h2
    64, // g3
    63, // f3
    52, // f4
    51, // e4
    41, // f5
    30, // f6
  ],
  [
    /* h4 */
    43, // h5
    44, // i5
    55, // i4
    67, // k3
    66, // i3
    77, // i2
    65, // h3
    64, // g3
    53, // g4
    41, // f5
    42, // g5
    31, // g6
  ],
  [
    /* i4 */
    44, // i5
    45, // k5
    56, // k4
    68, // l3
    67, // k3
    78, // k2
    66, // i3
    65, // h3
    54, // h4
    42, // g5
    43, // h5
    32, // h6
  ],
  [
    /* k4 */
    45, // k5
    46, // l5
    57, // l4
    ,
    68, // l3
    79, // l2
    67, // k3
    66, // i3
    55, // i4
    43, // h5
    44, // i5
    33, // i6
  ],
  [
    /* l4 */
    46, // l5
    ,
    ,
    ,
    ,
    ,
    68, // l3
    67, // k3
    56, // k4
    44, // i5
    45, // k5
    34, // k6
  ],
  [
    /* a3 */
    47, // a4
    37, // b5
    48, // b4
    49, // c4
    59, // b3
    70, // b2
    69, // a2
    ,
    ,
    ,
    ,
    ,
  ],
  [
    /* b3 */
    48, // b4
    38, // c5
    49, // c4
    50, // d4
    60, // c3
    71, // c2
    70, // b2
    80, // a1
    69, // a2
    ,
    58, // a3
    47, // a4
  ],
  [
    /* c3 */
    49, // c4
    39, // d5
    50, // d4
    51, // e4
    61, // d3
    72, // d2
    71, // c2
    81, // b1
    70, // b2
    69, // a2
    59, // b3
    48, // b4
  ],
  [
    /* d3 */
    50, // d4
    40, // e5
    51, // e4
    52, // f4
    62, // e3
    73, // e2
    72, // d2
    82, // c1
    71, // c2
    70, // b2
    60, // c3
    49, // c4
  ],
  [
    /* e3 */
    51, // e4
    41, // f5
    52, // f4
    64, // g3
    63, // f3
    74, // f2
    73, // e2
    83, // d1
    72, // d2
    71, // c2
    61, // d3
    50, // d4
  ],
  [
    /* f3 */
    52, // f4
    53, // g4
    64, // g3
    76, // h2
    75, // g2
    86, // g1
    74, // f2
    84, // e1
    73, // e2
    72, // d2
    62, // e3
    51, // e4
  ],
  [
    /* g3 */
    53, // g4
    54, // h4
    65, // h3
    77, // i2
    76, // h2
    87, // h1
    75, // g2
    74, // f2
    63, // f3
    62, // e3
    52, // f4
    41, // f5
  ],
  [
    /* h3 */
    54, // h4
    55, // i4
    66, // i3
    78, // k2
    77, // i2
    88, // i1
    76, // h2
    75, // g2
    64, // g3
    52, // f4
    53, // g4
    42, // g5
  ],
  [
    /* i3 */
    55, // i4
    56, // k4
    67, // k3
    79, // l2
    78, // k2
    89, // k1
    77, // i2
    76, // h2
    65, // h3
    53, // g4
    54, // h4
    43, // h5
  ],
  [
    /* k3 */
    56, // k4
    57, // l4
    68, // l3
    ,
    79, // l2
    90, // l1
    78, // k2
    77, // i2
    66, // i3
    54, // h4
    55, // i4
    44, // i5
  ],
  [
    /* l3 */
    57, // l4
    ,
    ,
    ,
    ,
    ,
    79, // l2
    78, // k2
    67, // k3
    55, // i4
    56, // k4
    45, // k5
  ],
  [
    /* a2 */
    58, // a3
    48, // b4
    59, // b3
    60, // c3
    70, // b2
    81, // b1
    80, // a1
    ,
    ,
    ,
    ,
    ,
  ],
  [
    /* b2 */
    59, // b3
    49, // c4
    60, // c3
    61, // d3
    71, // c2
    82, // c1
    81, // b1
    ,
    80, // a1
    ,
    69, // a2
    58, // a3
  ],
  [
    /* c2 */
    60, // c3
    50, // d4
    61, // d3
    62, // e3
    72, // d2
    83, // d1
    82, // c1
    ,
    81, // b1
    80, // a1
    70, // b2
    59, // b3
  ],
  [
    /* d2 */
    61, // d3
    51, // e4
    62, // e3
    63, // f3
    73, // e2
    84, // e1
    83, // d1
    ,
    82, // c1
    81, // b1
    71, // c2
    60, // c3
  ],
  [
    /* e2 */
    62, // e3
    52, // f4
    63, // f3
    75, // g2
    74, // f2
    85, // f1
    84, // e1
    ,
    83, // d1
    82, // c1
    72, // d2
    61, // d3
  ],
  [
    /* f2 */
    63, // f3
    64, // g3
    75, // g2
    87, // h1
    86, // g1
    ,
    85, // f1
    ,
    84, // e1
    83, // d1
    73, // e2
    62, // e3
  ],
  [
    /* g2 */
    64, // g3
    65, // h3
    76, // h2
    88, // i1
    87, // h1
    ,
    86, // g1
    85, // f1
    74, // f2
    73, // e2
    63, // f3
    52, // f4
  ],
  [
    /* h2 */
    65, // h3
    66, // i3
    77, // i2
    89, // k1
    88, // i1
    ,
    87, // h1
    86, // g1
    75, // g2
    63, // f3
    64, // g3
    53, // g4
  ],
  [
    /* i2 */
    66, // i3
    67, // k3
    78, // k2
    90, // l1
    89, // k1
    ,
    88, // i1
    87, // h1
    76, // h2
    64, // g3
    65, // h3
    54, // h4
  ],
  [
    /* k2 */
    67, // k3
    68, // l3
    79, // l2
    ,
    90, // l1
    ,
    89, // k1
    88, // i1
    77, // i2
    65, // h3
    66, // i3
    55, // i4
  ],
  [
    /* l2 */
    68, // l3
    ,
    ,
    ,
    ,
    ,
    90, // l1
    89, // k1
    78, // k2
    66, // i3
    67, // k3
    56, // k4
  ],
  [
    /* a1 */
    69, // a2
    59, // b3
    70, // b2
    71, // c2
    81, // b1
    ,
    ,
    ,
    ,
    ,
    ,
    ,
  ],
  [
    /* b1 */
    70, // b2
    60, // c3
    71, // c2
    72, // d2
    82, // c1
    ,
    ,
    ,
    ,
    ,
    80, // a1
    69, // a2
  ],
  [
    /* c1 */
    71, // c2
    61, // d3
    72, // d2
    73, // e2
    83, // d1
    ,
    ,
    ,
    ,
    ,
    81, // b1
    70, // b2
  ],
  [
    /* d1 */
    72, // d2
    62, // e3
    73, // e2
    74, // f2
    84, // e1
    ,
    ,
    ,
    ,
    ,
    82, // c1
    71, // c2
  ],
  [
    /* e1 */
    73, // e2
    63, // f3
    74, // f2
    86, // g1
    85, // f1
    ,
    ,
    ,
    ,
    ,
    83, // d1
    72, // d2
  ],
  [
    /* f1 */
    74, // f2
    75, // g2
    86, // g1
    ,
    ,
    ,
    ,
    ,
    ,
    ,
    84, // e1
    73, // e2
  ],
  [
    /* g1 */
    75, // g2
    76, // h2
    87, // h1
    ,
    ,
    ,
    ,
    ,
    85, // f1
    84, // e1
    74, // f2
    63, // f3
  ],
  [
    /* h1 */
    76, // h2
    77, // i2
    88, // i1
    ,
    ,
    ,
    ,
    ,
    86, // g1
    74, // f2
    75, // g2
    64, // g3
  ],
  [
    /* i1 */
    77, // i2
    78, // k2
    89, // k1
    ,
    ,
    ,
    ,
    ,
    87, // h1
    75, // g2
    76, // h2
    65, // h3
  ],
  [
    /* k1 */
    78, // k2
    79, // l2
    90, // l1
    ,
    ,
    ,
    ,
    ,
    88, // i1
    76, // h2
    77, // i2
    66, // i3
  ],
  [
    /* l1 */
    79, // l2
    ,
    ,
    ,
    ,
    ,
    ,
    ,
    89, // k1
    77, // i2
    78, // k2
    67, // k3
  ],
] as const

/** initial position */
export const initialPosition = 'b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1'

/** named positions in fen order */
export const positions = [
  'f11',
  'e10',
  'f10',
  'g10',
  'd9',
  'e9',
  'f9',
  'g9',
  'h9',
  'c8',
  'd8',
  'e8',
  'f8',
  'g8',
  'h8',
  'i8',
  'b7',
  'c7',
  'd7',
  'e7',
  'f7',
  'g7',
  'h7',
  'i7',
  'k7',
  'a6',
  'b6',
  'c6',
  'd6',
  'e6',
  'f6',
  'g6',
  'h6',
  'i6',
  'k6',
  'l6',
  'a5',
  'b5',
  'c5',
  'd5',
  'e5',
  'f5',
  'g5',
  'h5',
  'i5',
  'k5',
  'l5',
  'a4',
  'b4',
  'c4',
  'd4',
  'e4',
  'f4',
  'g4',
  'h4',
  'i4',
  'k4',
  'l4',
  'a3',
  'b3',
  'c3',
  'd3',
  'e3',
  'f3',
  'g3',
  'h3',
  'i3',
  'k3',
  'l3',
  'a2',
  'b2',
  'c2',
  'd2',
  'e2',
  'f2',
  'g2',
  'h2',
  'i2',
  'k2',
  'l2',
  'a1',
  'b1',
  'c1',
  'd1',
  'e1',
  'f1',
  'g1',
  'h1',
  'i1',
  'k1',
  'l1',
] as const
