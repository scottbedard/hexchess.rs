use crate::hex;

use crate::constants::{
    Color,
    HEXBOARD_GRAPH,
    Piece,
};

use crate::hexchess::hexchess::Hexchess;

/// get the color of a piece
pub fn get_color(piece: &Piece) -> Color {
    match piece {
        Piece::WhitePawn => Color::White,
        Piece::WhiteKnight => Color::White,
        Piece::WhiteBishop => Color::White,
        Piece::WhiteRook => Color::White,
        Piece::WhiteQueen => Color::White,
        Piece::WhiteKing => Color::White,
        Piece::BlackPawn => Color::Black,
        Piece::BlackKnight => Color::Black,
        Piece::BlackBishop => Color::Black,
        Piece::BlackRook => Color::Black,
        Piece::BlackQueen => Color::Black,
        Piece::BlackKing => Color::Black,
    }
}

/// test if position is black en passant target
pub fn is_legal_black_en_passant(position: &u8) -> bool {
    match position {
        hex!("b6") |
        hex!("c6") |
        hex!("d6") |
        hex!("e6") |
        hex!("f6") |
        hex!("g6") |
        hex!("h6") |
        hex!("i6") |
        hex!("k6") => true,
        _ => false,
    }
}

/// test if position is white en passant target
pub fn is_legal_white_en_passant_position(position: &u8) -> bool {
    match position {
        hex!("b2") |
        hex!("c3") |
        hex!("d4") |
        hex!("e5") |
        hex!("f6") |
        hex!("g5") |
        hex!("h4") |
        hex!("i3") |
        hex!("k2") => true,
        _ => false,
    }
}

/// test if position is en passant target
pub fn is_legal_en_passant(position: &u8) -> bool {
    is_legal_black_en_passant(position) || is_legal_white_en_passant_position(position)
}

/// test if position is black promotion position
pub fn is_black_promotion_position(position: &u8) -> bool {
    match position {
        hex!("a1") |
        hex!("b1") |
        hex!("c1") |
        hex!("d1") |
        hex!("e1") |
        hex!("f1") |
        hex!("g1") |
        hex!("h1") |
        hex!("i1") |
        hex!("k1") |
        hex!("l1") => true,
        _ => false,
    }
}

/// test if position is on first or last rank
pub fn is_white_promotion_position(position: &u8) -> bool {
    match position {
        hex!("f11") |
        hex!("e10") |
        hex!("g10") |
        hex!("d9") |
        hex!("h9") |
        hex!("c8") |
        hex!("i8") |
        hex!("b7") |
        hex!("k7") |
        hex!("a6") |
        hex!("l6") => true,
        _ => false,
    }
}

/// test if position is a promotion position
pub fn is_promotion_position(position: &u8) -> bool {
    is_black_promotion_position(position) || is_white_promotion_position(position)
}

/// convert position to index
pub fn to_index(source: &str) -> Result<u8, ()> {
    match source {
        "f11" => Ok(0),
        "e10" => Ok(1),
        "f10" => Ok(2),
        "g10" => Ok(3),
        "d9" => Ok(4),
        "e9" => Ok(5),
        "f9" => Ok(6),
        "g9" => Ok(7),
        "h9" => Ok(8),
        "c8" => Ok(9),
        "d8" => Ok(10),
        "e8" => Ok(11),
        "f8" => Ok(12),
        "g8" => Ok(13),
        "h8" => Ok(14),
        "i8" => Ok(15),
        "b7" => Ok(16),
        "c7" => Ok(17),
        "d7" => Ok(18),
        "e7" => Ok(19),
        "f7" => Ok(20),
        "g7" => Ok(21),
        "h7" => Ok(22),
        "i7" => Ok(23),
        "k7" => Ok(24),
        "a6" => Ok(25),
        "b6" => Ok(26),
        "c6" => Ok(27),
        "d6" => Ok(28),
        "e6" => Ok(29),
        "f6" => Ok(30),
        "g6" => Ok(31),
        "h6" => Ok(32),
        "i6" => Ok(33),
        "k6" => Ok(34),
        "l6" => Ok(35),
        "a5" => Ok(36),
        "b5" => Ok(37),
        "c5" => Ok(38),
        "d5" => Ok(39),
        "e5" => Ok(40),
        "f5" => Ok(41),
        "g5" => Ok(42),
        "h5" => Ok(43),
        "i5" => Ok(44),
        "k5" => Ok(45),
        "l5" => Ok(46),
        "a4" => Ok(47),
        "b4" => Ok(48),
        "c4" => Ok(49),
        "d4" => Ok(50),
        "e4" => Ok(51),
        "f4" => Ok(52),
        "g4" => Ok(53),
        "h4" => Ok(54),
        "i4" => Ok(55),
        "k4" => Ok(56),
        "l4" => Ok(57),
        "a3" => Ok(58),
        "b3" => Ok(59),
        "c3" => Ok(60),
        "d3" => Ok(61),
        "e3" => Ok(62),
        "f3" => Ok(63),
        "g3" => Ok(64),
        "h3" => Ok(65),
        "i3" => Ok(66),
        "k3" => Ok(67),
        "l3" => Ok(68),
        "a2" => Ok(69),
        "b2" => Ok(70),
        "c2" => Ok(71),
        "d2" => Ok(72),
        "e2" => Ok(73),
        "f2" => Ok(74),
        "g2" => Ok(75),
        "h2" => Ok(76),
        "i2" => Ok(77),
        "k2" => Ok(78),
        "l2" => Ok(79),
        "a1" => Ok(80),
        "b1" => Ok(81),
        "c1" => Ok(82),
        "d1" => Ok(83),
        "e1" => Ok(84),
        "f1" => Ok(85),
        "g1" => Ok(86),
        "h1" => Ok(87),
        "i1" => Ok(88),
        "k1" => Ok(89),
        "l1" => Ok(90),
        _ => Err(()),
    }
}

