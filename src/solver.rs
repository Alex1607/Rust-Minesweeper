use crate::board::Board;
use crate::field::{Field, FieldState};

pub(crate) struct Solver<'a> {
    board: &'a mut Board,
    tank_board: Option<Board>,
    made_changes: bool,
    border_optimization: bool,
    known_mines: Vec<Vec<bool>>,
    known_empty: Vec<Vec<bool>>,
}

#[derive(PartialEq)]
enum InteractAction {
    OPEN,
    FLAG,
}

impl Solver<'_> {
    pub(crate) fn new(board: &mut Board) -> Solver {
        Solver {
            board,
            made_changes: false,
            border_optimization: false,
            tank_board: None,
            known_mines: Vec::new(),
            known_empty: Vec::new(),
        }
    }

    pub(crate) fn solve_next_step(&mut self) {
        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if Self::get_field_value(self.board, x, z) > 0 {
                    Solver::solve_single(self.board, x, z);
                }
            }
        }
    }

    /**
     * Tank solver
     * By LuckyToilet: https://luckytoilet.wordpress.com/2012/12/23/2125/
     */
    fn tank_solver(solver: &mut Solver, board: &mut Board) {
        let mut border_blocks: Vec<&Field> = Vec::new();
        let mut all_empty_blocks: Vec<&Field> = Vec::new();
        solver.border_optimization = false;

        for x in 0..board.x_size {
            for z in 0..board.z_size {
                if board.fields[x as usize][z as usize].field_state == FieldState::CLOSED {
                    all_empty_blocks.push(&board.fields[x as usize][z as usize]);
                }
                if Solver::is_boundary(board, x, z)
                    && board.fields[x as usize][z as usize].field_state != FieldState::FLAGGED
                {
                    border_blocks.push(&board.fields[x as usize][z as usize]);
                }
            }
        }

        let count_blocks_out_of_range = all_empty_blocks.len() - border_blocks.len();
        if count_blocks_out_of_range > 8 {
            solver.border_optimization = true;
        } else {
            border_blocks = all_empty_blocks;
        }

        if border_blocks.is_empty() {
            println!("An error occured");
            return;
        }

        let mut segregated: Vec<Vec<&Field>>;
        if solver.border_optimization {
            segregated = Solver::tank_segregate();
        } else {
            segregated = Vec::new();
            segregated.push(border_blocks);
        }

        for fields in &segregated {
            let tank_solution: Vec<Vec<bool>> = Vec::new();
            solver.tank_board = Some(solver.board.clone());

            solver.known_mines = vec![vec![false; board.z_size as usize]; board.x_size as usize];
            solver.known_empty = vec![vec![false; board.z_size as usize]; board.x_size as usize];
            for x in 0..board.x_size {
                for z in 0..board.z_size {
                    solver.known_mines[x as usize][z as usize] =
                        board.fields[x as usize][z as usize].field_state == FieldState::FLAGGED;
                    solver.known_empty[x as usize][z as usize] =
                        Solver::get_field_value(board, x as i32, z as i32) >= 0;
                }
            }

            Solver::tank_recurse(&fields, 0);

            if tank_solution.is_empty() {
                println!("An error occured");
                return;
            }

            for (i, _field) in fields.into_iter().enumerate() {
                let mut all_mine = true;
                let mut all_empty = true;
                for sln in &tank_solution {
                    if !sln[i] {
                        all_mine = false;
                    }
                    if sln[i] {
                        all_empty = false;
                    }
                }

                let _field = fields[i];

                if all_mine {
                    // board.fields[field.x][field.z].field_state == FieldState::FLAGGED; //TODO: Define X and Z
                    solver.made_changes = true;
                } else if all_empty {
                    // board.open_field(field.x, field.z);
                }
            }
        }
    }

    fn tank_recurse(_fields: &Vec<&Field>, _i: i32) {}

    fn tank_segregate() -> Vec<Vec<&'static Field>> {
        return Vec::new();
    }

    fn solve_single(board: &mut Board, x: i32, z: i32) {
        let closed = Solver::count_surrounding_by_type(board, x, z, FieldState::CLOSED) as i32;
        if closed == 0 {
            return;
        }

        let mut already_flagged =
            Solver::count_surrounding_by_type(board, x, z, FieldState::FLAGGED) as i32;
        let field_value = Solver::get_field_value(board, x, z);

        if field_value == already_flagged + closed {
            Solver::interact_surrounding_fields(board, x, z, InteractAction::FLAG);
            already_flagged =
                Solver::count_surrounding_by_type(board, x, z, FieldState::FLAGGED) as i32;
        }

        if field_value == already_flagged {
            Solver::interact_surrounding_fields(board, x, z, InteractAction::OPEN);
        }
    }

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
        if board.fields[x as usize][z as usize].field_state != FieldState::CLOSED {
            return false;
        }

        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if Solver::is_out_of_bounds(board, xx, zz) {
                    continue;
                }

                return Solver::get_field_value(board, xx, zz) >= 0;
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

    //TODO: I could probably just use the count_surrounding_by_type methode instead of this one
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
                if xd == 0 && zd == 0 {
                    continue;
                }
                let xx = x + xd;
                let zz = z + zd;
                if Solver::is_out_of_bounds(board, xx, zz) {
                    continue;
                }

                let temp_field = &mut board.fields[xx as usize][zz as usize];

                if action == InteractAction::OPEN && temp_field.field_state == FieldState::CLOSED {
                    board.open_field(xx as usize, zz as usize);
                } else if action == InteractAction::FLAG && temp_field.field_state == FieldState::CLOSED {
                    temp_field.field_state = FieldState::FLAGGED;
                }
            }
        }
    }

    fn is_out_of_bounds(board: &Board, x: i32, z: i32) -> bool {
        x < 0 || x >= board.x_size || z < 0 || z >= board.z_size
    }

    fn get_field_value(board: &Board, x: i32, z: i32) -> i32 {
        let field = &board.fields[x as usize][z as usize];
        match field.field_state {
            FieldState::OPEN => field.value as i32,
            FieldState::CLOSED => -2,
            FieldState::FLAGGED => -1,
        }
    }
}
