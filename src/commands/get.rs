use crate::game::board::Position;
use crate::game::hexchess::Hexchess;

pub fn execute(fen: String, position: String) -> Result<String, String> {
    let hexchess = match Hexchess::parse(fen.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    let normalized_position = match Position::from(position.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    match hexchess.board.get(normalized_position) {
        Some(result) => Ok(result.to_string()),
        None => Ok(String::from("")),
    }
}
