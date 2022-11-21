use crate::board::{Board, GameState};
use crate::field::{Field, FieldState};
use crate::solver::Solver;

mod board;
mod field;
mod solver;

const MINE_COUNT: usize = 64;

fn main() {
    let mut board = Board::new(MINE_COUNT, 16, 32);

    // Board::generate_board(&mut board);

    let input = "0,0,0,0,0,0,0,1,9,1,0,0,0,0,0,0,2,9,9,1,0,0,1,9,1,1,1,1,0,1,1,1;0,0,1,1,1,0,0,1,1,1,0,0,0,1,1,1,2,9,3,1,0,0,1,2,2,2,9,1,0,1,9,1;0,1,2,9,2,1,1,0,0,0,1,1,1,2,9,2,1,1,1,0,0,0,1,3,9,3,1,1,0,2,2,2;0,1,9,2,2,9,2,1,0,0,1,9,1,2,9,2,0,0,0,0,0,1,2,9,9,2,0,0,0,2,9,3;0,2,2,2,1,2,9,2,1,1,1,1,1,2,2,2,0,0,0,0,0,1,9,3,2,1,1,2,2,3,9,9;1,2,9,1,0,1,1,3,9,2,0,0,0,1,9,1,0,0,0,0,1,2,2,1,0,0,1,9,9,3,2,2;2,9,3,2,1,2,2,3,9,2,0,0,0,1,1,1,0,0,0,0,1,9,1,0,0,1,2,4,9,2,0,0;9,3,9,2,3,9,9,4,3,2,0,0,0,1,1,1,0,0,0,0,1,1,1,0,0,1,9,2,1,1,0,0;2,3,3,9,3,9,5,9,9,1,0,0,0,1,9,1,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0;1,9,3,2,2,1,3,9,3,1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1;1,3,9,2,0,0,1,1,1,0,0,0,0,0,1,2,2,1,1,2,2,1,0,1,1,1,0,1,1,1,1,9;0,2,9,2,0,0,0,0,0,0,1,2,2,1,1,9,9,2,1,9,9,1,0,1,9,1,0,1,9,1,1,1;0,1,1,1,0,0,0,0,0,0,1,9,9,1,1,4,9,3,1,2,2,2,1,2,2,2,1,1,1,1,0,0;0,0,0,0,0,0,1,2,2,1,1,2,2,1,0,2,9,3,1,0,0,1,9,2,2,9,1,0,1,1,1,0;0,0,0,0,0,0,1,9,9,1,0,0,0,0,0,1,2,9,1,1,1,2,2,9,2,1,1,0,1,9,2,1;0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,1,1,1,1,9,1,1,1,1,0,0,0,1,2,9,1";

    let split: Vec<&str> = input.split(';').collect();
    for x in 0..split.len() {
        let xsplit: Vec<&str> = split[x].split(',').collect();
        for z in 0..xsplit.len() {
            board.fields[x][z] = Field {
                value: xsplit[z].parse().unwrap(),
                field_state: FieldState::CLOSED,
                mine: xsplit[z].eq("9"),
                x,
                z,
            }
        }
    }

    board.open_field(1, 1); //Open one field to test if the recursion is working

    for x in board.fields.iter() {
        for z in x {
            print!("{},", z.value)
        }
        print!(";")
    }
    println!();

    let mut solver = Solver::new(&mut board);

    //Since board is borrowed at this point inside Solver I have to use solver.board instead of just board
    while solver.board.game_state == GameState::PLAYING {
        solver.solve_next_step();
        solver.board.print();
    }
}
