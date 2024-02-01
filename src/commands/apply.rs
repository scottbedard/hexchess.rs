use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;

pub fn execute(hexchess_arg: String, notation_arg: String) -> Result<String, String> {
    let mut hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };
    
    let notation = match Notation::from(notation_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    match hexchess.apply(notation) {
        Ok(_) => Ok(hexchess.to_string()),
        Err(failure) => Err(failure.to_string()),
    }
}
