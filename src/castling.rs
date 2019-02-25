use crate::Player;

use std::convert::TryFrom;
use std::io;

#[derive(Debug, PartialEq, Eq)]
pub struct Availability {
    black_kingside: bool,
    black_queenside: bool,
    white_kingside: bool,
    white_queenside: bool,
}

pub enum Side {
    KingSide,
    QueenSide,
}

pub trait CanCastle {
    fn can_castle(&self, player: Player, side: Side) -> bool;
}

impl CanCastle for Availability {
    fn can_castle(&self, player: Player, side: Side) -> bool {
        match (player, side) {
            (Player::White, Side::KingSide)  => self.white_kingside,
            (Player::White, Side::QueenSide) => self.white_queenside,
            (Player::Black, Side::KingSide)  => self.black_kingside,
            (Player::Black, Side::QueenSide) => self.black_queenside,
        }
    }
}

impl Default for Availability {
    fn default() -> Self {
        Self {
            black_kingside: true,
            black_queenside: true,
            white_kingside: true,
            white_queenside: true,
        }
    }
}

impl TryFrom<&str> for Availability {
    type Error = io::Error;
    fn try_from(s: &str) -> Result<Availability, io::Error> {
        fn err(s: &str) -> Result<Availability, io::Error> {
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

        let mut ret = Availability {
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
