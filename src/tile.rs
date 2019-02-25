use std::convert::TryFrom;
use std::io;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Tile(u8);

impl Tile {
    pub fn new(file: File, rank: Rank) -> Self {
        Self(0b00000000 | file as u8 | (rank as u8) << 4)
    }
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

impl TryFrom<&str> for Tile {
    type Error = io::Error;
    fn try_from(s: &str) -> Result<Tile, io::Error> {
        fn err(s: &str) -> Result<Tile, io::Error> {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("'{}' is not a valid square. Valid ones are a letter 'a' through 'h' followed by a number 1 to 8, ex: c6", s),
            ))
        }

        if s.bytes().len() != 2 {
            return err(s);
        }

        let file = match s.bytes().next().unwrap() {
            c @ b'a' ... b'h' => File::from(c - b'a'),
            _ => return err(s),
        };

        let rank = match s.bytes().nth(1).unwrap() {
            c @ b'1' ... b'8' => Rank::from(c - b'1'),
            _ => return err(s),
        };

        Ok(Tile::new(file, rank))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl From<u8> for Rank {
    fn from(n: u8) -> Self {
        match n {
            n @ 0 ... 7 => unsafe { std::mem::transmute(n) },
            // 0 => Rank::One,
            // 1 => Rank::Two,
            // 2 => Rank::Three,
            // 3 => Rank::Four,
            // 4 => Rank::Five,
            // 5 => Rank::Six,
            // 6 => Rank::Seven,
            // 7 => Rank::Eight,
            _ => panic!("A rank conversion must be between 0 and 7, inclusive"),
        }
    }
}

impl From<u8> for File {
    fn from(n: u8) -> Self {
        match n {
            n @ 0 ... 7 => unsafe { std::mem::transmute(n) },
            // 0 => File::A
            // 1 => File::B
            // 2 => File::C
            // 3 => File::D
            // 4 => File::E
            // 5 => File::F
            // 6 => File::G
            // 7 => File::H
            _ => panic!("A file conversion must be between 0 and 7, inclusive"),
        }
    }
}