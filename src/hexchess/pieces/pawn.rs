use crate::constants::Color;
use crate::hex;
use crate::hexchess::hexchess::Hexchess;
use crate::hexchess::san::San;

use crate::hexchess::utils::{
    get_color,
    step,
};


pub fn pawn_moves_unsafe(
    hexchess: &Hexchess,
    from: u8,
    color: &Color,
) -> Vec<San> {
    let mut result: Vec<San> = vec![];

    let (
      forward_direction, 
      portside_direction, 
      starboard_direction
    ) = match color {
      Color::White => (0u8, 10u8, 2u8),
      Color::Black => (6u8, 4u8, 8u8),
    };

    // advance forward one position
    match advance(hexchess, from, from, forward_direction) {
        None => {},
        Some(san) => {
            result.push(san);

            // advance forward another position if possible
            if is_starting_position(from, *color) {
                match advance(hexchess, from, san.to, forward_direction) {
                    None => {}
                    Some(san) => result.push(san),
                };
            }
        },
    };

    // capture portside
    match capture(hexchess, from, portside_direction, *color) {
        None => {},
        Some(san) => result.push(san),
    };

    // capture starboard
    match capture(hexchess, from, starboard_direction, *color) {
        None => {},
        Some(san) => result.push(san),
    };

    // // expand promotions
    // for (let i = 0; i < moves.length; i++) {
    //     const { from, to } = moves[i]

    //     if (promotionPositions[color].includes(to)) {
    //     moves.splice(i, 1,
    //         { from, to, promotion: 'q' },
    //         { from, to, promotion: 'r' },
    //         { from, to, promotion: 'b' },
    //         { from, to, promotion: 'n' }
    //     )

    //     i += 4
    //     }
    // }
        
    result
}

fn advance(hexchess: &Hexchess, start: u8, from: u8, forward_direction: u8) -> Option<San> {
    match step(from, forward_direction) {
        None => None,
        Some(to) => match hexchess.board[to as usize] {
            None => Some(San { from: start, promotion: None, to }),
            Some(_) => None,
        }
    }
}

fn capture(hexchess: &Hexchess, from: u8, capture_direction: u8, friendly_color: Color) -> Option<San> {
    match step(from, capture_direction) {
        None => None,
        Some(to) => match hexchess.board[to as usize] {
            None => match hexchess.ep {
                None => None,
                Some(ep) => match to == ep && hexchess.turn == friendly_color {
                    true => Some(San { from, promotion: None, to }),
                    false => None,
                },
            },
            Some(piece) => match get_color(&piece) != friendly_color {
                true => Some(San { from, promotion: None, to }),
                false => None,
            },
        }
    }
}

