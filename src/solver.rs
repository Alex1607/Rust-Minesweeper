use std::intrinsics::fabsf32;

use crate::board::Board;
use crate::field::FieldState;

struct Solver {
    board: Board,
    made_changes: bool,
}

enum InteractAction {
    OPEN,
    FLAG,
}

impl Solver {
    fn new(board: Board, made_changes: bool) -> Solver {
        Solver {
            board,
            made_changes,
        }
    }

    pub(crate) fn solve_next_step() {}

    fn count_surrounding_by_type(board: &Board, x: i32, z: i32, search_for: FieldState) -> usize {
        let mut hits: usize = 0;
        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if Solver::is_out_of_bounds(board, xx, zz) {
                    continue;
                }

                if board.fields[xx as usize][zz as usize].field_state == search_for {
                    hits += 1;
                }
            }
        }
        hits
    }

    fn is_boundary(board: &Board, x: i32, z: i32) -> bool {
        if board.fields[x as usize][z as usize].field_state != FieldState::CLOSED { false }

        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if Solver::is_out_of_bounds(board, xx, zz) {
                    continue;
                }

                return board.fields[xx as usize][zz as usize].field_state >= 0;
            }
        }

        false
    }

    fn is_solved(board: &Board) -> bool {
        let mut flagged_fields: usize = 0;
        for x in &board.fields {
            for field in x {
                if field.field_state == FieldState::FLAGGED {
                    flagged_fields += 1;
                } else if field.field_state == FieldState::CLOSED {
                    return false;
                }
            }
        }
        flagged_fields == board.mine_count
    }

    fn count_flags_around(board: &Board, x: i32, z: i32) -> usize {
        let mut mines: usize = 0;
        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if Solver::is_out_of_bounds(board, xx, zz) {
                    continue;
                }

                if board.fields[xx as usize][zz as usize].field_state == FieldState::FLAGGED {
                    mines += 1;
                }
            }
        }
        mines
    }

    fn interact_surrounding_fields(board: &mut Board, x: i32, z: i32, action: InteractAction) {
        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if Solver::is_out_of_bounds(board, xx, zz) {
                    continue;
                }

                if action == InteractAction::OPEN {
                    board.open_field(xx as usize, zz as usize);
                } else if action == InteractAction::FLAG {
                    board.fields[xx as usize][zz as usize].field_state = FieldState::FLAGGED;
                }
            }
        }
    }

    fn is_out_of_bounds(board: &Board, x: i32, z: i32) -> bool {
        x < 0 || x >= board.x_size || z < 0 || z >= board.z_size
    }

    fn get_field_value(board: &Board, x: usize, z: usize) -> i8 {
        let field = &board.fields[x][z];
        match field.field_state {
            FieldState::OPEN => field.value as i8,
            FieldState::CLOSED => -2,
            FieldState::FLAGGED => -1
        }
    }
}