use crate::game::hexchess::Hexchess;

pub fn execute(fen: String) -> Result<String, String> {
    match Hexchess::from(fen.as_str()) {
        Ok(result) => Ok(result.to_json()),
        Err(failure) => Err(failure.to_string()),
    }
}