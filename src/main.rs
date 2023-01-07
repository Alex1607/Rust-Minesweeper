use crate::board::Board;
use crate::generator::NoGuessing;

mod board;
mod field;
mod generator;
mod solver;

const MINE_COUNT: usize = 1666;

fn main() {
    let mut board = Board::new(MINE_COUNT, 100, 100);

    Board::generate_board(&mut board, NoGuessing {}, 1, 1);
}
