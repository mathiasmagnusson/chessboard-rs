use std::convert::TryFrom;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub struct CastlingAvailability {
    black_kingside: bool,
    black_queenside: bool,
    white_kingside: bool,
    white_queenside: bool,
}

impl Default for CastlingAvailability {
    fn default() -> Self {
        Self {
            black_kingside: true,
            black_queenside: true,
            white_kingside: true,
            white_queenside: true,
        }
    }
}

impl TryFrom<&str> for CastlingAvailability {
    type Error = io::Error;
    fn try_from(s: &str) -> Result<CastlingAvailability, io::Error> {
        fn err(s: &str) -> Result<CastlingAvailability, io::Error> {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "{} contains invalid syntax for castling availability according to Forsyth–Edwards Notation (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)",
                    s
                ),
            ))
        }

        if s.len() < 1 || s.len() > 4 {
            return err(s);
        }

        let mut ret = CastlingAvailability {
            black_kingside: false,
            black_queenside: false,
            white_kingside: false,
            white_queenside: false,
        };

        if s.chars().next().unwrap() == '=' {
            return Ok(ret);
        }

        for c in s.chars() {
            match c {
                'k' => ret.black_kingside = true,
                'q' => ret.black_queenside = true,
                'K' => ret.white_kingside = true,
                'Q' => ret.white_queenside = true,
                _ => return err(s),
            }
        }

        Ok(ret)
    }
}
