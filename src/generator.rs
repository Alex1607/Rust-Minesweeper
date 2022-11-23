use std::time::Instant;

use rand::Rng;

use crate::board::{Board, GameState};
use crate::field::FieldState;
use crate::solver::Solver;

pub(crate) trait Generator {
    fn generate_field(&self, board: &mut Board, start_x: usize, start_z: usize);

    fn set_cords(board: &mut Board) {
        for x in 0..board.x_size as usize {
            for z in 0..board.z_size as usize {
                let field = &mut board.fields[x][z];
                field.x = x;
                field.z = z;
            }
        }
    }

    fn increment_values(board: &mut Board, x: i32, z: i32) {
        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if board.is_out_of_bounds(xx, zz) || (zd == 0 && xd == 0) {
                    continue;
                }

                let checked_field = &mut board.fields[xx as usize][zz as usize];
                if checked_field.mine {
                    continue;
                }

                checked_field.value += 1;
            }
        }
    }
}

pub(crate) struct Default;

impl Generator for Default {
    fn generate_field(&self, board: &mut Board, start_x: usize, start_z: usize) {
        let mut rng = rand::thread_rng();
        let mut placed_mines: usize = 0;

        while placed_mines < board.mine_count {
            let x = rng.gen_range(0..board.x_size);
            let z = rng.gen_range(0..board.z_size);

            let possible_mine = &mut board.fields[x as usize][z as usize];
            if possible_mine.mine {
                continue;
            }

            if start_x as i32 == x && start_z as i32 == z {
                continue;
            }

            possible_mine.mine = true;
            possible_mine.value = 9;
            placed_mines += 1;

            Self::increment_values(board, x, z);
        }

        Self::set_cords(board);
    }
}

pub(crate) struct Modified;

impl Generator for Modified {
    fn generate_field(&self, board: &mut Board, start_x: usize, start_z: usize) {
        let mut rng = rand::thread_rng();
        let mut placed_mines: usize = 0;

        'mineloop: while placed_mines < board.mine_count {
            let x = rng.gen_range(0..board.x_size);
            let z = rng.gen_range(0..board.z_size);

            let possible_mine = &mut board.fields[x as usize][z as usize];
            if possible_mine.mine {
                continue;
            }

            for xd in -1..=1_i32 {
                for zd in -1..=1_i32 {
                    if start_x as i32 + xd == x && start_z as i32 + zd == z {
                        continue 'mineloop;
                    }
                }
            }

            possible_mine.mine = true;
            possible_mine.value = 9;
            placed_mines += 1;

            Self::increment_values(board, x, z);
        }

        Self::set_cords(board);
    }
}

pub(crate) struct NoGuessing;

impl Generator for NoGuessing {
    fn generate_field(&self, board: &mut Board, start_x: usize, start_z: usize) {
        let mut found_solvable = false;
        let mut tries = 0;
        let start_time = Instant::now();
        while !found_solvable {
            tries += 1;

            Modified.generate_field(board, start_x, start_z);

            board.open_field(start_x, start_z);

            let mut solver = Solver::new(board);

            //Since board is borrowed at this point inside Solver I have to use solver.board instead of just board
            while solver.board.game_state == GameState::PLAYING {
                print!(".");
                solver.solve_next_step();
            }

            if board.game_state == GameState::GAMEOVER_SOLVED {
                found_solvable = true;
            } else {
                for x in 0..board.x_size as usize {
                    for z in 0..board.z_size as usize {
                        let field = &mut board.fields[x][z];
                        field.value = 0;
                        field.mine = false;
                        field.field_state = FieldState::CLOSED;
                    }
                }
                board.game_state = GameState::PREGAME;

                println!("Trying another field.");
            }
        }

        println!(
            "Found solution in {} ms while generating {} boards",
            start_time.elapsed().as_millis(),
            tries
        );

        for x in 0..board.x_size as usize {
            for z in 0..board.z_size as usize {
                let field = &mut board.fields[x][z];
                field.field_state = FieldState::CLOSED;
            }
        }
    }
}
