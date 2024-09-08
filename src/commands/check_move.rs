use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;

pub fn execute(hexchess_arg: String, notation_arg: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(_) => return Err(format!("invalid state: {}", hexchess_arg)),
    };

    let notation = match Notation::from(notation_arg.as_str()) {
        Ok(result) => result,
        Err(_) => return Err(format!("invalid notation: {}", notation_arg)),
    };

    if hexchess.all_targets().contains(&notation) {
        return Ok("ok".to_string());
    }

    return Err("illegal".to_string());
}

#[cfg(test)]
mod tests {
    use crate::app::{App, Command, handle};

    #[test]
    fn test_check_move() {
        let output = handle(App {
            command: Command::CheckMove {
                fen: crate::constants::INITIAL_HEXCHESS.to_string(),
                notation: "g4g5".to_string(),
            }
        });

        assert_eq!(Ok("ok".to_string()), output);
    }

    #[test]
    fn test_check_move_invalid() {        
        let output = handle(App {
            command: Command::CheckMove {
                fen: crate::constants::INITIAL_HEXCHESS.to_string(),
                notation: "whoops".to_string(),
            }
        });

        assert_eq!(output, Err("invalid notation: whoops".to_string()));
    }

    #[test]
    fn test_check_move_illegal() {
        let output = handle(App {
            command: Command::CheckMove {
                fen: crate::constants::INITIAL_HEXCHESS.to_string(),
                notation: "a1a2".to_string(),
            }
        });

        assert_eq!(output, Err("illegal".to_string()));
    }
}
