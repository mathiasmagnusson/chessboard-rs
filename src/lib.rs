#![feature(try_from)]

use std::convert::TryFrom;
use std::fmt;
use std::io;

pub struct Board {
    tiles: [Piece; 8 * 8],
    next_move: Color,
    castling_availablity: CastlingAvailability,
    en_passant_target_square: Option<Square>,
    halfmove_clock: u64,
    fullmove_number: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    None = 0,
    WPawn = 1,
    BPawn = -1,
    WBishop = 2,
    BBishop = -2,
    WKnight = 3,
    BKnight = -3,
    WRook = 4,
    BRook = -4,
    WQueen = 5,
    BQueen = -5,
    WKing = 6,
    BKing = -6,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CastlingAvailability {
    black_kingside: bool,
    black_queenside: bool,
    white_kingside: bool,
    white_queenside: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Square(u8);

impl Default for Board {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            tiles: [
                Piece::BRook, Piece::BKnight, Piece::BBishop, Piece::BQueen, Piece::BKing, Piece::BBishop, Piece::BKnight, Piece::BRook,
                Piece::BPawn, Piece::BPawn  , Piece::BPawn  , Piece::BPawn , Piece::BPawn, Piece::BPawn  , Piece::BPawn  , Piece::BPawn,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None ,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None ,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None ,
                Piece::None , Piece::None   , Piece::None   , Piece::None  , Piece::None , Piece::None   , Piece::None   , Piece::None ,
                Piece::WPawn, Piece::WPawn  , Piece::WPawn  , Piece::WPawn , Piece::WPawn, Piece::WPawn  , Piece::WPawn  , Piece::WPawn,
                Piece::WRook, Piece::WKnight, Piece::WBishop, Piece::WQueen, Piece::WKing, Piece::WBishop, Piece::WKnight, Piece::WRook,
            ],
            next_move: Color::White,
            castling_availablity: Default::default(),
            en_passant_target_square: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
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

        let mut tiles = [Piece::None; 8 * 8];
        let mut i = 0;
        for c in fen.next().unwrap().chars().filter(|c| *c != '/') {
            match c {
                c @ '1'...'8' => i += ((c as u8) - b'0') as usize,
                c => {
                    tiles[i] = Piece::try_from(c)?;
                    i += 1;
                }
            }
        }

        let next_move = match fen.next().unwrap() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return err(),
        };

        let castling_availablity = CastlingAvailability::try_from(fen.next().unwrap())?;

        let en_passant_target_square = match fen.next().unwrap() {
            "-" => None,
            s => Some(Square::try_from(s)?),
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

impl TryFrom<char> for Piece {
    type Error = io::Error;
    fn try_from(c: char) -> Result<Piece, io::Error> {
        match c {
            'p' => Ok(Piece::BPawn),
            'b' => Ok(Piece::BBishop),
            'n' => Ok(Piece::BKnight),
            'r' => Ok(Piece::BRook),
            'q' => Ok(Piece::BQueen),
            'k' => Ok(Piece::BKing),
            'P' => Ok(Piece::WPawn),
            'B' => Ok(Piece::WBishop),
            'N' => Ok(Piece::WKnight),
            'R' => Ok(Piece::WRook),
            'Q' => Ok(Piece::WQueen),
            'K' => Ok(Piece::WKing),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "m8, das not gododod (TODO: change this error message)",
            )),
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

impl TryFrom<&str> for Square {
    type Error = io::Error;
    fn try_from(s: &str) -> Result<Square, io::Error> {
        fn err() -> Result<Square, io::Error> {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "{} is not a valid square. Valid ones are a letter 'a' through 'h' followed by a number 1 to 8, ex: c6",
            ))
        }

        if s.len() != 2 {
            return err();
        }

        let file = match s.chars().next().unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return err(),
        };

        let rank = match s.chars().nth(2).unwrap() {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return err(),
        };

        Ok(Square(rank * 8 + file))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, p) in self.tiles.iter().enumerate() {
            write!(f, "{}", p)?;
            if i % 8 == 7 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Piece::BPawn => 'p',
                Piece::BBishop => 'b',
                Piece::BKnight => 'n',
                Piece::BRook => 'r',
                Piece::BQueen => 'q',
                Piece::BKing => 'k',
                Piece::WPawn => 'P',
                Piece::WBishop => 'B',
                Piece::WKnight => 'N',
                Piece::WRook => 'R',
                Piece::WQueen => 'Q',
                Piece::WKing => 'K',
                Piece::None => '*',
            }
        )?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0b11111111 ^ 7, 0xf8);
    }

    #[test]
    fn pen_notation_starting_position_gives_the_same_result_as_default_implementation() {
        let pen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let pen_board = Board::try_from(pen).unwrap();
        let def_board = Board::default();

        assert_eq!(format!("{}", pen_board), format!("{}", def_board));
    }
}
