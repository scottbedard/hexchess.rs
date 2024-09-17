use crate::game::hexchess::Hexchess;

pub fn execute(hexchess_arg: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(_) => return Err(format!("invalid state: {}", hexchess_arg)),
    };

    if hexchess.is_stalemate() {
        return Ok("stalemate".to_string());
    }

    if hexchess.is_checkmate() {
        return Ok("checkmate".to_string());
    }

    Ok(hexchess.turn.to_string())
}

#[cfg(test)]
mod tests {
    use crate::app::{App, Command, handle};

    #[test]
    fn test_get_status_w() {
        let output = handle(App {
            command: Command::GetStatus {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1".to_string(),
            }
        });

        assert_eq!(Ok("w".to_string()), output);
    }

    #[test]
    fn test_get_status_b() {
        let output = handle(App {
            command: Command::GetStatus {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 b - 0 1".to_string(),
            }
        });

        assert_eq!(Ok("b".to_string()), output);
    }

    #[test]
    fn test_get_status_stalemate() {
        let output = handle(App {
            command: Command::GetStatus {
                fen: "k/1P1/2K2/7/9/11/11/11/11/11/11 b - 0 1".to_string(),
            }
        });

        assert_eq!(Ok("stalemate".to_string()), output);
    }

    #[test]
    fn test_get_status_checkmate() {
        let output = handle(App {
            command: Command::GetStatus {
                fen: "b/qb1/n1b1n/r5r/ppppppppp/6N4/5P1PB2/4P1P4/1P1P5Q1/2P4BP1k/R2N1BK1RP1 b - 10 9".to_string(),
            }
        });

        assert_eq!(Ok("checkmate".to_string()), output);
    }
}
