use crate::game::hexchess::Hexchess;

pub fn execute(fen: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(fen.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    Ok(hexchess.to_json())
}

#[cfg(test)]
mod tests {
    use crate::app::{App, Command, handle};

    #[test]
    fn test_parse() {
        let output = handle(App {
            command: Command::Parse {
                fen: "b/qbk/n1b1n/r5r/ppppppppp/11/5P5/4P1P4/3P1B1P3/2P2B2P2/1PRNQBKNRP1".to_string(),
            }
        });

        assert_eq!(output, Ok("{\"board\":{\"a1\":null,\"a2\":null,\"a3\":null,\"a4\":null,\"a5\":null,\"a6\":null,\"b1\":\"P\",\"b2\":null,\"b3\":null,\"b4\":null,\"b5\":null,\"b6\":null,\"b7\":\"p\",\"c1\":\"R\",\"c2\":\"P\",\"c3\":null,\"c4\":null,\"c5\":null,\"c6\":null,\"c7\":\"p\",\"c8\":\"r\",\"d1\":\"N\",\"d2\":null,\"d3\":\"P\",\"d4\":null,\"d5\":null,\"d6\":null,\"d7\":\"p\",\"d8\":null,\"d9\":\"n\",\"e1\":\"Q\",\"e10\":\"q\",\"e2\":null,\"e3\":null,\"e4\":\"P\",\"e5\":null,\"e6\":null,\"e7\":\"p\",\"e8\":null,\"e9\":null,\"f1\":\"B\",\"f10\":\"b\",\"f11\":\"b\",\"f2\":\"B\",\"f3\":\"B\",\"f4\":null,\"f5\":\"P\",\"f6\":null,\"f7\":\"p\",\"f8\":null,\"f9\":\"b\",\"g1\":\"K\",\"g10\":\"k\",\"g2\":null,\"g3\":null,\"g4\":\"P\",\"g5\":null,\"g6\":null,\"g7\":\"p\",\"g8\":null,\"g9\":null,\"h1\":\"N\",\"h2\":null,\"h3\":\"P\",\"h4\":null,\"h5\":null,\"h6\":null,\"h7\":\"p\",\"h8\":null,\"h9\":\"n\",\"i1\":\"R\",\"i2\":\"P\",\"i3\":null,\"i4\":null,\"i5\":null,\"i6\":null,\"i7\":\"p\",\"i8\":\"r\",\"k1\":\"P\",\"k2\":null,\"k3\":null,\"k4\":null,\"k5\":null,\"k6\":null,\"k7\":\"p\",\"l1\":null,\"l2\":null,\"l3\":null,\"l4\":null,\"l5\":null,\"l6\":null},\"enPassant\":null,\"fullmove\":1,\"halfmove\":0,\"turn\":\"w\"}".to_string()));
    }

    #[test]
    fn test_parse_invalid_fen() {        
        let output = handle(App {
            command: Command::Parse {
                fen: "whoops".to_string(),
            }
        });

        assert_eq!(output, Err("invalid board: whoops".to_string()));
    }
}
