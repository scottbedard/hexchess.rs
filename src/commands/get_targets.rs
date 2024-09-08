use crate::game::board::Position;
use crate::game::hexchess::Hexchess;

pub fn execute(hexchess_arg: String, position: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    let from = match Position::from(position.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    let csv = hexchess
        .targets(from)
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Ok(csv)
}

#[cfg(test)]
mod tests {
    use crate::app::{App, Command, handle};

    #[test]
    fn test_get_targets() {
        let output1 = handle(App {
            command: Command::GetTargets {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                position: "g4".to_string(),
            }
        });

        assert_eq!(output1, Ok("g4g5,g4g6".to_string()));
    }

    #[test]
    fn test_get_targets_invalid_fen() {
        let output = handle(App {
            command: Command::GetTargets {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                position: "whoops".to_string(),
            }
        });

        assert_eq!(output, Err("invalid position: whoops".to_string()));

    }

    #[test]
    fn test_get_targets_invalid_position() {
        let output = handle(App {
            command: Command::GetTargets {
                fen: "whoops".to_string(),
                position: "g4".to_string(),
            }
        });

        assert_eq!(output, Err("invalid board: whoops".to_string()));
    }
}
