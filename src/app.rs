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
    /// Get all legal moves
    AllTargets {
        /// Hexchess state
        fen: String,
    },

    /// Apply sequence of moves to a position
    ApplySequence {
        /// Hexchess state
        fen: String,

        /// Algebraic hexchess notation
        sequence: String,
    },

    /// Get legal moves from a position
    GetTargets {
        /// Hexchess state
        fen: String,

        /// Hexchess coordinate
        position: String,
    },

    /// Parse hexchess fen to JSON
    Parse {
        /// Hexchess state
        fen: String,
    },
}

pub fn handle(app: App) -> Result<String, String> {
    match app.command {
        Command::AllTargets { fen } => commands::all_targets::execute(fen),
        Command::ApplySequence { fen, sequence } => commands::apply_sequence::execute(fen, sequence),
        Command::GetTargets { fen, position } => commands::get_targets::execute(fen, position),
        Command::Parse { fen } => commands::parse::execute(fen),
    }
}
