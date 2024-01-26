use crate::game::hexchess::Hexchess;

pub fn execute(fen: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(fen.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    panic!("hexchess: {:?}", hexchess)

//     Err("Not implemented")
}