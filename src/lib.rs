#![feature(try_from)]

mod board;
mod piece;
mod player;
mod tile;

pub use board::Board;
pub use piece::Piece;
pub use player::Player;
pub use tile::Tile;

pub mod castling;

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn square_shit_is_correct() {
        let rank = 0b010;
        let file = 0b101;

        let square = Tile::new(file, rank);

        assert_eq!(square.as_u8(), 0b_0010_0101);
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
