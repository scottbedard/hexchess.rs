use crate::game::board::{Position, get_step};
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::Color;

pub fn target(hexchess: &Hexchess, position: Position, color: Color) -> Vec<Notation> {
    let mut targets: Vec<Notation> = vec![];

    match advance_one(hexchess, position, color) {
        Some(notation) => {
            targets.push(notation);

            if is_starting_position(position, color) {
                match advance_two(hexchess, position, notation.to, color) {
                    Some(notation) => targets.push(notation),
                    None => (),
                }
            }
        },
        None => (),
    }

    targets
}

fn advance_one(hexchess: &Hexchess, position: Position, color: Color) -> Option<Notation> {
    let forward_direction = match color {
        Color::White => 0,
        Color::Black => 6,
    };

    let forward_position = match get_step(position, forward_direction) {
        Some(position) => position,
        None => return None,
    };
    
    match hexchess.board.get(forward_position) {
        Some(_) => None,
        None => return Some(Notation {
            from: position,
            to: forward_position,
            promotion: None,
        }),
    }
}

fn advance_two(hexchess: &Hexchess, from_position: Position, to_position: Position, color: Color) -> Option<Notation> {
    let forward_direction = match color {
        Color::White => 0,
        Color::Black => 6,
    };

    let forward_position = match get_step(to_position, forward_direction) {
        Some(position) => position,
        None => return None,
    };
    
    match hexchess.board.get(forward_position) {
        Some(_) => None,
        None => return Some(Notation {
            from: from_position,
            to: forward_position,
            promotion: None,
        }),
    }
}