pub fn to_position(index: &u8) -> &'static str {
    match index {
        0 => "f11",
        1 => "e10",
        2 => "f10",
        3 => "g10",
        4 => "d9",
        5 => "e9",
        6 => "f9",
        7 => "g9",
        8 => "h9",
        9 => "c8",
        10 => "d8",
        11 => "e8",
        12 => "f8",
        13 => "g8",
        14 => "h8",
        15 => "i8",
        16 => "b7",
        17 => "c7",
        18 => "d7",
        19 => "e7",
        20 => "f7",
        21 => "g7",
        22 => "h7",
        23 => "i7",
        24 => "k7",
        25 => "a6",
        26 => "b6",
        27 => "c6",
        28 => "d6",
        29 => "e6",
        30 => "f6",
        31 => "g6",
        32 => "h6",
        33 => "i6",
        34 => "k6",
        35 => "l6",
        36 => "a5",
        37 => "b5",
        38 => "c5",
        39 => "d5",
        40 => "e5",
        41 => "f5",
        42 => "g5",
        43 => "h5",
        44 => "i5",
        45 => "k5",
        46 => "l5",
        47 => "a4",
        48 => "b4",
        49 => "c4",
        50 => "d4",
        51 => "e4",
        52 => "f4",
        53 => "g4",
        54 => "h4",
        55 => "i4",
        56 => "k4",
        57 => "l4",
        58 => "a3",
        59 => "b3",
        60 => "c3",
        61 => "d3",
        62 => "e3",
        63 => "f3",
        64 => "g3",
        65 => "h3",
        66 => "i3",
        67 => "k3",
        68 => "l3",
        69 => "a2",
        70 => "b2",
        71 => "c2",
        72 => "d2",
        73 => "e2",
        74 => "f2",
        75 => "g2",
        76 => "h2",
        77 => "i2",
        78 => "k2",
        79 => "l2",
        80 => "a1",
        81 => "b1",
        82 => "c1",
        83 => "d1",
        84 => "e1",
        85 => "f1",
        86 => "g1",
        87 => "h1",
        88 => "i1",
        89 => "k1",
        90 => "l1",
        _ => panic!("invalid position index: {}", index),
    }
}

// step along the hexboard graph
pub fn step(from: u8, direction: u8) -> Option<u8> {
    HEXBOARD_GRAPH[from as usize][direction as usize]
}

