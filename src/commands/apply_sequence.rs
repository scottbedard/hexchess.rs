use crate::game::hexchess::Hexchess;

pub fn execute(hexchess_arg: String, sequence_arg: String) -> Result<String, String> {
    let mut hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    match hexchess.apply_sequence(&sequence_arg) {
        Ok(_) => Ok(hexchess.to_string()),
        Err(failure) => Err(failure),
    }
}
