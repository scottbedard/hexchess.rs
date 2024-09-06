use crate::game::board::Position;
use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;

pub fn execute(hexchess_arg: String, position: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    let from = match Position::from(position.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    let output = hexchess.targets(from)
        .into_iter()
        .map(|x| x.to_string());

    let result_csv = output.collect::<Vec<String>>().join(",");

    Ok(result_csv)
}
