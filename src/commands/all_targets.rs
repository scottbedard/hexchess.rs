use crate::game::hexchess::Hexchess;

pub fn execute(hexchess_arg: String) -> Result<String, String> {
    let hexchess = match Hexchess::from(hexchess_arg.as_str()) {
        Ok(result) => result,
        Err(failure) => return Err(failure.to_string()),
    };

    let csv = hexchess
        .all_targets()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Ok(csv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Command, handle};
    use crate::constants::INITIAL_HEXCHESS;

    #[test]
    fn test_all_targets() {
        let output = handle(App {
            command: Command::AllTargets {
                fen: INITIAL_HEXCHESS.to_string(),
            }
        });
        
        let targets_csv = match output {
            Ok(result) => result,
            Err(failure) => panic!("{}", failure),
        };

        let targets_vec: Vec<String> = targets_csv.split(",").map(|s| s.to_string()).collect();

        assert_eq!(targets_vec.len(), 51);
        assert_eq!("f5f6,e4e5,e4e6,g4g5,g4g6,d3d4,d3d5,f3h2,f3d2,h3h4,h3h5,c2c3,c2c4,f2g3,f2h4,f2i5,f2k6,f2e3,f2d4,f2c5,f2b6,i2i3,i2i4,b1b2,b1b3,c1d2,c1e3,c1f4,d1f4,d1g2,d1b2,d1c3,e1e2,e1e3,e1d2,e1c3,e1b4,e1a5,f1g2,f1e2,g1g2,g1h2,h1i3,h1k2,h1e2,h1f4,i1h2,i1g3,i1f4,k1k2,k1k3", targets_csv);
    }
}
