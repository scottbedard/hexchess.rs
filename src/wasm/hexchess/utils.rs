use crate::h;

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
        h!("b6") |
        h!("c6") |
        h!("d6") |
        h!("e6") |
        h!("f6") |
        h!("g6") |
        h!("h6") |
        h!("i6") |
        h!("k6") => true,
        _ => false,
    }
}

/// test if position is white en passant target
pub fn is_legal_white_en_passant_position(position: &u8) -> bool {
    match position {
        h!("b2") |
        h!("c3") |
        h!("d4") |
        h!("e5") |
        h!("f6") |
        h!("g5") |
        h!("h4") |
        h!("i3") |
        h!("k2") => true,
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
        h!("a1") |
        h!("b1") |
        h!("c1") |
        h!("d1") |
        h!("e1") |
        h!("f1") |
        h!("g1") |
        h!("h1") |
        h!("i1") |
        h!("k1") |
        h!("l1") => true,
        _ => false,
    }
}

/// test if position is on first or last rank
pub fn is_white_promotion_position(position: &u8) -> bool {
    match position {
        h!("f11") |
        h!("e10") |
        h!("g10") |
        h!("d9") |
        h!("h9") |
        h!("c8") |
        h!("i8") |
        h!("b7") |
        h!("k7") |
        h!("a6") |
        h!("l6") => true,
        _ => false,
    }
}

/// test if position is a promotion position
pub fn is_promotion_position(position: &u8) -> bool {
    is_black_promotion_position(position) || is_white_promotion_position(position)
}

