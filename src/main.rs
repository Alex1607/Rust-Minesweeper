use rand::Rng;

use crate::board::Board;

mod board;
mod field;

const MINE_COUNT: usize = 32;

fn main() {
    let mut board = Board::new(MINE_COUNT, 16, 16);

    Board::generate_board(&mut board);

    board.open_field(1, 1);

    board.print();
}