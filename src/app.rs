use clap::{Parser, Subcommand};
use crate::commands;

#[derive(Debug, Parser)]
#[clap(name = "hexchess", version)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Apply sequence of moves to a position
    Apply {
        /// Hexchess state
        fen: String,

        /// Algebraic hexchess notation
        sequence: String,
    },

    /// Get legal moves
    GetTargets {
        /// Hexchess state
        fen: String,

        /// Position to get targets from
        #[arg(short, long, default_value = None)]
        position: Option<String>,
    },

    /// Parse hexchess fen to JSON
    Parse {
        /// Hexchess state
        fen: String,
    },

    /// Test if a move is legal
    TestMove {
        /// Hexchess state
        fen: String,

        /// Move notation
        notation: String,
    },
}

pub fn handle(app: App) -> Result<String, String> {
    match app.command {
        Command::Apply { fen, sequence } => commands::apply::execute(fen, sequence),
        Command::GetTargets { fen, position } => commands::get_targets::execute(fen, position),
        Command::Parse { fen } => commands::parse::execute(fen),
        Command::TestMove { fen, notation } => commands::test_move::execute(fen, notation),
    }
}
