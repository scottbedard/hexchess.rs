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

    #[test]
    fn test_parse() {
        let app1 = App {
            command: Command::Parse {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
            }
        };

        let output1 = main_body(app1);

        let app2 = App {
            command: Command::Parse {
                fen: "whoops".to_string(),
            }
        };

        let output2 = main_body(app2);

        assert_eq!(output1, Ok("{\"board\":{\"a1\":null,\"a2\":null,\"a3\":null,\"a4\":null,\"a5\":null,\"a6\":null,\"b1\":\"P\",\"b2\":null,\"b3\":null,\"b4\":null,\"b5\":null,\"b6\":null,\"b7\":\"p\",\"c1\":\"R\",\"c2\":\"P\",\"c3\":null,\"c4\":null,\"c5\":null,\"c6\":null,\"c7\":\"p\",\"c8\":\"r\",\"d1\":\"N\",\"d2\":null,\"d3\":\"P\",\"d4\":null,\"d5\":null,\"d6\":null,\"d7\":\"p\",\"d8\":null,\"d9\":\"n\",\"e1\":\"Q\",\"e10\":\"q\",\"e2\":null,\"e3\":null,\"e4\":\"P\",\"e5\":null,\"e6\":null,\"e7\":\"p\",\"e8\":null,\"e9\":null,\"f1\":\"B\",\"f10\":\"b\",\"f11\":\"b\",\"f2\":\"B\",\"f3\":\"B\",\"f4\":null,\"f5\":\"P\",\"f6\":null,\"f7\":\"p\",\"f8\":null,\"f9\":\"b\",\"g1\":\"K\",\"g10\":\"k\",\"g2\":null,\"g3\":null,\"g4\":\"P\",\"g5\":null,\"g6\":null,\"g7\":\"p\",\"g8\":null,\"g9\":null,\"h1\":\"N\",\"h2\":null,\"h3\":\"P\",\"h4\":null,\"h5\":null,\"h6\":null,\"h7\":\"p\",\"h8\":null,\"h9\":\"n\",\"i1\":\"R\",\"i2\":\"P\",\"i3\":null,\"i4\":null,\"i5\":null,\"i6\":null,\"i7\":\"p\",\"i8\":\"r\",\"k1\":\"P\",\"k2\":null,\"k3\":null,\"k4\":null,\"k5\":null,\"k6\":null,\"k7\":\"p\",\"l1\":null,\"l2\":null,\"l3\":null,\"l4\":null,\"l5\":null,\"l6\":null},\"enPassant\":null,\"fullmove\":1,\"halfmove\":0,\"turn\":\"w\"}".to_string()));
        assert_eq!(output2, Err("invalid_board".to_string()));
    }
}