fn is_starting_position(position: Position, color: Color) -> bool {
    match color {
        Color::Black => match position {
            Position::B7 | Position::C7 | Position::D7 | Position::E7 | Position::F7 | Position::G7 | Position::H7 | Position::I7 | Position::K7 => true,
            _ => false,
        },
        Color::White => match position {
            Position::B1 | Position::C2 | Position::D3 | Position::E4 | Position::F5 | Position::G4 | Position::H3 | Position::I2 | Position::K1 => true,
            _ => false,
        },
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::piece::Piece;

    #[test]
    fn test_black_pawn_advance() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F7, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F7);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[0].to_string(), "f7f6");
        assert_eq!(targets[1].to_string(), "f7f5");
    }

    #[test]
    fn test_white_pawn_advance() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F5, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F5);

        assert_eq!(targets.len(), 2);

        assert_eq!(targets[0].to_string(), "f5f6");
        assert_eq!(targets[1].to_string(), "f5f7");
    }

    #[test]
    fn test_advance_one_blocked() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));
        hexchess.board.set(Position::F7, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F5);

        assert_eq!(targets.len(), 0);
    }

    #[test]
    fn test_advance_two_blocked() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F5, Some(Piece::WhitePawn));
        hexchess.board.set(Position::F7, Some(Piece::BlackPawn));

        let targets = hexchess.targets(Position::F5);

        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].to_string(), "f5f6");
    }

    #[test]
    fn test_no_double_move_off_non_starting_position() {
        let mut hexchess = Hexchess::new();

        hexchess.board.set(Position::F6, Some(Piece::WhitePawn));

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 1);
    }
    // it('no double-move off non-starting position', () => {
    //   const game = new Hexchess

    //   game.board.f6 = Pieces.WhitePawn
    
    //   const targets = game.getTargets('f6')
    
    //   expect(targets.length).toBe(1)
    //   expect(targets[0].toString()).toBe('f6f7')
    // })

    // it('black capture portside', () => {
    //   const game = new Hexchess
    //   game.turn = Colors.Black
    //   game.board.f6 = Pieces.BlackPawn
    //   game.board.g5 = Pieces.WhitePawn

    //   const targets = game.getTargets('f6')
    
    //   expect(targets.length).toBe(2)
    //   expect(targets[1].toString()).toBe('f6g5')
    // })

    // it('black capture starboard', () => {
    //   const game = new Hexchess
    //   game.turn = Colors.Black
    //   game.board.f6 = Pieces.BlackPawn
    //   game.board.e5 = Pieces.WhitePawn

    //   const targets = game.getTargets('f6')
    
    //   expect(targets.length).toBe(2)
    //   expect(targets[1].toString()).toBe('f6e5')
    // })

    // it('black capture en passant', () => {
    //   const game = new Hexchess
    //   game.board.f6 = Pieces.BlackPawn
    //   game.board.g4 = Pieces.WhitePawn

    //   game.turn = Colors.White
    //   game.applyNotation(['g4g6'])

    //   expect(game.enPassant).toBe('g5')

    //   const targets = game.getTargets('f6')

    //   expect(targets.length).toBe(2)
    //   expect(targets[1]).toEqual({ from: 'f6', to: 'g5', promotion: null })

    //   game.applyNotation(['f6g5'])
    //   expect(game.enPassant).toBe(null)
    // })

    // it('white capture portside', () => {
    //   const game = new Hexchess
    //   game.turn = Colors.White
    //   game.board.f6 = Pieces.WhitePawn
    //   game.board.e6 = Pieces.BlackPawn

    //   const targets = game.getTargets('f6')
    
    //   expect(targets.length).toBe(2)
    //   expect(targets[1].toString()).toBe('f6e6')
    // })

    // it('white capture starboard', () => {
    //   const game = new Hexchess
    //   game.turn = Colors.White
    //   game.board.f6 = Pieces.WhitePawn
    //   game.board.g6 = Pieces.BlackPawn

    //   const targets = game.getTargets('f6')
    
    //   expect(targets.length).toBe(2)
    //   expect(targets[1].toString()).toBe('f6g6')
    // })

    // it('white capture en passant', () => {
    //   const game = new Hexchess
    //   game.board.f6 = Pieces.WhitePawn
    //   game.board.e7 = Pieces.BlackPawn

    //   game.turn = Colors.Black
    //   game.applyNotation(['e7e5'])

    //   expect(game.enPassant).toBe('e6')

    //   const targets = game.getTargets('f6')

    //   expect(targets.length).toBe(2)
    //   expect(targets[1]).toEqual({ from: 'f6', to: 'e6', promotion: null })

    //   game.applyNotation(['f6e6'])
    //   expect(game.enPassant).toBe(null)
    // })

    // it('en passant cannot capture friendly pieces', () => {
    //   const game = Hexchess.init()

    //   game.applyNotation('g4g6')

    //   const targets = game.getTargets('f5')
    
    //   expect(targets.some(n => n.to === game.enPassant)).toBe(false)
    // })

    // it('promote to queen', () => {
    //   const game = new Hexchess

    //   game.board.f10 = Pieces.WhitePawn

    //   expect(game.getTargets('f10').map(String)).toContainEqual('f10f11q')

    //   game.applyNotation(['f10f11q'])

    //   expect(game.board.f11).toBe(Pieces.WhiteQueen)
    // })

    // it('promote to knight', () => {
    //   const game = new Hexchess

    //   game.board.f10 = Pieces.WhitePawn

    //   expect(game.getTargets('f10').map(String)).toContainEqual('f10f11n')

    //   game.applyNotation(['f10f11n'])

    //   expect(game.board.f11).toBe(Pieces.WhiteKnight)
    // })

    // it('promote to rook', () => {
    //   const game = new Hexchess

    //   game.board.f10 = Pieces.WhitePawn

    //   expect(game.getTargets('f10').map(String)).toContainEqual('f10f11r')

    //   game.applyNotation(['f10f11r'])

    //   expect(game.board.f11).toBe(Pieces.WhiteRook)
    // })
    
    // it('promote to bishop', () => {
    //   const game = new Hexchess

    //   game.board.f10 = Pieces.WhitePawn

    //   expect(game.getTargets('f10').map(String)).toContainEqual('f10f11b')

    //   game.applyNotation(['f10f11b'])

    //   expect(game.board.f11).toBe(Pieces.WhiteBishop)
    // })

    // it('promote pawn fails', () => {
    //   const game = new Hexchess

    //   game.board.f10 = Pieces.WhitePawn

    //   expect(game.getTargets('f10').map(String)).not.toContainEqual('f10f11p')

    //   expect(() => game.applyNotation(['f10f11p'])).toThrow()
    // })

    // it('promote king fails', () => {
    //   const game = new Hexchess

    //   game.board.f10 = Pieces.WhitePawn

    //   expect(game.getTargets('f10').map(String)).not.toContainEqual('f10f11k')

    //   expect(() => game.applyNotation(['f10f11k'])).toThrow()
    // })
}
