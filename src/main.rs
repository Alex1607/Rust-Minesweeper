use crate::board::{Board, GameState};
use crate::solver::Solver;

mod board;
mod field;
mod solver;

const MINE_COUNT: usize = 1666;

fn main() {
    let mut board = Board::new(MINE_COUNT, 100, 100);

    Board::generate_board(&mut board);

    board.open_field(1, 1); //Open one field to test if the recursion is working

    let mut solver = Solver::new(&mut board);

    //Since board is borrowed at this point inside Solver I have to use solver.board instead of just board
    while solver.board.game_state == GameState::PLAYING {
        solver.solve_next_step();
    }

    solver.board.print();
}
