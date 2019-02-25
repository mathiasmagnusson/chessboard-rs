use crate::castling;
use crate::Player;
use crate::Piece;
use crate::Player;
use crate::Tile;

use std::convert::TryFrom;
use std::fmt;
use std::io;

pub struct Board {
    /// Saved using the 0x88 method
    tiles: [Piece; 128],
    next_move: Player,
    castling_availablity: castling::Availability,
    en_passant_target_square: Option<Tile>,
    halfmove_clock: u64,
    fullmove_number: u64,
}

impl Default for Board {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            tiles: [                                                                                                                     // This is 'the right board' it doesn't exist and only exists as a filler (0x88)
                Piece::BRook, Piece::BKnight, Piece::BBishop, Piece::BQueen, Piece::BKing, Piece::BBishop, Piece::BKnight, Piece::BRook, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::BPawn, Piece::BPawn  , Piece::BPawn  , Piece::BPawn , Piece::BPawn, Piece::BPawn  , Piece::BPawn  , Piece::BPawn, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None , Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None , Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None , Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None , Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::WPawn, Piece::WPawn  , Piece::WPawn  , Piece::WPawn , Piece::WPawn, Piece::WPawn  , Piece::WPawn  , Piece::WPawn, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,
                Piece::WRook, Piece::WKnight, Piece::WBishop, Piece::WQueen, Piece::WKing, Piece::WBishop, Piece::WKnight, Piece::WRook, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None,

            ],
            next_move: Player::White,
            castling_availablity: Default::default(),
            en_passant_target_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}

impl TryFrom<&str> for Board {
    type Error = io::Error;
    fn try_from(fen: &str) -> Result<Board, io::Error> {
        fn err() -> Result<Board, io::Error> {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The input string did not contain valid Forsyth–Edwards Notation (https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)",
            ))
        }

        if fen.split(' ').count() != 6 || fen.split(' ').next().unwrap().split('/').count() != 8 {
            return err();
        }

        let mut fen = fen.split(' ');

        let mut tiles = [Piece::None; 128];
        let mut i = 0;
        for c in fen.next().unwrap().chars().filter(|c| *c != '/') {
            match c {
                c @ '1'...'8' => i += ((c as u8) - b'0') as usize,
                c => {
                    let file = (i % 8) as u8;
                    let rank = (i / 8) as u8;
                    tiles[Tile::new(file, rank).as_u8() as usize] = Piece::try_from(c)?;
                    i += 1;
                }
            }
        }

        let next_move = match fen.next().unwrap() {
            "w" => Player::White,
            "b" => Player::Black,
            _ => return err(),
        };

        let castling_availablity = castling::Availability::try_from(fen.next().unwrap())?;

        let en_passant_target_square = match fen.next().unwrap() {
            "-" => None,
            s => Some(Tile::try_from(s)?),
        };

        let halfmove_clock = match fen.next().unwrap().parse() {
            Ok(n) => n,
            Err(_) => return err(),
        };

        let fullmove_number = match fen.next().unwrap().parse() {
            Ok(n) => n,
            Err(_) => return err(),
        };

        Ok(Board {
            tiles,
            next_move,
            castling_availablity,
            en_passant_target_square,
            halfmove_clock,
            fullmove_number,
        })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                write!(f, "{} ", self.tiles[Tile::new(file, rank).as_u8() as usize])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl PartialEq for Board {
    fn eq(&self, rhs: &Self) -> bool {
        for (a, b) in self.tiles.iter().zip(rhs.tiles.iter()) {
            if a != b {
                return false;
            }
        }

        self.next_move == rhs.next_move
            && self.castling_availablity == rhs.castling_availablity
            && self.en_passant_target_square == rhs.en_passant_target_square
            && self.halfmove_clock == rhs.halfmove_clock
            && self.fullmove_number == rhs.fullmove_number
    }
}