/// walk along the board in a given direction
pub fn walk(hexchess: &Hexchess, from: u8, direction: u8, color: &Color) -> Vec<u8> {
    let mut path: Vec<u8> = Vec::new();
    let mut position: u8 = from;

    loop {
        position = match HEXBOARD_GRAPH[position as usize][direction as usize] {
            Some(index) => index,
            None => return path // <- end of board
        };

        let piece = match hexchess.board[position as usize] {
            Some(value) => value,
            None => {
                path.push(position); // <- unoccupied position
                continue;
            }
        };

        if get_color(&piece) == *color {
            return path // <- shop short of friendly piece
        }
        
        path.push(position); // <- and captury enemy piece
        return path;
    }
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    mod walk {
        use crate::hex;
        use super::*;

        #[test]
        fn stop_at_board_edge() {
            assert_eq!(
                walk(&Hexchess::new(), hex!("f6"), 0, &Color::White),
                [
                    hex!("f7"),
                    hex!("f8"),
                    hex!("f9"),
                    hex!("f10"),
                    hex!("f11"),
                ]
            );        
        }

        #[test]
        fn stop_before_friendly_piece() {
            let hexchess = Hexchess::from("1/3/2P2/7/9/5R5/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(
                walk(&hexchess, hex!("f6"), 0, &Color::White),
                [
                    hex!("f7"),
                    hex!("f8"),
                    // f9 is a friendly pawn
                ]
            );        
        }

        #[test]
        fn stop_on_hostile_piece() {
            let hexchess = Hexchess::from("1/3/2p2/7/9/5R5/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(
                walk(&hexchess, hex!("f6"), 0, &Color::White),
                [
                    hex!("f7"),
                    hex!("f8"),
                    hex!("f9"), // <- f9 is a hostile pawn
                ]
            );        
        }
    
        #[test]
        fn walk_perimeter_positions_in_all_directions() {
            struct Walk {
                from: u8,
                direction: u8,
                expected: Vec<u8>,
            }

            let hexchess = Hexchess::new();

            let tests = [
                Walk { from: hex!("a1"), direction: 0, expected: vec![hex!("a2"), hex!("a3"), hex!("a4"), hex!("a5"), hex!("a6")] },
                Walk { from: hex!("a1"), direction: 1, expected: vec![hex!("b3"), hex!("c5"), hex!("d7"), hex!("e9"), hex!("f11")] },
                Walk { from: hex!("a1"), direction: 2, expected: vec![hex!("b2"), hex!("c3"), hex!("d4"), hex!("e5"), hex!("f6"), hex!("g6"), hex!("h6"), hex!("i6"), hex!("k6"), hex!("l6")] },
                Walk { from: hex!("a1"), direction: 3, expected: vec![hex!("c2"), hex!("e3"), hex!("g3"), hex!("i2"), hex!("l1")] },
                Walk { from: hex!("a1"), direction: 4, expected: vec![hex!("b1"), hex!("c1"), hex!("d1"), hex!("e1"), hex!("f1")] },
                Walk { from: hex!("a1"), direction: 5, expected: vec![] },
                Walk { from: hex!("a1"), direction: 6, expected: vec![] },
                Walk { from: hex!("a1"), direction: 7, expected: vec![] },
                Walk { from: hex!("a1"), direction: 8, expected: vec![] },
                Walk { from: hex!("a1"), direction: 9, expected: vec![] },
                Walk { from: hex!("a1"), direction: 10, expected: vec![] },
                Walk { from: hex!("a1"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("a2"), direction: 0, expected: vec![hex!("a3"), hex!("a4"), hex!("a5"), hex!("a6")] },
                Walk { from: hex!("a2"), direction: 1, expected: vec![hex!("b4"), hex!("c6"), hex!("d8"), hex!("e10")] },
                Walk { from: hex!("a2"), direction: 2, expected: vec![hex!("b3"), hex!("c4"), hex!("d5"), hex!("e6"), hex!("f7"), hex!("g7"), hex!("h7"), hex!("i7"), hex!("k7")] },
                Walk { from: hex!("a2"), direction: 3, expected: vec![hex!("c3"), hex!("e4"), hex!("g4"), hex!("i3"), hex!("l2")] },
                Walk { from: hex!("a2"), direction: 4, expected: vec![hex!("b2"), hex!("c2"), hex!("d2"), hex!("e2"), hex!("f2"), hex!("g1")] },
                Walk { from: hex!("a2"), direction: 5, expected: vec![hex!("b1")] },
                Walk { from: hex!("a2"), direction: 6, expected: vec![hex!("a1")] },
                Walk { from: hex!("a2"), direction: 7, expected: vec![] },
                Walk { from: hex!("a2"), direction: 8, expected: vec![] },
                Walk { from: hex!("a2"), direction: 9, expected: vec![] },
                Walk { from: hex!("a2"), direction: 10, expected: vec![] },
                Walk { from: hex!("a2"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("a3"), direction: 0, expected: vec![hex!("a4"), hex!("a5"), hex!("a6")] },
                Walk { from: hex!("a3"), direction: 1, expected: vec![hex!("b5"), hex!("c7"), hex!("d9")] },
                Walk { from: hex!("a3"), direction: 2, expected: vec![hex!("b4"), hex!("c5"), hex!("d6"), hex!("e7"), hex!("f8"), hex!("g8"), hex!("h8"), hex!("i8")] },
                Walk { from: hex!("a3"), direction: 3, expected: vec![hex!("c4"), hex!("e5"), hex!("g5"), hex!("i4"), hex!("l3")] },
                Walk { from: hex!("a3"), direction: 4, expected: vec![hex!("b3"), hex!("c3"), hex!("d3"), hex!("e3"), hex!("f3"), hex!("g2"), hex!("h1")] },
                Walk { from: hex!("a3"), direction: 5, expected: vec![hex!("b2"), hex!("c1")] },
                Walk { from: hex!("a3"), direction: 6, expected: vec![hex!("a2"), hex!("a1")] },
                Walk { from: hex!("a3"), direction: 7, expected: vec![] },
                Walk { from: hex!("a3"), direction: 8, expected: vec![] },
                Walk { from: hex!("a3"), direction: 9, expected: vec![] },
                Walk { from: hex!("a3"), direction: 10, expected: vec![] },
                Walk { from: hex!("a3"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("a4"), direction: 0, expected: vec![hex!("a5"), hex!("a6")] },
                Walk { from: hex!("a4"), direction: 1, expected: vec![hex!("b6"), hex!("c8")] },
                Walk { from: hex!("a4"), direction: 2, expected: vec![hex!("b5"), hex!("c6"), hex!("d7"), hex!("e8"), hex!("f9"), hex!("g9"), hex!("h9")] },
                Walk { from: hex!("a4"), direction: 3, expected: vec![hex!("c5"), hex!("e6"), hex!("g6"), hex!("i5"), hex!("l4")] },
                Walk { from: hex!("a4"), direction: 4, expected: vec![hex!("b4"), hex!("c4"), hex!("d4"), hex!("e4"), hex!("f4"), hex!("g3"), hex!("h2"), hex!("i1")] },
                Walk { from: hex!("a4"), direction: 5, expected: vec![hex!("b3"), hex!("c2"), hex!("d1")] },
                Walk { from: hex!("a4"), direction: 6, expected: vec![hex!("a3"), hex!("a2"), hex!("a1")] },
                Walk { from: hex!("a4"), direction: 7, expected: vec![] },
                Walk { from: hex!("a4"), direction: 8, expected: vec![] },
                Walk { from: hex!("a4"), direction: 9, expected: vec![] },
                Walk { from: hex!("a4"), direction: 10, expected: vec![] },
                Walk { from: hex!("a4"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("a5"), direction: 0, expected: vec![hex!("a6")] },
                Walk { from: hex!("a5"), direction: 1, expected: vec![hex!("b7")] },
                Walk { from: hex!("a5"), direction: 2, expected: vec![hex!("b6"), hex!("c7"), hex!("d8"), hex!("e9"), hex!("f10"), hex!("g10")] },
                Walk { from: hex!("a5"), direction: 3, expected: vec![hex!("c6"), hex!("e7"), hex!("g7"), hex!("i6"), hex!("l5")] },
                Walk { from: hex!("a5"), direction: 4, expected: vec![hex!("b5"), hex!("c5"), hex!("d5"), hex!("e5"), hex!("f5"), hex!("g4"), hex!("h3"), hex!("i2"), hex!("k1")] },
                Walk { from: hex!("a5"), direction: 5, expected: vec![hex!("b4"), hex!("c3"), hex!("d2"), hex!("e1")] },
                Walk { from: hex!("a5"), direction: 6, expected: vec![hex!("a4"), hex!("a3"), hex!("a2"), hex!("a1")] },
                Walk { from: hex!("a5"), direction: 7, expected: vec![] },
                Walk { from: hex!("a5"), direction: 8, expected: vec![] },
                Walk { from: hex!("a5"), direction: 9, expected: vec![] },
                Walk { from: hex!("a5"), direction: 10, expected: vec![] },
                Walk { from: hex!("a5"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("a6"), direction: 0, expected: vec![] },
                Walk { from: hex!("a6"), direction: 1, expected: vec![] },
                Walk { from: hex!("a6"), direction: 2, expected: vec![hex!("b7"), hex!("c8"), hex!("d9"), hex!("e10"), hex!("f11")] },
                Walk { from: hex!("a6"), direction: 3, expected: vec![hex!("c7"), hex!("e8"), hex!("g8"), hex!("i7"), hex!("l6")] },
                Walk { from: hex!("a6"), direction: 4, expected: vec![hex!("b6"), hex!("c6"), hex!("d6"), hex!("e6"), hex!("f6"), hex!("g5"), hex!("h4"), hex!("i3"), hex!("k2"), hex!("l1")] },
                Walk { from: hex!("a6"), direction: 5, expected: vec![hex!("b5"), hex!("c4"), hex!("d3"), hex!("e2"), hex!("f1")] },
                Walk { from: hex!("a6"), direction: 6, expected: vec![hex!("a5"), hex!("a4"), hex!("a3"), hex!("a2"), hex!("a1")] },
                Walk { from: hex!("a6"), direction: 7, expected: vec![] },
                Walk { from: hex!("a6"), direction: 8, expected: vec![] },
                Walk { from: hex!("a6"), direction: 9, expected: vec![] },
                Walk { from: hex!("a6"), direction: 10, expected: vec![] },
                Walk { from: hex!("a6"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("b7"), direction: 0, expected: vec![] },
                Walk { from: hex!("b7"), direction: 1, expected: vec![] },
                Walk { from: hex!("b7"), direction: 2, expected: vec![hex!("c8"), hex!("d9"), hex!("e10"), hex!("f11")] },
                Walk { from: hex!("b7"), direction: 3, expected: vec![hex!("d8"), hex!("f9"), hex!("h8"), hex!("k7")] },
                Walk { from: hex!("b7"), direction: 4, expected: vec![hex!("c7"), hex!("d7"), hex!("e7"), hex!("f7"), hex!("g6"), hex!("h5"), hex!("i4"), hex!("k3"), hex!("l2")] },
                Walk { from: hex!("b7"), direction: 5, expected: vec![hex!("c6"), hex!("d5"), hex!("e4"), hex!("f3"), hex!("g1")] },
                Walk { from: hex!("b7"), direction: 6, expected: vec![hex!("b6"), hex!("b5"), hex!("b4"), hex!("b3"), hex!("b2"), hex!("b1")] },
                Walk { from: hex!("b7"), direction: 7, expected: vec![hex!("a5")] },
                Walk { from: hex!("b7"), direction: 8, expected: vec![hex!("a6")] },
                Walk { from: hex!("b7"), direction: 9, expected: vec![] },
                Walk { from: hex!("b7"), direction: 10, expected: vec![] },
                Walk { from: hex!("b7"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("c8"), direction: 0, expected: vec![] },
                Walk { from: hex!("c8"), direction: 1, expected: vec![] },
                Walk { from: hex!("c8"), direction: 2, expected: vec![hex!("d9"), hex!("e10"), hex!("f11")] },
                Walk { from: hex!("c8"), direction: 3, expected: vec![hex!("e9"), hex!("g9"), hex!("i8")] },
                Walk { from: hex!("c8"), direction: 4, expected: vec![hex!("d8"), hex!("e8"), hex!("f8"), hex!("g7"), hex!("h6"), hex!("i5"), hex!("k4"), hex!("l3")] },
                Walk { from: hex!("c8"), direction: 5, expected: vec![hex!("d7"), hex!("e6"), hex!("f5"), hex!("g3"), hex!("h1")] },
                Walk { from: hex!("c8"), direction: 6, expected: vec![hex!("c7"), hex!("c6"), hex!("c5"), hex!("c4"), hex!("c3"), hex!("c2"), hex!("c1")] },
                Walk { from: hex!("c8"), direction: 7, expected: vec![hex!("b6"), hex!("a4")] },
                Walk { from: hex!("c8"), direction: 8, expected: vec![hex!("b7"), hex!("a6")] },
                Walk { from: hex!("c8"), direction: 9, expected: vec![] },
                Walk { from: hex!("c8"), direction: 10, expected: vec![] },
                Walk { from: hex!("c8"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("d9"), direction: 0, expected: vec![] },
                Walk { from: hex!("d9"), direction: 1, expected: vec![] },
                Walk { from: hex!("d9"), direction: 2, expected: vec![hex!("e10"), hex!("f11")] },
                Walk { from: hex!("d9"), direction: 3, expected: vec![hex!("f10"), hex!("h9")] },
                Walk { from: hex!("d9"), direction: 4, expected: vec![hex!("e9"), hex!("f9"), hex!("g8"), hex!("h7"), hex!("i6"), hex!("k5"), hex!("l4")] },
                Walk { from: hex!("d9"), direction: 5, expected: vec![hex!("e8"), hex!("f7"), hex!("g5"), hex!("h3"), hex!("i1")] },
                Walk { from: hex!("d9"), direction: 6, expected: vec![hex!("d8"), hex!("d7"), hex!("d6"), hex!("d5"), hex!("d4"), hex!("d3"), hex!("d2"), hex!("d1")] },
                Walk { from: hex!("d9"), direction: 7, expected: vec![hex!("c7"), hex!("b5"), hex!("a3")] },
                Walk { from: hex!("d9"), direction: 8, expected: vec![hex!("c8"), hex!("b7"), hex!("a6")] },
                Walk { from: hex!("d9"), direction: 9, expected: vec![] },
                Walk { from: hex!("d9"), direction: 10, expected: vec![] },
                Walk { from: hex!("d9"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("e10"), direction: 0, expected: vec![] },
                Walk { from: hex!("e10"), direction: 1, expected: vec![] },
                Walk { from: hex!("e10"), direction: 2, expected: vec![hex!("f11")] },
                Walk { from: hex!("e10"), direction: 3, expected: vec![hex!("g10")] },
                Walk { from: hex!("e10"), direction: 4, expected: vec![hex!("f10"), hex!("g9"), hex!("h8"), hex!("i7"), hex!("k6"), hex!("l5")] },
                Walk { from: hex!("e10"), direction: 5, expected: vec![hex!("f9"), hex!("g7"), hex!("h5"), hex!("i3"), hex!("k1")] },
                Walk { from: hex!("e10"), direction: 6, expected: vec![hex!("e9"), hex!("e8"), hex!("e7"), hex!("e6"), hex!("e5"), hex!("e4"), hex!("e3"), hex!("e2"), hex!("e1")] },
                Walk { from: hex!("e10"), direction: 7, expected: vec![hex!("d8"), hex!("c6"), hex!("b4"), hex!("a2")] },
                Walk { from: hex!("e10"), direction: 8, expected: vec![hex!("d9"), hex!("c8"), hex!("b7"), hex!("a6")] },
                Walk { from: hex!("e10"), direction: 9, expected: vec![] },
                Walk { from: hex!("e10"), direction: 10, expected: vec![] },
                Walk { from: hex!("e10"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("f11"), direction: 0, expected: vec![] },
                Walk { from: hex!("f11"), direction: 1, expected: vec![] },
                Walk { from: hex!("f11"), direction: 2, expected: vec![] },
                Walk { from: hex!("f11"), direction: 3, expected: vec![] },
                Walk { from: hex!("f11"), direction: 4, expected: vec![hex!("g10"), hex!("h9"), hex!("i8"), hex!("k7"), hex!("l6")] },
                Walk { from: hex!("f11"), direction: 5, expected: vec![hex!("g9"), hex!("h7"), hex!("i5"), hex!("k3"), hex!("l1")] },
                Walk { from: hex!("f11"), direction: 6, expected: vec![hex!("f10"), hex!("f9"), hex!("f8"), hex!("f7"), hex!("f6"), hex!("f5"), hex!("f4"), hex!("f3"), hex!("f2"), hex!("f1")] },
                Walk { from: hex!("f11"), direction: 7, expected: vec![hex!("e9"), hex!("d7"), hex!("c5"), hex!("b3"), hex!("a1")] },
                Walk { from: hex!("f11"), direction: 8, expected: vec![hex!("e10"), hex!("d9"), hex!("c8"), hex!("b7"), hex!("a6")] },
                Walk { from: hex!("f11"), direction: 9, expected: vec![] },
                Walk { from: hex!("f11"), direction: 10, expected: vec![] },
                Walk { from: hex!("f11"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("g10"), direction: 0, expected: vec![] },
                Walk { from: hex!("g10"), direction: 1, expected: vec![] },
                Walk { from: hex!("g10"), direction: 2, expected: vec![] },
                Walk { from: hex!("g10"), direction: 3, expected: vec![] },
                Walk { from: hex!("g10"), direction: 4, expected: vec![hex!("h9"), hex!("i8"), hex!("k7"), hex!("l6")] },
                Walk { from: hex!("g10"), direction: 5, expected: vec![hex!("h8"), hex!("i6"), hex!("k4"), hex!("l2")] },
                Walk { from: hex!("g10"), direction: 6, expected: vec![hex!("g9"), hex!("g8"), hex!("g7"), hex!("g6"), hex!("g5"), hex!("g4"), hex!("g3"), hex!("g2"), hex!("g1")] },
                Walk { from: hex!("g10"), direction: 7, expected: vec![hex!("f9"), hex!("e7"), hex!("d5"), hex!("c3"), hex!("b1")] },
                Walk { from: hex!("g10"), direction: 8, expected: vec![hex!("f10"), hex!("e9"), hex!("d8"), hex!("c7"), hex!("b6"), hex!("a5")] },
                Walk { from: hex!("g10"), direction: 9, expected: vec![hex!("e10")] },
                Walk { from: hex!("g10"), direction: 10, expected: vec![hex!("f11")] },
                Walk { from: hex!("g10"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("h9"), direction: 0, expected: vec![] },
                Walk { from: hex!("h9"), direction: 1, expected: vec![] },
                Walk { from: hex!("h9"), direction: 2, expected: vec![] },
                Walk { from: hex!("h9"), direction: 3, expected: vec![] },
                Walk { from: hex!("h9"), direction: 4, expected: vec![hex!("i8"), hex!("k7"), hex!("l6")] },
                Walk { from: hex!("h9"), direction: 5, expected: vec![hex!("i7"), hex!("k5"), hex!("l3")] },
                Walk { from: hex!("h9"), direction: 6, expected: vec![hex!("h8"), hex!("h7"), hex!("h6"), hex!("h5"), hex!("h4"), hex!("h3"), hex!("h2"), hex!("h1")] },
                Walk { from: hex!("h9"), direction: 7, expected: vec![hex!("g8"), hex!("f7"), hex!("e5"), hex!("d3"), hex!("c1")] },
                Walk { from: hex!("h9"), direction: 8, expected: vec![hex!("g9"), hex!("f9"), hex!("e8"), hex!("d7"), hex!("c6"), hex!("b5"), hex!("a4")] },
                Walk { from: hex!("h9"), direction: 9, expected: vec![hex!("f10"), hex!("d9")] },
                Walk { from: hex!("h9"), direction: 10, expected: vec![hex!("g10"), hex!("f11")] },
                Walk { from: hex!("h9"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("i8"), direction: 0, expected: vec![] },
                Walk { from: hex!("i8"), direction: 1, expected: vec![] },
                Walk { from: hex!("i8"), direction: 2, expected: vec![] },
                Walk { from: hex!("i8"), direction: 3, expected: vec![] },
                Walk { from: hex!("i8"), direction: 4, expected: vec![hex!("k7"), hex!("l6")] },
                Walk { from: hex!("i8"), direction: 5, expected: vec![hex!("k6"), hex!("l4")] },
                Walk { from: hex!("i8"), direction: 6, expected: vec![hex!("i7"), hex!("i6"), hex!("i5"), hex!("i4"), hex!("i3"), hex!("i2"), hex!("i1")] },
                Walk { from: hex!("i8"), direction: 7, expected: vec![hex!("h7"), hex!("g6"), hex!("f5"), hex!("e3"), hex!("d1")] },
                Walk { from: hex!("i8"), direction: 8, expected: vec![hex!("h8"), hex!("g8"), hex!("f8"), hex!("e7"), hex!("d6"), hex!("c5"), hex!("b4"), hex!("a3")] },
                Walk { from: hex!("i8"), direction: 9, expected: vec![hex!("g9"), hex!("e9"), hex!("c8")] },
                Walk { from: hex!("i8"), direction: 10, expected: vec![hex!("h9"), hex!("g10"), hex!("f11")] },
                Walk { from: hex!("i8"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("k7"), direction: 0, expected: vec![] },
                Walk { from: hex!("k7"), direction: 1, expected: vec![] },
                Walk { from: hex!("k7"), direction: 2, expected: vec![] },
                Walk { from: hex!("k7"), direction: 3, expected: vec![] },
                Walk { from: hex!("k7"), direction: 4, expected: vec![hex!("l6")] },
                Walk { from: hex!("k7"), direction: 5, expected: vec![hex!("l5")] },
                Walk { from: hex!("k7"), direction: 6, expected: vec![hex!("k6"), hex!("k5"), hex!("k4"), hex!("k3"), hex!("k2"), hex!("k1")] },
                Walk { from: hex!("k7"), direction: 7, expected: vec![hex!("i6"), hex!("h5"), hex!("g4"), hex!("f3"), hex!("e1")] },
                Walk { from: hex!("k7"), direction: 8, expected: vec![hex!("i7"), hex!("h7"), hex!("g7"), hex!("f7"), hex!("e6"), hex!("d5"), hex!("c4"), hex!("b3"), hex!("a2")] },
                Walk { from: hex!("k7"), direction: 9, expected: vec![hex!("h8"), hex!("f9"), hex!("d8"), hex!("b7")] },
                Walk { from: hex!("k7"), direction: 10, expected: vec![hex!("i8"), hex!("h9"), hex!("g10"), hex!("f11")] },
                Walk { from: hex!("k7"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("l6"), direction: 0, expected: vec![] },
                Walk { from: hex!("l6"), direction: 1, expected: vec![] },
                Walk { from: hex!("l6"), direction: 2, expected: vec![] },
                Walk { from: hex!("l6"), direction: 3, expected: vec![] },
                Walk { from: hex!("l6"), direction: 4, expected: vec![] },
                Walk { from: hex!("l6"), direction: 5, expected: vec![] },
                Walk { from: hex!("l6"), direction: 6, expected: vec![hex!("l5"), hex!("l4"), hex!("l3"), hex!("l2"), hex!("l1")] },
                Walk { from: hex!("l6"), direction: 7, expected: vec![hex!("k5"), hex!("i4"), hex!("h3"), hex!("g2"), hex!("f1")] },
                Walk { from: hex!("l6"), direction: 8, expected: vec![hex!("k6"), hex!("i6"), hex!("h6"), hex!("g6"), hex!("f6"), hex!("e5"), hex!("d4"), hex!("c3"), hex!("b2"), hex!("a1")] },
                Walk { from: hex!("l6"), direction: 9, expected: vec![hex!("i7"), hex!("g8"), hex!("e8"), hex!("c7"), hex!("a6")] },
                Walk { from: hex!("l6"), direction: 10, expected: vec![hex!("k7"), hex!("i8"), hex!("h9"), hex!("g10"), hex!("f11")] },
                Walk { from: hex!("l6"), direction: 11, expected: vec![] },
            
                Walk { from: hex!("l5"), direction: 0, expected: vec![hex!("l6")] },
                Walk { from: hex!("l5"), direction: 1, expected: vec![] },
                Walk { from: hex!("l5"), direction: 2, expected: vec![] },
                Walk { from: hex!("l5"), direction: 3, expected: vec![] },
                Walk { from: hex!("l5"), direction: 4, expected: vec![] },
                Walk { from: hex!("l5"), direction: 5, expected: vec![] },
                Walk { from: hex!("l5"), direction: 6, expected: vec![hex!("l4"), hex!("l3"), hex!("l2"), hex!("l1")] },
                Walk { from: hex!("l5"), direction: 7, expected: vec![hex!("k4"), hex!("i3"), hex!("h2"), hex!("g1")] },
                Walk { from: hex!("l5"), direction: 8, expected: vec![hex!("k5"), hex!("i5"), hex!("h5"), hex!("g5"), hex!("f5"), hex!("e4"), hex!("d3"), hex!("c2"), hex!("b1")] },
                Walk { from: hex!("l5"), direction: 9, expected: vec![hex!("i6"), hex!("g7"), hex!("e7"), hex!("c6"), hex!("a5")] },
                Walk { from: hex!("l5"), direction: 10, expected: vec![hex!("k6"), hex!("i7"), hex!("h8"), hex!("g9"), hex!("f10"), hex!("e10")] },
                Walk { from: hex!("l5"), direction: 11, expected: vec![hex!("k7")] },
            
                Walk { from: hex!("l4"), direction: 0, expected: vec![hex!("l5"), hex!("l6")] },
                Walk { from: hex!("l4"), direction: 1, expected: vec![] },
                Walk { from: hex!("l4"), direction: 2, expected: vec![] },
                Walk { from: hex!("l4"), direction: 3, expected: vec![] },
                Walk { from: hex!("l4"), direction: 4, expected: vec![] },
                Walk { from: hex!("l4"), direction: 5, expected: vec![] },
                Walk { from: hex!("l4"), direction: 6, expected: vec![hex!("l3"), hex!("l2"), hex!("l1")] },
                Walk { from: hex!("l4"), direction: 7, expected: vec![hex!("k3"), hex!("i2"), hex!("h1")] },
                Walk { from: hex!("l4"), direction: 8, expected: vec![hex!("k4"), hex!("i4"), hex!("h4"), hex!("g4"), hex!("f4"), hex!("e3"), hex!("d2"), hex!("c1")] },
                Walk { from: hex!("l4"), direction: 9, expected: vec![hex!("i5"), hex!("g6"), hex!("e6"), hex!("c5"), hex!("a4")] },
                Walk { from: hex!("l4"), direction: 10, expected: vec![hex!("k5"), hex!("i6"), hex!("h7"), hex!("g8"), hex!("f9"), hex!("e9"), hex!("d9")] },
                Walk { from: hex!("l4"), direction: 11, expected: vec![hex!("k6"), hex!("i8")] },
            
                Walk { from: hex!("l3"), direction: 0, expected: vec![hex!("l4"), hex!("l5"), hex!("l6")] },
                Walk { from: hex!("l3"), direction: 1, expected: vec![] },
                Walk { from: hex!("l3"), direction: 2, expected: vec![] },
                Walk { from: hex!("l3"), direction: 3, expected: vec![] },
                Walk { from: hex!("l3"), direction: 4, expected: vec![] },
                Walk { from: hex!("l3"), direction: 5, expected: vec![] },
                Walk { from: hex!("l3"), direction: 6, expected: vec![hex!("l2"), hex!("l1")] },
                Walk { from: hex!("l3"), direction: 7, expected: vec![hex!("k2"), hex!("i1")] },
                Walk { from: hex!("l3"), direction: 8, expected: vec![hex!("k3"), hex!("i3"), hex!("h3"), hex!("g3"), hex!("f3"), hex!("e2"), hex!("d1")] },
                Walk { from: hex!("l3"), direction: 9, expected: vec![hex!("i4"), hex!("g5"), hex!("e5"), hex!("c4"), hex!("a3")] },
                Walk { from: hex!("l3"), direction: 10, expected: vec![hex!("k4"), hex!("i5"), hex!("h6"), hex!("g7"), hex!("f8"), hex!("e8"), hex!("d8"), hex!("c8")] },
                Walk { from: hex!("l3"), direction: 11, expected: vec![hex!("k5"), hex!("i7"), hex!("h9")] },
            
                Walk { from: hex!("l2"), direction: 0, expected: vec![hex!("l3"), hex!("l4"), hex!("l5"), hex!("l6")] },
                Walk { from: hex!("l2"), direction: 1, expected: vec![] },
                Walk { from: hex!("l2"), direction: 2, expected: vec![] },
                Walk { from: hex!("l2"), direction: 3, expected: vec![] },
                Walk { from: hex!("l2"), direction: 4, expected: vec![] },
                Walk { from: hex!("l2"), direction: 5, expected: vec![] },
                Walk { from: hex!("l2"), direction: 6, expected: vec![hex!("l1")] },
                Walk { from: hex!("l2"), direction: 7, expected: vec![hex!("k1")] },
                Walk { from: hex!("l2"), direction: 8, expected: vec![hex!("k2"), hex!("i2"), hex!("h2"), hex!("g2"), hex!("f2"), hex!("e1")] },
                Walk { from: hex!("l2"), direction: 9, expected: vec![hex!("i3"), hex!("g4"), hex!("e4"), hex!("c3"), hex!("a2")] },
                Walk { from: hex!("l2"), direction: 10, expected: vec![hex!("k3"), hex!("i4"), hex!("h5"), hex!("g6"), hex!("f7"), hex!("e7"), hex!("d7"), hex!("c7"), hex!("b7")] },
                Walk { from: hex!("l2"), direction: 11, expected: vec![hex!("k4"), hex!("i6"), hex!("h8"), hex!("g10")] },
            
                Walk { from: hex!("l1"), direction: 0, expected: vec![hex!("l2"), hex!("l3"), hex!("l4"), hex!("l5"), hex!("l6")] },
                Walk { from: hex!("l1"), direction: 1, expected: vec![] },
                Walk { from: hex!("l1"), direction: 2, expected: vec![] },
                Walk { from: hex!("l1"), direction: 3, expected: vec![] },
                Walk { from: hex!("l1"), direction: 4, expected: vec![] },
                Walk { from: hex!("l1"), direction: 5, expected: vec![] },
                Walk { from: hex!("l1"), direction: 6, expected: vec![] },
                Walk { from: hex!("l1"), direction: 7, expected: vec![] },
                Walk { from: hex!("l1"), direction: 8, expected: vec![hex!("k1"), hex!("i1"), hex!("h1"), hex!("g1"), hex!("f1")] },
                Walk { from: hex!("l1"), direction: 9, expected: vec![hex!("i2"), hex!("g3"), hex!("e3"), hex!("c2"), hex!("a1")] },
                Walk { from: hex!("l1"), direction: 10, expected: vec![hex!("k2"), hex!("i3"), hex!("h4"), hex!("g5"), hex!("f6"), hex!("e6"), hex!("d6"), hex!("c6"), hex!("b6"), hex!("a6")] },
                Walk { from: hex!("l1"), direction: 11, expected: vec![hex!("k3"), hex!("i5"), hex!("h7"), hex!("g9"), hex!("f11")] },
            
                Walk { from: hex!("k1"), direction: 0, expected: vec![hex!("k2"), hex!("k3"), hex!("k4"), hex!("k5"), hex!("k6"), hex!("k7")] },
                Walk { from: hex!("k1"), direction: 1, expected: vec![hex!("l2")] },
                Walk { from: hex!("k1"), direction: 2, expected: vec![hex!("l1")] },
                Walk { from: hex!("k1"), direction: 3, expected: vec![] },
                Walk { from: hex!("k1"), direction: 4, expected: vec![] },
                Walk { from: hex!("k1"), direction: 5, expected: vec![] },
                Walk { from: hex!("k1"), direction: 6, expected: vec![] },
                Walk { from: hex!("k1"), direction: 7, expected: vec![] },
                Walk { from: hex!("k1"), direction: 8, expected: vec![hex!("i1"), hex!("h1"), hex!("g1"), hex!("f1")] },
                Walk { from: hex!("k1"), direction: 9, expected: vec![hex!("h2"), hex!("f3"), hex!("d2"), hex!("b1")] },
                Walk { from: hex!("k1"), direction: 10, expected: vec![hex!("i2"), hex!("h3"), hex!("g4"), hex!("f5"), hex!("e5"), hex!("d5"), hex!("c5"), hex!("b5"), hex!("a5")] },
                Walk { from: hex!("k1"), direction: 11, expected: vec![hex!("i3"), hex!("h5"), hex!("g7"), hex!("f9"), hex!("e10")] },
            
                Walk { from: hex!("i1"), direction: 0, expected: vec![hex!("i2"), hex!("i3"), hex!("i4"), hex!("i5"), hex!("i6"), hex!("i7"), hex!("i8")] },
                Walk { from: hex!("i1"), direction: 1, expected: vec![hex!("k2"), hex!("l3")] },
                Walk { from: hex!("i1"), direction: 2, expected: vec![hex!("k1"), hex!("l1")] },
                Walk { from: hex!("i1"), direction: 3, expected: vec![] },
                Walk { from: hex!("i1"), direction: 4, expected: vec![] },
                Walk { from: hex!("i1"), direction: 5, expected: vec![] },
                Walk { from: hex!("i1"), direction: 6, expected: vec![] },
                Walk { from: hex!("i1"), direction: 7, expected: vec![] },
                Walk { from: hex!("i1"), direction: 8, expected: vec![hex!("h1"), hex!("g1"), hex!("f1")] },
                Walk { from: hex!("i1"), direction: 9, expected: vec![hex!("g2"), hex!("e2"), hex!("c1")] },
                Walk { from: hex!("i1"), direction: 10, expected: vec![hex!("h2"), hex!("g3"), hex!("f4"), hex!("e4"), hex!("d4"), hex!("c4"), hex!("b4"), hex!("a4")] },
                Walk { from: hex!("i1"), direction: 11, expected: vec![hex!("h3"), hex!("g5"), hex!("f7"), hex!("e8"), hex!("d9")] },
            
                Walk { from: hex!("h1"), direction: 0, expected: vec![hex!("h2"), hex!("h3"), hex!("h4"), hex!("h5"), hex!("h6"), hex!("h7"), hex!("h8"), hex!("h9")] },
                Walk { from: hex!("h1"), direction: 1, expected: vec![hex!("i2"), hex!("k3"), hex!("l4")] },
                Walk { from: hex!("h1"), direction: 2, expected: vec![hex!("i1"), hex!("k1"), hex!("l1")] },
                Walk { from: hex!("h1"), direction: 3, expected: vec![] },
                Walk { from: hex!("h1"), direction: 4, expected: vec![] },
                Walk { from: hex!("h1"), direction: 5, expected: vec![] },
                Walk { from: hex!("h1"), direction: 6, expected: vec![] },
                Walk { from: hex!("h1"), direction: 7, expected: vec![] },
                Walk { from: hex!("h1"), direction: 8, expected: vec![hex!("g1"), hex!("f1")] },
                Walk { from: hex!("h1"), direction: 9, expected: vec![hex!("f2"), hex!("d1")] },
                Walk { from: hex!("h1"), direction: 10, expected: vec![hex!("g2"), hex!("f3"), hex!("e3"), hex!("d3"), hex!("c3"), hex!("b3"), hex!("a3")] },
                Walk { from: hex!("h1"), direction: 11, expected: vec![hex!("g3"), hex!("f5"), hex!("e6"), hex!("d7"), hex!("c8")] },
            
                Walk { from: hex!("g1"), direction: 0, expected: vec![hex!("g2"), hex!("g3"), hex!("g4"), hex!("g5"), hex!("g6"), hex!("g7"), hex!("g8"), hex!("g9"), hex!("g10")] },
                Walk { from: hex!("g1"), direction: 1, expected: vec![hex!("h2"), hex!("i3"), hex!("k4"), hex!("l5")] },
                Walk { from: hex!("g1"), direction: 2, expected: vec![hex!("h1"), hex!("i1"), hex!("k1"), hex!("l1")] },
                Walk { from: hex!("g1"), direction: 3, expected: vec![] },
                Walk { from: hex!("g1"), direction: 4, expected: vec![] },
                Walk { from: hex!("g1"), direction: 5, expected: vec![] },
                Walk { from: hex!("g1"), direction: 6, expected: vec![] },
                Walk { from: hex!("g1"), direction: 7, expected: vec![] },
                Walk { from: hex!("g1"), direction: 8, expected: vec![hex!("f1")] },
                Walk { from: hex!("g1"), direction: 9, expected: vec![hex!("e1")] },
                Walk { from: hex!("g1"), direction: 10, expected: vec![hex!("f2"), hex!("e2"), hex!("d2"), hex!("c2"), hex!("b2"), hex!("a2")] },
                Walk { from: hex!("g1"), direction: 11, expected: vec![hex!("f3"), hex!("e4"), hex!("d5"), hex!("c6"), hex!("b7")] },
            
                Walk { from: hex!("f1"), direction: 0, expected: vec![hex!("f2"), hex!("f3"), hex!("f4"), hex!("f5"), hex!("f6"), hex!("f7"), hex!("f8"), hex!("f9"), hex!("f10"), hex!("f11")] },
                Walk { from: hex!("f1"), direction: 1, expected: vec![hex!("g2"), hex!("h3"), hex!("i4"), hex!("k5"), hex!("l6")] },
                Walk { from: hex!("f1"), direction: 2, expected: vec![hex!("g1"), hex!("h1"), hex!("i1"), hex!("k1"), hex!("l1")] },
                Walk { from: hex!("f1"), direction: 3, expected: vec![] },
                Walk { from: hex!("f1"), direction: 4, expected: vec![] },
                Walk { from: hex!("f1"), direction: 5, expected: vec![] },
                Walk { from: hex!("f1"), direction: 6, expected: vec![] },
                Walk { from: hex!("f1"), direction: 7, expected: vec![] },
                Walk { from: hex!("f1"), direction: 8, expected: vec![] },
                Walk { from: hex!("f1"), direction: 9, expected: vec![] },
                Walk { from: hex!("f1"), direction: 10, expected: vec![hex!("e1"), hex!("d1"), hex!("c1"), hex!("b1"), hex!("a1")] },
                Walk { from: hex!("f1"), direction: 11, expected: vec![hex!("e2"), hex!("d3"), hex!("c4"), hex!("b5"), hex!("a6")] },
            
                Walk { from: hex!("e1"), direction: 0, expected: vec![hex!("e2"), hex!("e3"), hex!("e4"), hex!("e5"), hex!("e6"), hex!("e7"), hex!("e8"), hex!("e9"), hex!("e10")] },
                Walk { from: hex!("e1"), direction: 1, expected: vec![hex!("f3"), hex!("g4"), hex!("h5"), hex!("i6"), hex!("k7")] },
                Walk { from: hex!("e1"), direction: 2, expected: vec![hex!("f2"), hex!("g2"), hex!("h2"), hex!("i2"), hex!("k2"), hex!("l2")] },
                Walk { from: hex!("e1"), direction: 3, expected: vec![hex!("g1")] },
                Walk { from: hex!("e1"), direction: 4, expected: vec![hex!("f1")] },
                Walk { from: hex!("e1"), direction: 5, expected: vec![] },
                Walk { from: hex!("e1"), direction: 6, expected: vec![] },
                Walk { from: hex!("e1"), direction: 7, expected: vec![] },
                Walk { from: hex!("e1"), direction: 8, expected: vec![] },
                Walk { from: hex!("e1"), direction: 9, expected: vec![] },
                Walk { from: hex!("e1"), direction: 10, expected: vec![hex!("d1"), hex!("c1"), hex!("b1"), hex!("a1")] },
                Walk { from: hex!("e1"), direction: 11, expected: vec![hex!("d2"), hex!("c3"), hex!("b4"), hex!("a5")] },
            
                Walk { from: hex!("d1"), direction: 0, expected: vec![hex!("d2"), hex!("d3"), hex!("d4"), hex!("d5"), hex!("d6"), hex!("d7"), hex!("d8"), hex!("d9")] },
                Walk { from: hex!("d1"), direction: 1, expected: vec![hex!("e3"), hex!("f5"), hex!("g6"), hex!("h7"), hex!("i8")] },
                Walk { from: hex!("d1"), direction: 2, expected: vec![hex!("e2"), hex!("f3"), hex!("g3"), hex!("h3"), hex!("i3"), hex!("k3"), hex!("l3")] },
                Walk { from: hex!("d1"), direction: 3, expected: vec![hex!("f2"), hex!("h1")] },
                Walk { from: hex!("d1"), direction: 4, expected: vec![hex!("e1"), hex!("f1")] },
                Walk { from: hex!("d1"), direction: 5, expected: vec![] },
                Walk { from: hex!("d1"), direction: 6, expected: vec![] },
                Walk { from: hex!("d1"), direction: 7, expected: vec![] },
                Walk { from: hex!("d1"), direction: 8, expected: vec![] },
                Walk { from: hex!("d1"), direction: 9, expected: vec![] },
                Walk { from: hex!("d1"), direction: 10, expected: vec![hex!("c1"), hex!("b1"), hex!("a1")] },
                Walk { from: hex!("d1"), direction: 11, expected: vec![hex!("c2"), hex!("b3"), hex!("a4")] },
            
                Walk { from: hex!("c1"), direction: 0, expected: vec![hex!("c2"), hex!("c3"), hex!("c4"), hex!("c5"), hex!("c6"), hex!("c7"), hex!("c8")] },
                Walk { from: hex!("c1"), direction: 1, expected: vec![hex!("d3"), hex!("e5"), hex!("f7"), hex!("g8"), hex!("h9")] },
                Walk { from: hex!("c1"), direction: 2, expected: vec![hex!("d2"), hex!("e3"), hex!("f4"), hex!("g4"), hex!("h4"), hex!("i4"), hex!("k4"), hex!("l4")] },
                Walk { from: hex!("c1"), direction: 3, expected: vec![hex!("e2"), hex!("g2"), hex!("i1")] },
                Walk { from: hex!("c1"), direction: 4, expected: vec![hex!("d1"), hex!("e1"), hex!("f1")] },
                Walk { from: hex!("c1"), direction: 5, expected: vec![] },
                Walk { from: hex!("c1"), direction: 6, expected: vec![] },
                Walk { from: hex!("c1"), direction: 7, expected: vec![] },
                Walk { from: hex!("c1"), direction: 8, expected: vec![] },
                Walk { from: hex!("c1"), direction: 9, expected: vec![] },
                Walk { from: hex!("c1"), direction: 10, expected: vec![hex!("b1"), hex!("a1")] },
                Walk { from: hex!("c1"), direction: 11, expected: vec![hex!("b2"), hex!("a3")] },
            
                Walk { from: hex!("b1"), direction: 0, expected: vec![hex!("b2"), hex!("b3"), hex!("b4"), hex!("b5"), hex!("b6"), hex!("b7")] },
                Walk { from: hex!("b1"), direction: 1, expected: vec![hex!("c3"), hex!("d5"), hex!("e7"), hex!("f9"), hex!("g10")] },
                Walk { from: hex!("b1"), direction: 2, expected: vec![hex!("c2"), hex!("d3"), hex!("e4"), hex!("f5"), hex!("g5"), hex!("h5"), hex!("i5"), hex!("k5"), hex!("l5")] },
                Walk { from: hex!("b1"), direction: 3, expected: vec![hex!("d2"), hex!("f3"), hex!("h2"), hex!("k1")] },
                Walk { from: hex!("b1"), direction: 4, expected: vec![hex!("c1"), hex!("d1"), hex!("e1"), hex!("f1")] },
                Walk { from: hex!("b1"), direction: 5, expected: vec![] },
                Walk { from: hex!("b1"), direction: 6, expected: vec![] },
                Walk { from: hex!("b1"), direction: 7, expected: vec![] },
                Walk { from: hex!("b1"), direction: 8, expected: vec![] },
                Walk { from: hex!("b1"), direction: 9, expected: vec![] },
                Walk { from: hex!("b1"), direction: 10, expected: vec![hex!("a1")] },
                Walk { from: hex!("b1"), direction: 11, expected: vec![hex!("a2")] },
            ];

            for test in tests.iter() {
                assert_eq!(
                    walk(&hexchess, test.from, test.direction, &Color::White),
                    test.expected
                );       
            }
        }
    }
}
