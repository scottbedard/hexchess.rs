use crate::game::hexchess::Hexchess;
use crate::game::notation::Notation;

// cargo run -- apply "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1 w - 0 1" "e4e5"

pub fn execute(hexchess_arg: String, notation_arg: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };
    
    let notation = match Notation::from(notation_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    Err(String::from("not implemented"))
}
