use crate::game::board::Position;
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;
use crate::game::piece::Color;

pub fn target(hexchess: &Hexchess, from: Position, color: Color) -> Vec<Notation> {
    let mut targets: Vec<Notation> = vec![];

    for n in [1, 3, 5, 7, 9, 11] {
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
    fn test_bishop_targets() {
        let hexchess = Hexchess::from("1/3/5/7/9/5B5/11/11/11/11/11").unwrap();

        let targets = hexchess.targets(Position::F6);

        assert_eq!(targets.len(), 12);

        assert_eq!(targets[0].to_string(), "f6g7");
        assert_eq!(targets[1].to_string(), "f6h8");
    
        assert_eq!(targets[2].to_string(), "f6h5");
        assert_eq!(targets[3].to_string(), "f6k4");
    
        assert_eq!(targets[4].to_string(), "f6g4");
        assert_eq!(targets[5].to_string(), "f6h2");
    
        assert_eq!(targets[6].to_string(), "f6e4");
        assert_eq!(targets[7].to_string(), "f6d2");
    
        assert_eq!(targets[8].to_string(), "f6d5");
        assert_eq!(targets[9].to_string(), "f6b4");
    
        assert_eq!(targets[10].to_string(), "f6e7");
        assert_eq!(targets[11].to_string(), "f6d8");
    }
}
