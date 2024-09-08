use crate::game::hexchess::Hexchess;

pub fn execute(hexchess_arg: String, sequence_arg: String) -> Result<String, String> {
    let mut hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    match hexchess.apply_sequence(&sequence_arg) {
        Ok(_) => Ok(hexchess.to_string()),
        Err(failure) => Err(failure),
    }
}

#[cfg(test)]
mod tests {
    use crate::app::{App, Command, handle};

    #[test]
    fn test_apply_sequence() {
        let output = handle(App {
            command: Command::ApplySequence {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                sequence: "g4g5 e7e6".to_string(),
            }
        });


        assert_eq!(output, Ok("b/qbk/n1b1n/r5r/ppp1ppppp/4p6/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 2".to_string()));
    }

    #[test]
    fn test_apply_sequence_invalid_fen() {
        let output = handle(App {
            command: Command::ApplySequence {
                fen: "whoops".to_string(),
                sequence: "g4g5 e7e6".to_string(),
            }
        });
        
        assert_eq!(output, Err("invalid board: whoops".to_string()));
    }

    #[test]
    fn test_apply_sequence_invalid_sequence() {
        let output = handle(App {
            command: Command::ApplySequence {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                sequence: "whoops".to_string(),
            }
        });

        assert_eq!(output, Err("invalid notation at index 0: whoops".to_string()));
    }
}