/// convert position to index
pub fn index(source: &str) -> Result<u8, ()> {
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

/// convert index to position name
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
        position = match step(position, direction) {
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
    use super::*;

    #[test]
    fn test_to_position() {
        assert_eq!(to_position(&0), "f11");
        assert_eq!(to_position(&1), "e10");
        assert_eq!(to_position(&2), "f10");
        assert_eq!(to_position(&3), "g10");
        assert_eq!(to_position(&4), "d9");
        assert_eq!(to_position(&5), "e9");
        assert_eq!(to_position(&6), "f9");
        assert_eq!(to_position(&7), "g9");
        assert_eq!(to_position(&8), "h9");
        assert_eq!(to_position(&9), "c8");
        assert_eq!(to_position(&10), "d8");
        assert_eq!(to_position(&11), "e8");
        assert_eq!(to_position(&12), "f8");
        assert_eq!(to_position(&13), "g8");
        assert_eq!(to_position(&14), "h8");
        assert_eq!(to_position(&15), "i8");
        assert_eq!(to_position(&16), "b7");
        assert_eq!(to_position(&17), "c7");
        assert_eq!(to_position(&18), "d7");
        assert_eq!(to_position(&19), "e7");
        assert_eq!(to_position(&20), "f7");
        assert_eq!(to_position(&21), "g7");
        assert_eq!(to_position(&22), "h7");
        assert_eq!(to_position(&23), "i7");
        assert_eq!(to_position(&24), "k7");
        assert_eq!(to_position(&25), "a6");
        assert_eq!(to_position(&26), "b6");
        assert_eq!(to_position(&27), "c6");
        assert_eq!(to_position(&28), "d6");
        assert_eq!(to_position(&29), "e6");
        assert_eq!(to_position(&30), "f6");
        assert_eq!(to_position(&31), "g6");
        assert_eq!(to_position(&32), "h6");
        assert_eq!(to_position(&33), "i6");
        assert_eq!(to_position(&34), "k6");
        assert_eq!(to_position(&35), "l6");
        assert_eq!(to_position(&36), "a5");
        assert_eq!(to_position(&37), "b5");
        assert_eq!(to_position(&38), "c5");
        assert_eq!(to_position(&39), "d5");
        assert_eq!(to_position(&40), "e5");
        assert_eq!(to_position(&41), "f5");
        assert_eq!(to_position(&42), "g5");
        assert_eq!(to_position(&43), "h5");
        assert_eq!(to_position(&44), "i5");
        assert_eq!(to_position(&45), "k5");
        assert_eq!(to_position(&46), "l5");
        assert_eq!(to_position(&47), "a4");
        assert_eq!(to_position(&48), "b4");
        assert_eq!(to_position(&49), "c4");
        assert_eq!(to_position(&50), "d4");
        assert_eq!(to_position(&51), "e4");
        assert_eq!(to_position(&52), "f4");
        assert_eq!(to_position(&53), "g4");
        assert_eq!(to_position(&54), "h4");
        assert_eq!(to_position(&55), "i4");
        assert_eq!(to_position(&56), "k4");
        assert_eq!(to_position(&57), "l4");
        assert_eq!(to_position(&58), "a3");
        assert_eq!(to_position(&59), "b3");
        assert_eq!(to_position(&60), "c3");
        assert_eq!(to_position(&61), "d3");
        assert_eq!(to_position(&62), "e3");
        assert_eq!(to_position(&63), "f3");
        assert_eq!(to_position(&64), "g3");
        assert_eq!(to_position(&65), "h3");
        assert_eq!(to_position(&66), "i3");
        assert_eq!(to_position(&67), "k3");
        assert_eq!(to_position(&68), "l3");
        assert_eq!(to_position(&69), "a2");
        assert_eq!(to_position(&70), "b2");
        assert_eq!(to_position(&71), "c2");
        assert_eq!(to_position(&72), "d2");
        assert_eq!(to_position(&73), "e2");
        assert_eq!(to_position(&74), "f2");
        assert_eq!(to_position(&75), "g2");
        assert_eq!(to_position(&76), "h2");
        assert_eq!(to_position(&77), "i2");
        assert_eq!(to_position(&78), "k2");
        assert_eq!(to_position(&79), "l2");
        assert_eq!(to_position(&80), "a1");
        assert_eq!(to_position(&81), "b1");
        assert_eq!(to_position(&82), "c1");
        assert_eq!(to_position(&83), "d1");
        assert_eq!(to_position(&84), "e1");
        assert_eq!(to_position(&85), "f1");
        assert_eq!(to_position(&86), "g1");
        assert_eq!(to_position(&87), "h1");
        assert_eq!(to_position(&88), "i1");
        assert_eq!(to_position(&89), "k1");
        assert_eq!(to_position(&90), "l1");
    }

    #[test]
    #[should_panic]
    fn test_to_position_panics_on_out_of_bounds() {
        to_position(&91);
    }

    mod walk {
        use crate::h;
        use super::*;

        #[test]
        fn stop_at_board_edge() {
            assert_eq!(
                walk(&Hexchess::new(), h!("f6"), 0, &Color::White),
                [
                    h!("f7"),
                    h!("f8"),
                    h!("f9"),
                    h!("f10"),
                    h!("f11"),
                ]
            );        
        }

        #[test]
        fn stop_before_friendly_piece() {
            let hexchess = Hexchess::parse("1/3/2P2/7/9/5R5/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(
                walk(&hexchess, h!("f6"), 0, &Color::White),
                [
                    h!("f7"),
                    h!("f8"),
                    // f9 is a friendly pawn
                ]
            );        
        }

        #[test]
        fn stop_on_hostile_piece() {
            let hexchess = Hexchess::parse("1/3/2p2/7/9/5R5/11/11/11/11/11 w - 0 1").unwrap();

            assert_eq!(
                walk(&hexchess, h!("f6"), 0, &Color::White),
                [
                    h!("f7"),
                    h!("f8"),
                    h!("f9"), // <- f9 is a hostile pawn
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
                Walk { from: h!("a1"), direction: 0, expected: vec![h!("a2"), h!("a3"), h!("a4"), h!("a5"), h!("a6")] },
                Walk { from: h!("a1"), direction: 1, expected: vec![h!("b3"), h!("c5"), h!("d7"), h!("e9"), h!("f11")] },
                Walk { from: h!("a1"), direction: 2, expected: vec![h!("b2"), h!("c3"), h!("d4"), h!("e5"), h!("f6"), h!("g6"), h!("h6"), h!("i6"), h!("k6"), h!("l6")] },
                Walk { from: h!("a1"), direction: 3, expected: vec![h!("c2"), h!("e3"), h!("g3"), h!("i2"), h!("l1")] },
                Walk { from: h!("a1"), direction: 4, expected: vec![h!("b1"), h!("c1"), h!("d1"), h!("e1"), h!("f1")] },
                Walk { from: h!("a1"), direction: 5, expected: vec![] },
                Walk { from: h!("a1"), direction: 6, expected: vec![] },
                Walk { from: h!("a1"), direction: 7, expected: vec![] },
                Walk { from: h!("a1"), direction: 8, expected: vec![] },
                Walk { from: h!("a1"), direction: 9, expected: vec![] },
                Walk { from: h!("a1"), direction: 10, expected: vec![] },
                Walk { from: h!("a1"), direction: 11, expected: vec![] },
            
                Walk { from: h!("a2"), direction: 0, expected: vec![h!("a3"), h!("a4"), h!("a5"), h!("a6")] },
                Walk { from: h!("a2"), direction: 1, expected: vec![h!("b4"), h!("c6"), h!("d8"), h!("e10")] },
                Walk { from: h!("a2"), direction: 2, expected: vec![h!("b3"), h!("c4"), h!("d5"), h!("e6"), h!("f7"), h!("g7"), h!("h7"), h!("i7"), h!("k7")] },
                Walk { from: h!("a2"), direction: 3, expected: vec![h!("c3"), h!("e4"), h!("g4"), h!("i3"), h!("l2")] },
                Walk { from: h!("a2"), direction: 4, expected: vec![h!("b2"), h!("c2"), h!("d2"), h!("e2"), h!("f2"), h!("g1")] },
                Walk { from: h!("a2"), direction: 5, expected: vec![h!("b1")] },
                Walk { from: h!("a2"), direction: 6, expected: vec![h!("a1")] },
                Walk { from: h!("a2"), direction: 7, expected: vec![] },
                Walk { from: h!("a2"), direction: 8, expected: vec![] },
                Walk { from: h!("a2"), direction: 9, expected: vec![] },
                Walk { from: h!("a2"), direction: 10, expected: vec![] },
                Walk { from: h!("a2"), direction: 11, expected: vec![] },
            
                Walk { from: h!("a3"), direction: 0, expected: vec![h!("a4"), h!("a5"), h!("a6")] },
                Walk { from: h!("a3"), direction: 1, expected: vec![h!("b5"), h!("c7"), h!("d9")] },
                Walk { from: h!("a3"), direction: 2, expected: vec![h!("b4"), h!("c5"), h!("d6"), h!("e7"), h!("f8"), h!("g8"), h!("h8"), h!("i8")] },
                Walk { from: h!("a3"), direction: 3, expected: vec![h!("c4"), h!("e5"), h!("g5"), h!("i4"), h!("l3")] },
                Walk { from: h!("a3"), direction: 4, expected: vec![h!("b3"), h!("c3"), h!("d3"), h!("e3"), h!("f3"), h!("g2"), h!("h1")] },
                Walk { from: h!("a3"), direction: 5, expected: vec![h!("b2"), h!("c1")] },
                Walk { from: h!("a3"), direction: 6, expected: vec![h!("a2"), h!("a1")] },
                Walk { from: h!("a3"), direction: 7, expected: vec![] },
                Walk { from: h!("a3"), direction: 8, expected: vec![] },
                Walk { from: h!("a3"), direction: 9, expected: vec![] },
                Walk { from: h!("a3"), direction: 10, expected: vec![] },
                Walk { from: h!("a3"), direction: 11, expected: vec![] },
            
                Walk { from: h!("a4"), direction: 0, expected: vec![h!("a5"), h!("a6")] },
                Walk { from: h!("a4"), direction: 1, expected: vec![h!("b6"), h!("c8")] },
                Walk { from: h!("a4"), direction: 2, expected: vec![h!("b5"), h!("c6"), h!("d7"), h!("e8"), h!("f9"), h!("g9"), h!("h9")] },
                Walk { from: h!("a4"), direction: 3, expected: vec![h!("c5"), h!("e6"), h!("g6"), h!("i5"), h!("l4")] },
                Walk { from: h!("a4"), direction: 4, expected: vec![h!("b4"), h!("c4"), h!("d4"), h!("e4"), h!("f4"), h!("g3"), h!("h2"), h!("i1")] },
                Walk { from: h!("a4"), direction: 5, expected: vec![h!("b3"), h!("c2"), h!("d1")] },
                Walk { from: h!("a4"), direction: 6, expected: vec![h!("a3"), h!("a2"), h!("a1")] },
                Walk { from: h!("a4"), direction: 7, expected: vec![] },
                Walk { from: h!("a4"), direction: 8, expected: vec![] },
                Walk { from: h!("a4"), direction: 9, expected: vec![] },
                Walk { from: h!("a4"), direction: 10, expected: vec![] },
                Walk { from: h!("a4"), direction: 11, expected: vec![] },
            
                Walk { from: h!("a5"), direction: 0, expected: vec![h!("a6")] },
                Walk { from: h!("a5"), direction: 1, expected: vec![h!("b7")] },
                Walk { from: h!("a5"), direction: 2, expected: vec![h!("b6"), h!("c7"), h!("d8"), h!("e9"), h!("f10"), h!("g10")] },
                Walk { from: h!("a5"), direction: 3, expected: vec![h!("c6"), h!("e7"), h!("g7"), h!("i6"), h!("l5")] },
                Walk { from: h!("a5"), direction: 4, expected: vec![h!("b5"), h!("c5"), h!("d5"), h!("e5"), h!("f5"), h!("g4"), h!("h3"), h!("i2"), h!("k1")] },
                Walk { from: h!("a5"), direction: 5, expected: vec![h!("b4"), h!("c3"), h!("d2"), h!("e1")] },
                Walk { from: h!("a5"), direction: 6, expected: vec![h!("a4"), h!("a3"), h!("a2"), h!("a1")] },
                Walk { from: h!("a5"), direction: 7, expected: vec![] },
                Walk { from: h!("a5"), direction: 8, expected: vec![] },
                Walk { from: h!("a5"), direction: 9, expected: vec![] },
                Walk { from: h!("a5"), direction: 10, expected: vec![] },
                Walk { from: h!("a5"), direction: 11, expected: vec![] },
            
                Walk { from: h!("a6"), direction: 0, expected: vec![] },
                Walk { from: h!("a6"), direction: 1, expected: vec![] },
                Walk { from: h!("a6"), direction: 2, expected: vec![h!("b7"), h!("c8"), h!("d9"), h!("e10"), h!("f11")] },
                Walk { from: h!("a6"), direction: 3, expected: vec![h!("c7"), h!("e8"), h!("g8"), h!("i7"), h!("l6")] },
                Walk { from: h!("a6"), direction: 4, expected: vec![h!("b6"), h!("c6"), h!("d6"), h!("e6"), h!("f6"), h!("g5"), h!("h4"), h!("i3"), h!("k2"), h!("l1")] },
                Walk { from: h!("a6"), direction: 5, expected: vec![h!("b5"), h!("c4"), h!("d3"), h!("e2"), h!("f1")] },
                Walk { from: h!("a6"), direction: 6, expected: vec![h!("a5"), h!("a4"), h!("a3"), h!("a2"), h!("a1")] },
                Walk { from: h!("a6"), direction: 7, expected: vec![] },
                Walk { from: h!("a6"), direction: 8, expected: vec![] },
                Walk { from: h!("a6"), direction: 9, expected: vec![] },
                Walk { from: h!("a6"), direction: 10, expected: vec![] },
                Walk { from: h!("a6"), direction: 11, expected: vec![] },
            
                Walk { from: h!("b7"), direction: 0, expected: vec![] },
                Walk { from: h!("b7"), direction: 1, expected: vec![] },
                Walk { from: h!("b7"), direction: 2, expected: vec![h!("c8"), h!("d9"), h!("e10"), h!("f11")] },
                Walk { from: h!("b7"), direction: 3, expected: vec![h!("d8"), h!("f9"), h!("h8"), h!("k7")] },
                Walk { from: h!("b7"), direction: 4, expected: vec![h!("c7"), h!("d7"), h!("e7"), h!("f7"), h!("g6"), h!("h5"), h!("i4"), h!("k3"), h!("l2")] },
                Walk { from: h!("b7"), direction: 5, expected: vec![h!("c6"), h!("d5"), h!("e4"), h!("f3"), h!("g1")] },
                Walk { from: h!("b7"), direction: 6, expected: vec![h!("b6"), h!("b5"), h!("b4"), h!("b3"), h!("b2"), h!("b1")] },
                Walk { from: h!("b7"), direction: 7, expected: vec![h!("a5")] },
                Walk { from: h!("b7"), direction: 8, expected: vec![h!("a6")] },
                Walk { from: h!("b7"), direction: 9, expected: vec![] },
                Walk { from: h!("b7"), direction: 10, expected: vec![] },
                Walk { from: h!("b7"), direction: 11, expected: vec![] },
            
                Walk { from: h!("c8"), direction: 0, expected: vec![] },
                Walk { from: h!("c8"), direction: 1, expected: vec![] },
                Walk { from: h!("c8"), direction: 2, expected: vec![h!("d9"), h!("e10"), h!("f11")] },
                Walk { from: h!("c8"), direction: 3, expected: vec![h!("e9"), h!("g9"), h!("i8")] },
                Walk { from: h!("c8"), direction: 4, expected: vec![h!("d8"), h!("e8"), h!("f8"), h!("g7"), h!("h6"), h!("i5"), h!("k4"), h!("l3")] },
                Walk { from: h!("c8"), direction: 5, expected: vec![h!("d7"), h!("e6"), h!("f5"), h!("g3"), h!("h1")] },
                Walk { from: h!("c8"), direction: 6, expected: vec![h!("c7"), h!("c6"), h!("c5"), h!("c4"), h!("c3"), h!("c2"), h!("c1")] },
                Walk { from: h!("c8"), direction: 7, expected: vec![h!("b6"), h!("a4")] },
                Walk { from: h!("c8"), direction: 8, expected: vec![h!("b7"), h!("a6")] },
                Walk { from: h!("c8"), direction: 9, expected: vec![] },
                Walk { from: h!("c8"), direction: 10, expected: vec![] },
                Walk { from: h!("c8"), direction: 11, expected: vec![] },
            
                Walk { from: h!("d9"), direction: 0, expected: vec![] },
                Walk { from: h!("d9"), direction: 1, expected: vec![] },
                Walk { from: h!("d9"), direction: 2, expected: vec![h!("e10"), h!("f11")] },
                Walk { from: h!("d9"), direction: 3, expected: vec![h!("f10"), h!("h9")] },
                Walk { from: h!("d9"), direction: 4, expected: vec![h!("e9"), h!("f9"), h!("g8"), h!("h7"), h!("i6"), h!("k5"), h!("l4")] },
                Walk { from: h!("d9"), direction: 5, expected: vec![h!("e8"), h!("f7"), h!("g5"), h!("h3"), h!("i1")] },
                Walk { from: h!("d9"), direction: 6, expected: vec![h!("d8"), h!("d7"), h!("d6"), h!("d5"), h!("d4"), h!("d3"), h!("d2"), h!("d1")] },
                Walk { from: h!("d9"), direction: 7, expected: vec![h!("c7"), h!("b5"), h!("a3")] },
                Walk { from: h!("d9"), direction: 8, expected: vec![h!("c8"), h!("b7"), h!("a6")] },
                Walk { from: h!("d9"), direction: 9, expected: vec![] },
                Walk { from: h!("d9"), direction: 10, expected: vec![] },
                Walk { from: h!("d9"), direction: 11, expected: vec![] },
            
                Walk { from: h!("e10"), direction: 0, expected: vec![] },
                Walk { from: h!("e10"), direction: 1, expected: vec![] },
                Walk { from: h!("e10"), direction: 2, expected: vec![h!("f11")] },
                Walk { from: h!("e10"), direction: 3, expected: vec![h!("g10")] },
                Walk { from: h!("e10"), direction: 4, expected: vec![h!("f10"), h!("g9"), h!("h8"), h!("i7"), h!("k6"), h!("l5")] },
                Walk { from: h!("e10"), direction: 5, expected: vec![h!("f9"), h!("g7"), h!("h5"), h!("i3"), h!("k1")] },
                Walk { from: h!("e10"), direction: 6, expected: vec![h!("e9"), h!("e8"), h!("e7"), h!("e6"), h!("e5"), h!("e4"), h!("e3"), h!("e2"), h!("e1")] },
                Walk { from: h!("e10"), direction: 7, expected: vec![h!("d8"), h!("c6"), h!("b4"), h!("a2")] },
                Walk { from: h!("e10"), direction: 8, expected: vec![h!("d9"), h!("c8"), h!("b7"), h!("a6")] },
                Walk { from: h!("e10"), direction: 9, expected: vec![] },
                Walk { from: h!("e10"), direction: 10, expected: vec![] },
                Walk { from: h!("e10"), direction: 11, expected: vec![] },
            
                Walk { from: h!("f11"), direction: 0, expected: vec![] },
                Walk { from: h!("f11"), direction: 1, expected: vec![] },
                Walk { from: h!("f11"), direction: 2, expected: vec![] },
                Walk { from: h!("f11"), direction: 3, expected: vec![] },
                Walk { from: h!("f11"), direction: 4, expected: vec![h!("g10"), h!("h9"), h!("i8"), h!("k7"), h!("l6")] },
                Walk { from: h!("f11"), direction: 5, expected: vec![h!("g9"), h!("h7"), h!("i5"), h!("k3"), h!("l1")] },
                Walk { from: h!("f11"), direction: 6, expected: vec![h!("f10"), h!("f9"), h!("f8"), h!("f7"), h!("f6"), h!("f5"), h!("f4"), h!("f3"), h!("f2"), h!("f1")] },
                Walk { from: h!("f11"), direction: 7, expected: vec![h!("e9"), h!("d7"), h!("c5"), h!("b3"), h!("a1")] },
                Walk { from: h!("f11"), direction: 8, expected: vec![h!("e10"), h!("d9"), h!("c8"), h!("b7"), h!("a6")] },
                Walk { from: h!("f11"), direction: 9, expected: vec![] },
                Walk { from: h!("f11"), direction: 10, expected: vec![] },
                Walk { from: h!("f11"), direction: 11, expected: vec![] },
            
                Walk { from: h!("g10"), direction: 0, expected: vec![] },
                Walk { from: h!("g10"), direction: 1, expected: vec![] },
                Walk { from: h!("g10"), direction: 2, expected: vec![] },
                Walk { from: h!("g10"), direction: 3, expected: vec![] },
                Walk { from: h!("g10"), direction: 4, expected: vec![h!("h9"), h!("i8"), h!("k7"), h!("l6")] },
                Walk { from: h!("g10"), direction: 5, expected: vec![h!("h8"), h!("i6"), h!("k4"), h!("l2")] },
                Walk { from: h!("g10"), direction: 6, expected: vec![h!("g9"), h!("g8"), h!("g7"), h!("g6"), h!("g5"), h!("g4"), h!("g3"), h!("g2"), h!("g1")] },
                Walk { from: h!("g10"), direction: 7, expected: vec![h!("f9"), h!("e7"), h!("d5"), h!("c3"), h!("b1")] },
                Walk { from: h!("g10"), direction: 8, expected: vec![h!("f10"), h!("e9"), h!("d8"), h!("c7"), h!("b6"), h!("a5")] },
                Walk { from: h!("g10"), direction: 9, expected: vec![h!("e10")] },
                Walk { from: h!("g10"), direction: 10, expected: vec![h!("f11")] },
                Walk { from: h!("g10"), direction: 11, expected: vec![] },
            
                Walk { from: h!("h9"), direction: 0, expected: vec![] },
                Walk { from: h!("h9"), direction: 1, expected: vec![] },
                Walk { from: h!("h9"), direction: 2, expected: vec![] },
                Walk { from: h!("h9"), direction: 3, expected: vec![] },
                Walk { from: h!("h9"), direction: 4, expected: vec![h!("i8"), h!("k7"), h!("l6")] },
                Walk { from: h!("h9"), direction: 5, expected: vec![h!("i7"), h!("k5"), h!("l3")] },
                Walk { from: h!("h9"), direction: 6, expected: vec![h!("h8"), h!("h7"), h!("h6"), h!("h5"), h!("h4"), h!("h3"), h!("h2"), h!("h1")] },
                Walk { from: h!("h9"), direction: 7, expected: vec![h!("g8"), h!("f7"), h!("e5"), h!("d3"), h!("c1")] },
                Walk { from: h!("h9"), direction: 8, expected: vec![h!("g9"), h!("f9"), h!("e8"), h!("d7"), h!("c6"), h!("b5"), h!("a4")] },
                Walk { from: h!("h9"), direction: 9, expected: vec![h!("f10"), h!("d9")] },
                Walk { from: h!("h9"), direction: 10, expected: vec![h!("g10"), h!("f11")] },
                Walk { from: h!("h9"), direction: 11, expected: vec![] },
            
                Walk { from: h!("i8"), direction: 0, expected: vec![] },
                Walk { from: h!("i8"), direction: 1, expected: vec![] },
                Walk { from: h!("i8"), direction: 2, expected: vec![] },
                Walk { from: h!("i8"), direction: 3, expected: vec![] },
                Walk { from: h!("i8"), direction: 4, expected: vec![h!("k7"), h!("l6")] },
                Walk { from: h!("i8"), direction: 5, expected: vec![h!("k6"), h!("l4")] },
                Walk { from: h!("i8"), direction: 6, expected: vec![h!("i7"), h!("i6"), h!("i5"), h!("i4"), h!("i3"), h!("i2"), h!("i1")] },
                Walk { from: h!("i8"), direction: 7, expected: vec![h!("h7"), h!("g6"), h!("f5"), h!("e3"), h!("d1")] },
                Walk { from: h!("i8"), direction: 8, expected: vec![h!("h8"), h!("g8"), h!("f8"), h!("e7"), h!("d6"), h!("c5"), h!("b4"), h!("a3")] },
                Walk { from: h!("i8"), direction: 9, expected: vec![h!("g9"), h!("e9"), h!("c8")] },
                Walk { from: h!("i8"), direction: 10, expected: vec![h!("h9"), h!("g10"), h!("f11")] },
                Walk { from: h!("i8"), direction: 11, expected: vec![] },
            
                Walk { from: h!("k7"), direction: 0, expected: vec![] },
                Walk { from: h!("k7"), direction: 1, expected: vec![] },
                Walk { from: h!("k7"), direction: 2, expected: vec![] },
                Walk { from: h!("k7"), direction: 3, expected: vec![] },
                Walk { from: h!("k7"), direction: 4, expected: vec![h!("l6")] },
                Walk { from: h!("k7"), direction: 5, expected: vec![h!("l5")] },
                Walk { from: h!("k7"), direction: 6, expected: vec![h!("k6"), h!("k5"), h!("k4"), h!("k3"), h!("k2"), h!("k1")] },
                Walk { from: h!("k7"), direction: 7, expected: vec![h!("i6"), h!("h5"), h!("g4"), h!("f3"), h!("e1")] },
                Walk { from: h!("k7"), direction: 8, expected: vec![h!("i7"), h!("h7"), h!("g7"), h!("f7"), h!("e6"), h!("d5"), h!("c4"), h!("b3"), h!("a2")] },
                Walk { from: h!("k7"), direction: 9, expected: vec![h!("h8"), h!("f9"), h!("d8"), h!("b7")] },
                Walk { from: h!("k7"), direction: 10, expected: vec![h!("i8"), h!("h9"), h!("g10"), h!("f11")] },
                Walk { from: h!("k7"), direction: 11, expected: vec![] },
            
                Walk { from: h!("l6"), direction: 0, expected: vec![] },
                Walk { from: h!("l6"), direction: 1, expected: vec![] },
                Walk { from: h!("l6"), direction: 2, expected: vec![] },
                Walk { from: h!("l6"), direction: 3, expected: vec![] },
                Walk { from: h!("l6"), direction: 4, expected: vec![] },
                Walk { from: h!("l6"), direction: 5, expected: vec![] },
                Walk { from: h!("l6"), direction: 6, expected: vec![h!("l5"), h!("l4"), h!("l3"), h!("l2"), h!("l1")] },
                Walk { from: h!("l6"), direction: 7, expected: vec![h!("k5"), h!("i4"), h!("h3"), h!("g2"), h!("f1")] },
                Walk { from: h!("l6"), direction: 8, expected: vec![h!("k6"), h!("i6"), h!("h6"), h!("g6"), h!("f6"), h!("e5"), h!("d4"), h!("c3"), h!("b2"), h!("a1")] },
                Walk { from: h!("l6"), direction: 9, expected: vec![h!("i7"), h!("g8"), h!("e8"), h!("c7"), h!("a6")] },
                Walk { from: h!("l6"), direction: 10, expected: vec![h!("k7"), h!("i8"), h!("h9"), h!("g10"), h!("f11")] },
                Walk { from: h!("l6"), direction: 11, expected: vec![] },
            
                Walk { from: h!("l5"), direction: 0, expected: vec![h!("l6")] },
                Walk { from: h!("l5"), direction: 1, expected: vec![] },
                Walk { from: h!("l5"), direction: 2, expected: vec![] },
                Walk { from: h!("l5"), direction: 3, expected: vec![] },
                Walk { from: h!("l5"), direction: 4, expected: vec![] },
                Walk { from: h!("l5"), direction: 5, expected: vec![] },
                Walk { from: h!("l5"), direction: 6, expected: vec![h!("l4"), h!("l3"), h!("l2"), h!("l1")] },
                Walk { from: h!("l5"), direction: 7, expected: vec![h!("k4"), h!("i3"), h!("h2"), h!("g1")] },
                Walk { from: h!("l5"), direction: 8, expected: vec![h!("k5"), h!("i5"), h!("h5"), h!("g5"), h!("f5"), h!("e4"), h!("d3"), h!("c2"), h!("b1")] },
                Walk { from: h!("l5"), direction: 9, expected: vec![h!("i6"), h!("g7"), h!("e7"), h!("c6"), h!("a5")] },
                Walk { from: h!("l5"), direction: 10, expected: vec![h!("k6"), h!("i7"), h!("h8"), h!("g9"), h!("f10"), h!("e10")] },
                Walk { from: h!("l5"), direction: 11, expected: vec![h!("k7")] },
            
                Walk { from: h!("l4"), direction: 0, expected: vec![h!("l5"), h!("l6")] },
                Walk { from: h!("l4"), direction: 1, expected: vec![] },
                Walk { from: h!("l4"), direction: 2, expected: vec![] },
                Walk { from: h!("l4"), direction: 3, expected: vec![] },
                Walk { from: h!("l4"), direction: 4, expected: vec![] },
                Walk { from: h!("l4"), direction: 5, expected: vec![] },
                Walk { from: h!("l4"), direction: 6, expected: vec![h!("l3"), h!("l2"), h!("l1")] },
                Walk { from: h!("l4"), direction: 7, expected: vec![h!("k3"), h!("i2"), h!("h1")] },
                Walk { from: h!("l4"), direction: 8, expected: vec![h!("k4"), h!("i4"), h!("h4"), h!("g4"), h!("f4"), h!("e3"), h!("d2"), h!("c1")] },
                Walk { from: h!("l4"), direction: 9, expected: vec![h!("i5"), h!("g6"), h!("e6"), h!("c5"), h!("a4")] },
                Walk { from: h!("l4"), direction: 10, expected: vec![h!("k5"), h!("i6"), h!("h7"), h!("g8"), h!("f9"), h!("e9"), h!("d9")] },
                Walk { from: h!("l4"), direction: 11, expected: vec![h!("k6"), h!("i8")] },
            
                Walk { from: h!("l3"), direction: 0, expected: vec![h!("l4"), h!("l5"), h!("l6")] },
                Walk { from: h!("l3"), direction: 1, expected: vec![] },
                Walk { from: h!("l3"), direction: 2, expected: vec![] },
                Walk { from: h!("l3"), direction: 3, expected: vec![] },
                Walk { from: h!("l3"), direction: 4, expected: vec![] },
                Walk { from: h!("l3"), direction: 5, expected: vec![] },
                Walk { from: h!("l3"), direction: 6, expected: vec![h!("l2"), h!("l1")] },
                Walk { from: h!("l3"), direction: 7, expected: vec![h!("k2"), h!("i1")] },
                Walk { from: h!("l3"), direction: 8, expected: vec![h!("k3"), h!("i3"), h!("h3"), h!("g3"), h!("f3"), h!("e2"), h!("d1")] },
                Walk { from: h!("l3"), direction: 9, expected: vec![h!("i4"), h!("g5"), h!("e5"), h!("c4"), h!("a3")] },
                Walk { from: h!("l3"), direction: 10, expected: vec![h!("k4"), h!("i5"), h!("h6"), h!("g7"), h!("f8"), h!("e8"), h!("d8"), h!("c8")] },
                Walk { from: h!("l3"), direction: 11, expected: vec![h!("k5"), h!("i7"), h!("h9")] },
            
                Walk { from: h!("l2"), direction: 0, expected: vec![h!("l3"), h!("l4"), h!("l5"), h!("l6")] },
                Walk { from: h!("l2"), direction: 1, expected: vec![] },
                Walk { from: h!("l2"), direction: 2, expected: vec![] },
                Walk { from: h!("l2"), direction: 3, expected: vec![] },
                Walk { from: h!("l2"), direction: 4, expected: vec![] },
                Walk { from: h!("l2"), direction: 5, expected: vec![] },
                Walk { from: h!("l2"), direction: 6, expected: vec![h!("l1")] },
                Walk { from: h!("l2"), direction: 7, expected: vec![h!("k1")] },
                Walk { from: h!("l2"), direction: 8, expected: vec![h!("k2"), h!("i2"), h!("h2"), h!("g2"), h!("f2"), h!("e1")] },
                Walk { from: h!("l2"), direction: 9, expected: vec![h!("i3"), h!("g4"), h!("e4"), h!("c3"), h!("a2")] },
                Walk { from: h!("l2"), direction: 10, expected: vec![h!("k3"), h!("i4"), h!("h5"), h!("g6"), h!("f7"), h!("e7"), h!("d7"), h!("c7"), h!("b7")] },
                Walk { from: h!("l2"), direction: 11, expected: vec![h!("k4"), h!("i6"), h!("h8"), h!("g10")] },
            
                Walk { from: h!("l1"), direction: 0, expected: vec![h!("l2"), h!("l3"), h!("l4"), h!("l5"), h!("l6")] },
                Walk { from: h!("l1"), direction: 1, expected: vec![] },
                Walk { from: h!("l1"), direction: 2, expected: vec![] },
                Walk { from: h!("l1"), direction: 3, expected: vec![] },
                Walk { from: h!("l1"), direction: 4, expected: vec![] },
                Walk { from: h!("l1"), direction: 5, expected: vec![] },
                Walk { from: h!("l1"), direction: 6, expected: vec![] },
                Walk { from: h!("l1"), direction: 7, expected: vec![] },
                Walk { from: h!("l1"), direction: 8, expected: vec![h!("k1"), h!("i1"), h!("h1"), h!("g1"), h!("f1")] },
                Walk { from: h!("l1"), direction: 9, expected: vec![h!("i2"), h!("g3"), h!("e3"), h!("c2"), h!("a1")] },
                Walk { from: h!("l1"), direction: 10, expected: vec![h!("k2"), h!("i3"), h!("h4"), h!("g5"), h!("f6"), h!("e6"), h!("d6"), h!("c6"), h!("b6"), h!("a6")] },
                Walk { from: h!("l1"), direction: 11, expected: vec![h!("k3"), h!("i5"), h!("h7"), h!("g9"), h!("f11")] },
            
                Walk { from: h!("k1"), direction: 0, expected: vec![h!("k2"), h!("k3"), h!("k4"), h!("k5"), h!("k6"), h!("k7")] },
                Walk { from: h!("k1"), direction: 1, expected: vec![h!("l2")] },
                Walk { from: h!("k1"), direction: 2, expected: vec![h!("l1")] },
                Walk { from: h!("k1"), direction: 3, expected: vec![] },
                Walk { from: h!("k1"), direction: 4, expected: vec![] },
                Walk { from: h!("k1"), direction: 5, expected: vec![] },
                Walk { from: h!("k1"), direction: 6, expected: vec![] },
                Walk { from: h!("k1"), direction: 7, expected: vec![] },
                Walk { from: h!("k1"), direction: 8, expected: vec![h!("i1"), h!("h1"), h!("g1"), h!("f1")] },
                Walk { from: h!("k1"), direction: 9, expected: vec![h!("h2"), h!("f3"), h!("d2"), h!("b1")] },
                Walk { from: h!("k1"), direction: 10, expected: vec![h!("i2"), h!("h3"), h!("g4"), h!("f5"), h!("e5"), h!("d5"), h!("c5"), h!("b5"), h!("a5")] },
                Walk { from: h!("k1"), direction: 11, expected: vec![h!("i3"), h!("h5"), h!("g7"), h!("f9"), h!("e10")] },
            
                Walk { from: h!("i1"), direction: 0, expected: vec![h!("i2"), h!("i3"), h!("i4"), h!("i5"), h!("i6"), h!("i7"), h!("i8")] },
                Walk { from: h!("i1"), direction: 1, expected: vec![h!("k2"), h!("l3")] },
                Walk { from: h!("i1"), direction: 2, expected: vec![h!("k1"), h!("l1")] },
                Walk { from: h!("i1"), direction: 3, expected: vec![] },
                Walk { from: h!("i1"), direction: 4, expected: vec![] },
                Walk { from: h!("i1"), direction: 5, expected: vec![] },
                Walk { from: h!("i1"), direction: 6, expected: vec![] },
                Walk { from: h!("i1"), direction: 7, expected: vec![] },
                Walk { from: h!("i1"), direction: 8, expected: vec![h!("h1"), h!("g1"), h!("f1")] },
                Walk { from: h!("i1"), direction: 9, expected: vec![h!("g2"), h!("e2"), h!("c1")] },
                Walk { from: h!("i1"), direction: 10, expected: vec![h!("h2"), h!("g3"), h!("f4"), h!("e4"), h!("d4"), h!("c4"), h!("b4"), h!("a4")] },
                Walk { from: h!("i1"), direction: 11, expected: vec![h!("h3"), h!("g5"), h!("f7"), h!("e8"), h!("d9")] },
            
                Walk { from: h!("h1"), direction: 0, expected: vec![h!("h2"), h!("h3"), h!("h4"), h!("h5"), h!("h6"), h!("h7"), h!("h8"), h!("h9")] },
                Walk { from: h!("h1"), direction: 1, expected: vec![h!("i2"), h!("k3"), h!("l4")] },
                Walk { from: h!("h1"), direction: 2, expected: vec![h!("i1"), h!("k1"), h!("l1")] },
                Walk { from: h!("h1"), direction: 3, expected: vec![] },
                Walk { from: h!("h1"), direction: 4, expected: vec![] },
                Walk { from: h!("h1"), direction: 5, expected: vec![] },
                Walk { from: h!("h1"), direction: 6, expected: vec![] },
                Walk { from: h!("h1"), direction: 7, expected: vec![] },
                Walk { from: h!("h1"), direction: 8, expected: vec![h!("g1"), h!("f1")] },
                Walk { from: h!("h1"), direction: 9, expected: vec![h!("f2"), h!("d1")] },
                Walk { from: h!("h1"), direction: 10, expected: vec![h!("g2"), h!("f3"), h!("e3"), h!("d3"), h!("c3"), h!("b3"), h!("a3")] },
                Walk { from: h!("h1"), direction: 11, expected: vec![h!("g3"), h!("f5"), h!("e6"), h!("d7"), h!("c8")] },
            
                Walk { from: h!("g1"), direction: 0, expected: vec![h!("g2"), h!("g3"), h!("g4"), h!("g5"), h!("g6"), h!("g7"), h!("g8"), h!("g9"), h!("g10")] },
                Walk { from: h!("g1"), direction: 1, expected: vec![h!("h2"), h!("i3"), h!("k4"), h!("l5")] },
                Walk { from: h!("g1"), direction: 2, expected: vec![h!("h1"), h!("i1"), h!("k1"), h!("l1")] },
                Walk { from: h!("g1"), direction: 3, expected: vec![] },
                Walk { from: h!("g1"), direction: 4, expected: vec![] },
                Walk { from: h!("g1"), direction: 5, expected: vec![] },
                Walk { from: h!("g1"), direction: 6, expected: vec![] },
                Walk { from: h!("g1"), direction: 7, expected: vec![] },
                Walk { from: h!("g1"), direction: 8, expected: vec![h!("f1")] },
                Walk { from: h!("g1"), direction: 9, expected: vec![h!("e1")] },
                Walk { from: h!("g1"), direction: 10, expected: vec![h!("f2"), h!("e2"), h!("d2"), h!("c2"), h!("b2"), h!("a2")] },
                Walk { from: h!("g1"), direction: 11, expected: vec![h!("f3"), h!("e4"), h!("d5"), h!("c6"), h!("b7")] },
            
                Walk { from: h!("f1"), direction: 0, expected: vec![h!("f2"), h!("f3"), h!("f4"), h!("f5"), h!("f6"), h!("f7"), h!("f8"), h!("f9"), h!("f10"), h!("f11")] },
                Walk { from: h!("f1"), direction: 1, expected: vec![h!("g2"), h!("h3"), h!("i4"), h!("k5"), h!("l6")] },
                Walk { from: h!("f1"), direction: 2, expected: vec![h!("g1"), h!("h1"), h!("i1"), h!("k1"), h!("l1")] },
                Walk { from: h!("f1"), direction: 3, expected: vec![] },
                Walk { from: h!("f1"), direction: 4, expected: vec![] },
                Walk { from: h!("f1"), direction: 5, expected: vec![] },
                Walk { from: h!("f1"), direction: 6, expected: vec![] },
                Walk { from: h!("f1"), direction: 7, expected: vec![] },
                Walk { from: h!("f1"), direction: 8, expected: vec![] },
                Walk { from: h!("f1"), direction: 9, expected: vec![] },
                Walk { from: h!("f1"), direction: 10, expected: vec![h!("e1"), h!("d1"), h!("c1"), h!("b1"), h!("a1")] },
                Walk { from: h!("f1"), direction: 11, expected: vec![h!("e2"), h!("d3"), h!("c4"), h!("b5"), h!("a6")] },
            
                Walk { from: h!("e1"), direction: 0, expected: vec![h!("e2"), h!("e3"), h!("e4"), h!("e5"), h!("e6"), h!("e7"), h!("e8"), h!("e9"), h!("e10")] },
                Walk { from: h!("e1"), direction: 1, expected: vec![h!("f3"), h!("g4"), h!("h5"), h!("i6"), h!("k7")] },
                Walk { from: h!("e1"), direction: 2, expected: vec![h!("f2"), h!("g2"), h!("h2"), h!("i2"), h!("k2"), h!("l2")] },
                Walk { from: h!("e1"), direction: 3, expected: vec![h!("g1")] },
                Walk { from: h!("e1"), direction: 4, expected: vec![h!("f1")] },
                Walk { from: h!("e1"), direction: 5, expected: vec![] },
                Walk { from: h!("e1"), direction: 6, expected: vec![] },
                Walk { from: h!("e1"), direction: 7, expected: vec![] },
                Walk { from: h!("e1"), direction: 8, expected: vec![] },
                Walk { from: h!("e1"), direction: 9, expected: vec![] },
                Walk { from: h!("e1"), direction: 10, expected: vec![h!("d1"), h!("c1"), h!("b1"), h!("a1")] },
                Walk { from: h!("e1"), direction: 11, expected: vec![h!("d2"), h!("c3"), h!("b4"), h!("a5")] },
            
                Walk { from: h!("d1"), direction: 0, expected: vec![h!("d2"), h!("d3"), h!("d4"), h!("d5"), h!("d6"), h!("d7"), h!("d8"), h!("d9")] },
                Walk { from: h!("d1"), direction: 1, expected: vec![h!("e3"), h!("f5"), h!("g6"), h!("h7"), h!("i8")] },
                Walk { from: h!("d1"), direction: 2, expected: vec![h!("e2"), h!("f3"), h!("g3"), h!("h3"), h!("i3"), h!("k3"), h!("l3")] },
                Walk { from: h!("d1"), direction: 3, expected: vec![h!("f2"), h!("h1")] },
                Walk { from: h!("d1"), direction: 4, expected: vec![h!("e1"), h!("f1")] },
                Walk { from: h!("d1"), direction: 5, expected: vec![] },
                Walk { from: h!("d1"), direction: 6, expected: vec![] },
                Walk { from: h!("d1"), direction: 7, expected: vec![] },
                Walk { from: h!("d1"), direction: 8, expected: vec![] },
                Walk { from: h!("d1"), direction: 9, expected: vec![] },
                Walk { from: h!("d1"), direction: 10, expected: vec![h!("c1"), h!("b1"), h!("a1")] },
                Walk { from: h!("d1"), direction: 11, expected: vec![h!("c2"), h!("b3"), h!("a4")] },
            
                Walk { from: h!("c1"), direction: 0, expected: vec![h!("c2"), h!("c3"), h!("c4"), h!("c5"), h!("c6"), h!("c7"), h!("c8")] },
                Walk { from: h!("c1"), direction: 1, expected: vec![h!("d3"), h!("e5"), h!("f7"), h!("g8"), h!("h9")] },
                Walk { from: h!("c1"), direction: 2, expected: vec![h!("d2"), h!("e3"), h!("f4"), h!("g4"), h!("h4"), h!("i4"), h!("k4"), h!("l4")] },
                Walk { from: h!("c1"), direction: 3, expected: vec![h!("e2"), h!("g2"), h!("i1")] },
                Walk { from: h!("c1"), direction: 4, expected: vec![h!("d1"), h!("e1"), h!("f1")] },
                Walk { from: h!("c1"), direction: 5, expected: vec![] },
                Walk { from: h!("c1"), direction: 6, expected: vec![] },
                Walk { from: h!("c1"), direction: 7, expected: vec![] },
                Walk { from: h!("c1"), direction: 8, expected: vec![] },
                Walk { from: h!("c1"), direction: 9, expected: vec![] },
                Walk { from: h!("c1"), direction: 10, expected: vec![h!("b1"), h!("a1")] },
                Walk { from: h!("c1"), direction: 11, expected: vec![h!("b2"), h!("a3")] },
            
                Walk { from: h!("b1"), direction: 0, expected: vec![h!("b2"), h!("b3"), h!("b4"), h!("b5"), h!("b6"), h!("b7")] },
                Walk { from: h!("b1"), direction: 1, expected: vec![h!("c3"), h!("d5"), h!("e7"), h!("f9"), h!("g10")] },
                Walk { from: h!("b1"), direction: 2, expected: vec![h!("c2"), h!("d3"), h!("e4"), h!("f5"), h!("g5"), h!("h5"), h!("i5"), h!("k5"), h!("l5")] },
                Walk { from: h!("b1"), direction: 3, expected: vec![h!("d2"), h!("f3"), h!("h2"), h!("k1")] },
                Walk { from: h!("b1"), direction: 4, expected: vec![h!("c1"), h!("d1"), h!("e1"), h!("f1")] },
                Walk { from: h!("b1"), direction: 5, expected: vec![] },
                Walk { from: h!("b1"), direction: 6, expected: vec![] },
                Walk { from: h!("b1"), direction: 7, expected: vec![] },
                Walk { from: h!("b1"), direction: 8, expected: vec![] },
                Walk { from: h!("b1"), direction: 9, expected: vec![] },
                Walk { from: h!("b1"), direction: 10, expected: vec![h!("a1")] },
                Walk { from: h!("b1"), direction: 11, expected: vec![h!("a2")] },
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
