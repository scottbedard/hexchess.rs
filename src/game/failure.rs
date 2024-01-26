use serde::{Deserialize, Serialize};

/// Known hexchess failure
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Failure {
    #[serde(alias = "invalid_board")]
    InvalidBoard,

    #[serde(alias = "invalid_color")]
    InvalidColor,

    #[serde(alias = "invalid_piece")]
    InvalidPiece,

    #[serde(alias = "invalid_position")]
    InvalidPosition,
}
