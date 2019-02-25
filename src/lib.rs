#![feature(try_from)]

mod board;
mod r#move;
mod piece;
mod player;
mod tile;

pub use board::Board;
pub use piece::Piece;
pub use player::Player;
pub use r#move::Move;
pub use tile::File;
pub use tile::Rank;
pub use tile::Tile;

pub mod castling;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn tile_shit_is_correct() {
        //                   0b_0101  0b_0010
        let tile1 = Tile::new(File::F, Rank::Three).as_u8();
        let tile2 = Tile::try_from("f3").unwrap().as_u8();

        assert_eq!(tile1, 0b_0010_0101);
        assert_eq!(tile1, tile2);
    }

    #[test]
    fn pen_notation_starting_position_gives_the_same_result_as_default_implementation() {
        let pen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let pen_board = Board::try_from(pen).unwrap();
        let def_board = Board::default();

        assert_eq!(
            format!("{}", pen_board),
            format!("{}", def_board),
            "\n{}\n",
            pen_board
        );
    }
}
