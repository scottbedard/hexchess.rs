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
    Apply {
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

    /// Parse hexchess to JSON
    Parse {
        /// Hexchess state
        fen: String,
    },

    /// List target moves from a position
    Targets {
        /// Hexchess state
        fen: String,

        /// Hexchess coordinate
        position: String,
    },
}

fn main_body(app: App) -> Result<String, String> {
    match app.command {
        Command::Apply { fen, sequence } => commands::apply::execute(fen, sequence),
        Command::Get { fen, position } => commands::get::execute(fen, position),
        Command::Parse { fen } => commands::parse::execute(fen),
        Command::Targets { fen, position } => commands::targets::execute(fen, position),
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
        let app = App {
            command: Command::Apply {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
                sequence: "g4g5 e7e6".to_string(),
            }
        };

        let output = main_body(app);

        assert_eq!(output, Ok("b/qbk/n1b1n/r5r/ppp1ppppp/4p6/5PP4/4P6/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 2".to_string()));
    }
}