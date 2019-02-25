#![feature(try_from)]

use std::convert::TryFrom;
use std::fmt;
use std::io;

pub struct Board {
    /// Saved using the 0x88 method
    tiles: [Piece; 128],
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

impl Square {
    pub fn new(file: u8, rank: u8) -> Self {
        assert!(file < 8);
        assert!(rank < 8);

        Self(0b00000000 | file | rank << 4)
    }
    pub fn as_u8(&self) -> u8 {
        self.0
    }
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

        let mut tiles = [Piece::None; 128];
        let mut i = 0;
        for c in fen.next().unwrap().chars().filter(|c| *c != '/') {
            match c {
                c @ '1'...'8' => i += ((c as u8) - b'0') as usize,
                c => {
                    let file = (i % 8) as u8;
                    let rank = (i / 8) as u8;
                    tiles[Square::new(file, rank).as_u8() as usize] = Piece::try_from(c)?;
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
        fn err(s: &str) -> Result<Square, io::Error> {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{}' is not a valid square. Valid ones are a letter 'a' through 'h' followed by a number 1 to 8, ex: c6", s),
            ))
        }

        if s.bytes().len() != 2 {
            return err(s);
        }

        let file = match s.bytes().next().unwrap() {
            c @ b'a' ... b'h' => c - b'a',
            _ => return err(s),
        };

        let rank = match s.bytes().nth(2).unwrap() {
            c @ b'1' ... b'h' => c - b'1',
            _ => return err(s),
        };

        Ok(Square::new(file, rank))
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in 0..8 {
            for file in 0..8 {
                write!(f, "{} ", self.tiles[Square::new(file, rank).as_u8() as usize])?;
            }
            writeln!(f)?;
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
    fn square_shit_is_correct() {
        let rank = 0b010;
        let file = 0b101;

        let square = Square::new(file, rank);

        assert_eq!(square.as_u8(), 0b_0010_0101);
    }

    #[test]
    fn pen_notation_starting_position_gives_the_same_result_as_default_implementation() {
        let pen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let pen_board = Board::try_from(pen).unwrap();
        let def_board = Board::default();

        assert_eq!(format!("{}", pen_board), format!("{}", def_board), "\n{}\n", pen_board);
    }
}
