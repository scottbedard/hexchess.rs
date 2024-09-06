use clap::{Parser, Subcommand};
use exitcode::{DATAERR, OK};

mod commands;
mod constants;
mod game;

#[derive(Debug, Parser)]
#[clap(name = "hexchess", version)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Apply a sequence of moves to a game
    ApplySequence {
        /// Hexchess state
        fen: String,

        /// Algebraic hexchess notation
        sequence: String,
    },

    /// Get piece value at position
    Get {
        /// Hexchess state
        fen: String,

        /// Hexchess coordinate
        position: String,
    },

    /// Get targets from a position
    GetTargets {
        /// Hexchess state
        fen: String,

        /// Hexchess coordinate
        position: String,
    },

    /// Parse hexchess to JSON
    Parse {
        /// Hexchess state
        fen: String,
    },
}

fn main_body(app: App) -> Result<String, String> {
    match app.command {
        Command::ApplySequence { fen, sequence } => commands::apply_sequence::execute(fen, sequence),
        Command::Get { fen, position } => commands::get::execute(fen, position),
        Command::GetTargets { fen, position } => commands::get_targets::execute(fen, position),
        Command::Parse { fen } => commands::parse::execute(fen),
    }
}

fn main() {
    let app = App::parse();

    match main_body(app) {
        Ok(output) => {
            println!("{}", output);
            std::process::exit(OK);
        },
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(DATAERR);
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_sequence() {
        let app1 = App {
            command: Command::ApplySequence {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                sequence: "g4g5 e7e6".to_string(),
            }
        };

        let app2 = App {
            command: Command::ApplySequence {
                fen: "whoops".to_string(),
                sequence: "g4g5 e7e6".to_string(),
            }
        };

        let app3 = App {
            command: Command::ApplySequence {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                sequence: "whoops".to_string(),
            }
        };

        let output1 = main_body(app1);
        let output2 = main_body(app2);
        let output3 = main_body(app3);

        assert_eq!(output1, Ok("b/qbk/n1b1n/r5r/ppp1ppppp/4p6/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 2".to_string()));
        assert_eq!(output2, Err("invalid_board".to_string()));
        assert_eq!(output3, Err("Invalid notation at index 0: whoops".to_string()));
    }

    #[test]
    fn test_get_targets() {
        let app1 = App {
            command: Command::GetTargets {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                position: "g4".to_string(),
            }
        };

        let output1 = main_body(app1);

        let app2 = App {
            command: Command::GetTargets {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                position: "whoops".to_string(),
            }
        };

        let output2 = main_body(app2);

        let app3 = App {
            command: Command::GetTargets {
                fen: "whoops".to_string(),
                position: "g4".to_string(),
            }
        };

        let output3 = main_body(app3);

        assert_eq!(output1, Ok("g4g5,g4g6".to_string()));
        assert_eq!(output2, Err("invalid_position".to_string()));
        assert_eq!(output3, Err("invalid_board".to_string()));
    }
}