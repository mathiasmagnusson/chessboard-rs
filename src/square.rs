use std::convert::TryFrom;
use std::io;

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
            c @ b'a'...b'h' => c - b'a',
            _ => return err(s),
        };

        let rank = match s.bytes().nth(2).unwrap() {
            c @ b'1'...b'h' => c - b'1',
            _ => return err(s),
        };

        Ok(Square::new(file, rank))
    }
}
