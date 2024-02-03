use crate::game::board::Position;
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::Color;

pub fn target(hexchess: &Hexchess, from: Position, color: Color) -> Vec<Notation> {
    let mut targets: Vec<Notation> = vec![];

    for n in [0, 2, 4, 6, 8, 10] {
        hexchess.board
            .walk(from, color, n)
            .into_iter()
            .for_each(|to| targets.push(Notation { from, promotion: None, to }));
    }

    targets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_rook_targets() {
        let hexchess = Hexchess::from("1/3/5/7/9/5r5/11/11/11/11/11").unwrap();

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 30);
        
        assert_eq!(targets[0].to_string(), "f6f7");
        assert_eq!(targets[1].to_string(), "f6f8");
        assert_eq!(targets[2].to_string(), "f6f9");
        assert_eq!(targets[3].to_string(), "f6f10");
        assert_eq!(targets[4].to_string(), "f6f11");
    
        assert_eq!(targets[5].to_string(), "f6g6");
        assert_eq!(targets[6].to_string(), "f6h6");
        assert_eq!(targets[7].to_string(), "f6i6");
        assert_eq!(targets[8].to_string(), "f6k6");
        assert_eq!(targets[9].to_string(), "f6l6");
    
        assert_eq!(targets[10].to_string(), "f6g5");
        assert_eq!(targets[11].to_string(), "f6h4");
        assert_eq!(targets[12].to_string(), "f6i3");
        assert_eq!(targets[13].to_string(), "f6k2");
        assert_eq!(targets[14].to_string(), "f6l1");
    
        assert_eq!(targets[15].to_string(), "f6f5");
        assert_eq!(targets[16].to_string(), "f6f4");
        assert_eq!(targets[17].to_string(), "f6f3");
        assert_eq!(targets[18].to_string(), "f6f2");
        assert_eq!(targets[19].to_string(), "f6f1");
    
        assert_eq!(targets[20].to_string(), "f6e5");
        assert_eq!(targets[21].to_string(), "f6d4");
        assert_eq!(targets[22].to_string(), "f6c3");
        assert_eq!(targets[23].to_string(), "f6b2");
        assert_eq!(targets[24].to_string(), "f6a1");
    
        assert_eq!(targets[25].to_string(), "f6e6");
        assert_eq!(targets[26].to_string(), "f6d6");
        assert_eq!(targets[27].to_string(), "f6c6");
        assert_eq!(targets[28].to_string(), "f6b6");
        assert_eq!(targets[29].to_string(), "f6a6");
    }

    #[test]
    fn test_white_rook_targets() {
        let hexchess = Hexchess::from("1/3/5/7/9/5R5/11/11/11/11/11").unwrap();

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 30);
        
        assert_eq!(targets[0].to_string(), "f6f7");
        assert_eq!(targets[1].to_string(), "f6f8");
        assert_eq!(targets[2].to_string(), "f6f9");
        assert_eq!(targets[3].to_string(), "f6f10");
        assert_eq!(targets[4].to_string(), "f6f11");
    
        assert_eq!(targets[5].to_string(), "f6g6");
        assert_eq!(targets[6].to_string(), "f6h6");
        assert_eq!(targets[7].to_string(), "f6i6");
        assert_eq!(targets[8].to_string(), "f6k6");
        assert_eq!(targets[9].to_string(), "f6l6");
    
        assert_eq!(targets[10].to_string(), "f6g5");
        assert_eq!(targets[11].to_string(), "f6h4");
        assert_eq!(targets[12].to_string(), "f6i3");
        assert_eq!(targets[13].to_string(), "f6k2");
        assert_eq!(targets[14].to_string(), "f6l1");
    
        assert_eq!(targets[15].to_string(), "f6f5");
        assert_eq!(targets[16].to_string(), "f6f4");
        assert_eq!(targets[17].to_string(), "f6f3");
        assert_eq!(targets[18].to_string(), "f6f2");
        assert_eq!(targets[19].to_string(), "f6f1");
    
        assert_eq!(targets[20].to_string(), "f6e5");
        assert_eq!(targets[21].to_string(), "f6d4");
        assert_eq!(targets[22].to_string(), "f6c3");
        assert_eq!(targets[23].to_string(), "f6b2");
        assert_eq!(targets[24].to_string(), "f6a1");
    
        assert_eq!(targets[25].to_string(), "f6e6");
        assert_eq!(targets[26].to_string(), "f6d6");
        assert_eq!(targets[27].to_string(), "f6c6");
        assert_eq!(targets[28].to_string(), "f6b6");
        assert_eq!(targets[29].to_string(), "f6a6");
    }
}
