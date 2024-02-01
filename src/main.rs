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
    /// Apply notation to game state
    Apply {
        /// Game state to apply notation to
        fen: String,

        /// Algebraic hexchess notation
        notation: String,
    },

    /// Get piece value at position
    Get {
        /// Game state
        fen: String,

        /// Position to get value of
        position: String,
    },

    /// Parse game state to JSON
    Parse {
        /// Game start to parse
        fen: String,
    },

    /// Get targets of a position
    Targets {
        /// Game state
        fen: String,

        /// Position to get targets of
        position: String,
    },
}

fn main() {
    let app = App::parse();

    let result = match app.command {
        Command::Apply { fen, notation } => commands::apply::execute(fen, notation),
        Command::Get { fen, position } => commands::get::execute(fen, position),
        Command::Parse { fen } => commands::parse::execute(fen),
        Command::Targets { fen, position } => commands::targets::execute(fen, position),
    };

    match result {
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
