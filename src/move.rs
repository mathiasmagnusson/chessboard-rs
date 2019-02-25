use crate::Tile;

use std::convert::TryFrom;
use std::io;

pub enum Move {
    Castling(Box<(Move, Move)>),
    Move { from: Tile, dest: Tile },
}

impl TryFrom<&str> for Move {
    type Error = io::Error;
    fn try_from(fen: &str) -> Result<Move, io::Error> {
        unimplemented!("not yet m8, use Move::Move {{ from: Tile::try_from(\"e2\"), to: Tile::try_from(\"e4\") }} for now instead");
    }
}
