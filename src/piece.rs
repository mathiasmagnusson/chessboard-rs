use std::convert::TryFrom;
use std::fmt;
use std::io;

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