fn is_starting_position(position: u8, color: Color) -> bool {
    match color {
        Color::Black => match position {
            hex!("b7") |
            hex!("c7") |
            hex!("d7") |
            hex!("e7") |
            hex!("f7") |
            hex!("g7") |
            hex!("h7") |
            hex!("i7") |
            hex!("k7") => true,
            _ => false,
        },
        Color::White => match position {
            hex!("b1") |
            hex!("c2") |
            hex!("d3") |
            hex!("e4") |
            hex!("f5") |
            hex!("g4") |
            hex!("h3") |
            hex!("i2") |
            hex!("k1") => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hex;
    use super::*;

    #[test]
    fn black_starting_pawns() {
        let b7 = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("b7"));
        assert_eq!(b7[0], San { from: hex!("b7"), promotion: None, to: hex!("b6") });
        assert_eq!(b7[1], San { from: hex!("b7"), promotion: None, to: hex!("b5") });

        let c7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("c7"));
        assert_eq!(c7[0], San { from: hex!("c7"), promotion: None, to: hex!("c6") });
        assert_eq!(c7[1], San { from: hex!("c7"), promotion: None, to: hex!("c5") });

        let d7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("d7"));
        assert_eq!(d7[0], San { from: hex!("d7"), promotion: None, to: hex!("d6") });
        assert_eq!(d7[1], San { from: hex!("d7"), promotion: None, to: hex!("d5") });

        let e7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("e7"));
        assert_eq!(e7[0], San { from: hex!("e7"), promotion: None, to: hex!("e6") });
        assert_eq!(e7[1], San { from: hex!("e7"), promotion: None, to: hex!("e5") });

        let f7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("f7"));
        assert_eq!(f7[0], San { from: hex!("f7"), promotion: None, to: hex!("f6") });
        assert_eq!(f7[1], San { from: hex!("f7"), promotion: None, to: hex!("f5") });

        let g7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("g7"));
        assert_eq!(g7[0], San { from: hex!("g7"), promotion: None, to: hex!("g6") });
        assert_eq!(g7[1], San { from: hex!("g7"), promotion: None, to: hex!("g5") });

        let h7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("h7"));
        assert_eq!(h7[0], San { from: hex!("h7"), promotion: None, to: hex!("h6") });
        assert_eq!(h7[1], San { from: hex!("h7"), promotion: None, to: hex!("h5") });

        let i7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("i7"));
        assert_eq!(i7[0], San { from: hex!("i7"), promotion: None, to: hex!("i6") });
        assert_eq!(i7[1], San { from: hex!("i7"), promotion: None, to: hex!("i5") });

        let k7: Vec<San> = Hexchess::from("1/3/5/7/ppppppppp/11/11/11/11/11/11 b - 0 1").unwrap().moves_from(hex!("k7"));
        assert_eq!(k7[0], San { from: hex!("k7"), promotion: None, to: hex!("k6") });
        assert_eq!(k7[1], San { from: hex!("k7"), promotion: None, to: hex!("k5") });
    }

    #[test]
    fn black_blocked_friendly() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4p4/5p5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(hex!("f7"));

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn black_blocked_friendly_double() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4p4/11/5p5/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(hex!("f7"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: hex!("f7"), promotion: None, to: hex!("f6") });
    }

    #[test]
    fn black_blocked_hostile() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4p4/5P5/11/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(hex!("f7"));

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn black_blocked_hostile_double() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4p4/11/5P5/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(hex!("f7"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: hex!("f7"), promotion: None, to: hex!("f6") });
    }

    #[test]
    fn white_starting_pawns() {
        let f5: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("f5"));
        assert_eq!(f5[0], San { from: hex!("f5"), promotion: None, to: hex!("f6") });
        assert_eq!(f5[1], San { from: hex!("f5"), promotion: None, to: hex!("f7") });

        let e4: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("e4"));
        assert_eq!(e4[0], San { from: hex!("e4"), promotion: None, to: hex!("e5") });
        assert_eq!(e4[1], San { from: hex!("e4"), promotion: None, to: hex!("e6") });

        let g4: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("g4"));
        assert_eq!(g4[0], San { from: hex!("g4"), promotion: None, to: hex!("g5") });
        assert_eq!(g4[1], San { from: hex!("g4"), promotion: None, to: hex!("g6") });

        let d3: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("d3"));
        assert_eq!(d3[0], San { from: hex!("d3"), promotion: None, to: hex!("d4") });
        assert_eq!(d3[1], San { from: hex!("d3"), promotion: None, to: hex!("d5") });

        let h3: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("h3"));
        assert_eq!(h3[0], San { from: hex!("h3"), promotion: None, to: hex!("h4") });
        assert_eq!(h3[1], San { from: hex!("h3"), promotion: None, to: hex!("h5") });

        let c2: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("c2"));
        assert_eq!(c2[0], San { from: hex!("c2"), promotion: None, to: hex!("c3") });
        assert_eq!(c2[1], San { from: hex!("c2"), promotion: None, to: hex!("c4") });

        let i2: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("i2"));
        assert_eq!(i2[0], San { from: hex!("i2"), promotion: None, to: hex!("i3") });
        assert_eq!(i2[1], San { from: hex!("i2"), promotion: None, to: hex!("i4") });

        let b1: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("b1"));
        assert_eq!(b1[0], San { from: hex!("b1"), promotion: None, to: hex!("b2") });
        assert_eq!(b1[1], San { from: hex!("b1"), promotion: None, to: hex!("b3") });

        let k1: Vec<San> = Hexchess::from("1/3/5/7/9/11/5P5/4P1P4/3P3P3/2P5P2/1P7P1 w - 0 1").unwrap().moves_from(hex!("k1"));
        assert_eq!(k1[0], San { from: hex!("k1"), promotion: None, to: hex!("k2") });
        assert_eq!(k1[1], San { from: hex!("k1"), promotion: None, to: hex!("k3") });
    }

    #[test]
    fn white_blocked_friendly() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/9/5P5/5P5/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(hex!("f5"));

        assert_eq!(result.len(), 0);
    }
    
    #[test]
    fn white_blocked_friendly_double() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4P4/11/5P5/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(hex!("f5"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: hex!("f5"), promotion: None, to: hex!("f6") });
    }

    #[test]
    fn white_blocked_hostile() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/9/5p5/5P5/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(hex!("f5"));

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn white_blocked_hostile_double() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4p4/11/5P5/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(hex!("f5"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: hex!("f5"), promotion: None, to: hex!("f6") });
    }

    #[test]
    fn black_capture() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/9/5p5/4P1P4/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(hex!("f6"));

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("g5") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("e5") });
    }

    #[test]
    fn black_capture_blocked() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/9/5p5/4p1p4/11/11/11/11 b - 0 1")
            .unwrap()
            .moves_from(hex!("f6"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f5") });
    }

    #[test]
    fn white_capture() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/9/4pPp4/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(hex!("f6"));

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
        assert_eq!(result[1], San { from: hex!("f6"), promotion: None, to: hex!("e6") });
        assert_eq!(result[2], San { from: hex!("f6"), promotion: None, to: hex!("g6") });
    }

    #[test]
    fn white_capture_blocked() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/9/4PPP4/11/11/11/11/11 w - 0 1")
            .unwrap()
            .moves_from(hex!("f6"));

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], San { from: hex!("f6"), promotion: None, to: hex!("f7") });
    }

    #[test]
    fn black_capture_en_passant_portside() {
        let result: Vec<San> = Hexchess::from("1/3/5/7/4P4/4p6/11/11/11/11/11 b f6 0 1")
            .unwrap()
            .moves_from(hex!("e6"));

        assert_eq!(result.len(), 2);
        assert_eq!(result[0], San { from: hex!("e6"), promotion: None, to: hex!("e5") });
        assert_eq!(result[1], San { from: hex!("e6"), promotion: None, to: hex!("f6") });
    }
    
    // test('black (portside out of turn)', () => {
    //     const hexchess = new Hexchess('1/3/5/7/4P4/4p6/11/11/11/11/11 w f6 0 1')

    //     expect(hexchess.moves('e6')).toEqual([
    //     { from: 'e6', to: 'e5' },
    //     // f6 is out of turn
    //     ])
    // })

    // test('black (starboard)', () => {
    //     const hexchess = new Hexchess('1/3/5/7/4P4/6p4/11/11/11/11/11 b f6 0 1')

    //     expect(hexchess.moves('g6')).toEqual([
    //     { from: 'g6', to: 'g5' },
    //     { from: 'g6', to: 'f6' }, // en passant
    //     ])
    // })

    // test('black (starboard out of turn)', () => {
    //     const hexchess = new Hexchess('1/3/5/7/4P4/6p4/11/11/11/11/11 w f6 0 1')

    //     expect(hexchess.moves('g6')).toEqual([
    //     { from: 'g6', to: 'g5' },
    //     // f6 is out of turn
    //     ])
    // })

    // test('en passant unsets on next move', () => {
    //     const hexchess = new Hexchess('1/3/5/7/8p/11/11/11/11/11/1P9 w - 0 1')

    //     hexchess.apply('b1b3')

    //     expect(hexchess.ep).toEqual('b2')

    //     hexchess.apply('k7k6')

    //     expect(hexchess.ep).toEqual(null)
    // })
    // })

    // describe('promotion', () => {
    // test('black', () => {
    //     const hexchess = new Hexchess('1/3/5/7/9/11/11/11/11/5p5/11 w - 0 1')

    //     expect(hexchess.moves('f2')).toEqual([
    //     { from: 'f2', to: 'f1', promotion: 'q' },
    //     { from: 'f2', to: 'f1', promotion: 'r' },
    //     { from: 'f2', to: 'f1', promotion: 'b' },
    //     { from: 'f2', to: 'f1', promotion: 'n' },
    //     ])
    // })

    // test('black (capture portside)', () => {
    //     const hexchess = new Hexchess('1/3/5/7/9/11/11/11/11/5p5/4rrK4 w - 0 1')

    //     expect(hexchess.moves('f2')).toEqual([
    //     { from: 'f2', to: 'g1', promotion: 'q' },
    //     { from: 'f2', to: 'g1', promotion: 'r' },
    //     { from: 'f2', to: 'g1', promotion: 'b' },
    //     { from: 'f2', to: 'g1', promotion: 'n' },
    //     ])
    // })

    // test('black (capture starboard)', () => {
    //     const hexchess = new Hexchess('1/3/5/7/9/11/11/11/11/5p5/4Krr4 w - 0 1')

    //     expect(hexchess.moves('f2')).toEqual([
    //     { from: 'f2', to: 'e1', promotion: 'q' },
    //     { from: 'f2', to: 'e1', promotion: 'r' },
    //     { from: 'f2', to: 'e1', promotion: 'b' },
    //     { from: 'f2', to: 'e1', promotion: 'n' },
    //     ])
    // })

    // test('white', () => {
    //     const hexchess = new Hexchess('1/1P1/5/7/9/11/11/11/11/11/11 w - 0 1')

    //     expect(hexchess.moves('f10')).toEqual([
    //     { from: 'f10', to: 'f11', promotion: 'q' },
    //     { from: 'f10', to: 'f11', promotion: 'r' },
    //     { from: 'f10', to: 'f11', promotion: 'b' },
    //     { from: 'f10', to: 'f11', promotion: 'n' },
    //     ])
    // })

    // test('white (capture portside)', () => {
    //     const hexchess = new Hexchess('R/kPR/5/7/9/11/11/11/11/11/11 w - 0 1')

    //     expect(hexchess.moves('f10')).toEqual([
    //     { from: 'f10', to: 'e10', promotion: 'q' },
    //     { from: 'f10', to: 'e10', promotion: 'r' },
    //     { from: 'f10', to: 'e10', promotion: 'b' },
    //     { from: 'f10', to: 'e10', promotion: 'n' },
    //     ])
    // })

    // test('white (capture starboard)', () => {
    //     const hexchess = new Hexchess('R/RPk/5/7/9/11/11/11/11/11/11 w - 0 1')

    //     expect(hexchess.moves('f10')).toEqual([
    //     { from: 'f10', to: 'g10', promotion: 'q' },
    //     { from: 'f10', to: 'g10', promotion: 'r' },
    //     { from: 'f10', to: 'g10', promotion: 'b' },
    //     { from: 'f10', to: 'g10', promotion: 'n' },
    //     ])
    // })
}
