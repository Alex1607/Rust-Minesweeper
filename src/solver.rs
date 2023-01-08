use std::collections::LinkedList;

use crate::board::GameState;
use crate::field::Field;
use crate::{board::Board, field::FieldState};

pub(crate) struct Solver<'a> {
    pub board: &'a mut Board,
    tank_board: Vec<Vec<Field>>,
    made_changes: bool,
    tried_tank: bool,
    reruns: usize,
    border_optimization: bool,
    known_mines: Vec<Vec<bool>>,
    known_empty: Vec<Vec<bool>>,
    tank_solutions: Vec<Vec<bool>>,
}

#[derive(PartialEq)]
enum InteractAction {
    Open,
    Flag,
}

impl Solver<'_> {
    pub(crate) fn new(board: &mut Board) -> Solver {
        Solver {
            board,
            made_changes: false,
            tried_tank: false,
            border_optimization: false,
            tank_board: Vec::new(),
            reruns: 0,
            known_mines: Vec::new(),
            known_empty: Vec::new(),
            tank_solutions: Vec::new(),
        }
    }

    pub(crate) fn solve_next_step(&mut self) {
        if self.made_changes {
            self.reruns = 0;
            self.tried_tank = false;
        } else if self.reruns < 3 {
            self.reruns += 1;
        } else if self.is_solved() {
            self.board.game_state = GameState::GameoverSolved;
            println!("\n\nGame was successfully solved\n");
            return;
        } else if !self.tried_tank {
            self.tank_solver();
            self.tried_tank = true;
        } else {
            self.board.game_state = GameState::GameoverFailed;
            println!("Board is not possible to solve without guessing.");
            return;
        }

        self.made_changes = false;

        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if Solver::get_field_value(&self.board.fields, x, z) > 0 {
                    self.solve_single(x, z);
                }
            }
        }
    }

    /**
     * Tank solver
     * By LuckyToilet: https://luckytoilet.wordpress.com/2012/12/23/2125/
     */
    fn tank_solver(&mut self) {
        let mut border_blocks: Vec<(usize, usize)> = Vec::new();
        let mut all_empty_blocks: Vec<(usize, usize)> = Vec::new();
        self.border_optimization = self.board.z_size * self.board.x_size > 2048;

        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if self.board.fields[x as usize][z as usize].field_state == FieldState::Closed {
                    all_empty_blocks.push((x as usize, z as usize));
                }
                if self.is_boundary(x, z)
                    && self.board.fields[x as usize][z as usize].field_state != FieldState::Flagged
                {
                    border_blocks.push((x as usize, z as usize));
                }
            }
        }

        let count_blocks_out_of_range = all_empty_blocks.len() - border_blocks.len();
        if count_blocks_out_of_range > 8 {
            self.border_optimization = true;
        } else {
            border_blocks = all_empty_blocks;
        }

        if border_blocks.is_empty() {
            return;
        }

        let mut segregated: Vec<Vec<(usize, usize)>> = Vec::new();
        if self.border_optimization {
            segregated = self.tank_segregate(&border_blocks);
        } else {
            segregated.push(border_blocks);
        }

        for f in 0..segregated.len() {
            self.tank_solutions = Vec::new();
            self.tank_board = self.board.fields.clone();

            self.known_mines =
                vec![vec![false; self.board.z_size as usize]; self.board.x_size as usize];
            self.known_empty =
                vec![vec![false; self.board.z_size as usize]; self.board.x_size as usize];
            for x in 0..self.board.x_size {
                for z in 0..self.board.z_size {
                    self.known_mines[x as usize][z as usize] =
                        self.board.fields[x as usize][z as usize].field_state
                            == FieldState::Flagged;
                    self.known_empty[x as usize][z as usize] =
                        Solver::get_field_value(&self.board.fields, x, z) >= 0;
                }
            }

            self.tank_recurse(segregated.get(f).unwrap(), 0);

            if self.tank_solutions.is_empty() {
                return;
            }

            for i in 0..segregated.get(f).unwrap().len() {
                let mut all_mine = true;
                let mut all_empty = true;
                for sln in &self.tank_solutions {
                    if !sln[i] {
                        all_mine = false;
                    }
                    if sln[i] {
                        all_empty = false;
                    }
                }

                let field = segregated[f][i];

                if all_mine {
                    self.board.fields[field.0][field.1].field_state = FieldState::Flagged;
                    self.made_changes = true;
                } else if all_empty {
                    self.board.open_field(field.0, field.1);
                }
            }
        }
    }

    fn tank_recurse(&mut self, border_tiles: &Vec<(usize, usize)>, depth: usize) {
        if border_tiles.len() > 25 {
            println!("Stopping early, too many borders");
            return;
        }
        let mut flag_count = 0;
        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if self.known_mines[x as usize][z as usize] {
                    flag_count += 1;
                }

                let current_value = Solver::get_field_value(&self.tank_board, x, z);
                if current_value < 0 {
                    continue;
                }

                if self.count_in_field(&self.known_mines, x as usize, z as usize) > current_value {
                    return;
                }

                let max_x = self.board.x_size;
                let max_z = self.board.z_size;
                let bordering = if (x == 0 && z == 0) || (x == max_x - 1 && z == max_z - 1) {
                    3
                } else if x == 0 || z == 0 || x == max_x - 1 || z == max_z - 1 {
                    5
                } else {
                    8
                };

                if bordering - self.count_in_field(&self.known_empty, x as usize, z as usize)
                    < current_value
                {
                    return;
                }
            }
        }

        if flag_count > self.board.mine_count {
            return;
        }

        if depth == border_tiles.len() {
            if !self.border_optimization && flag_count < self.board.mine_count {
                return;
            }

            let mut solution: Vec<bool> = vec![false; border_tiles.len()];
            for x in 0..border_tiles.len() {
                solution[x] = self.known_mines[border_tiles[x].0][border_tiles[x].1];
            }
            self.tank_solutions.push(solution);
            return;
        }

        let field = border_tiles[depth];

        self.known_mines[field.0][field.1] = true;
        self.tank_recurse(border_tiles, depth + 1);
        self.known_mines[field.0][field.1] = false;

        self.known_empty[field.0][field.1] = true;
        self.tank_recurse(border_tiles, depth + 1);
        self.known_empty[field.0][field.1] = false;
    }

    fn tank_segregate(&self, border_blocks: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
        let mut all_regions: Vec<Vec<(usize, usize)>> = Vec::new();
        let mut covered: Vec<(usize, usize)> = Vec::new();

        loop {
            let mut queue: LinkedList<(usize, usize)> = LinkedList::new();
            let mut finished_region: Vec<(usize, usize)> = Vec::new();

            for x in border_blocks {
                if !covered.contains(x) {
                    queue.push_back(*x);
                    break;
                }
            }

            if queue.is_empty() {
                break;
            }

            while !queue.is_empty() {
                let field = queue.pop_front().unwrap();
                finished_region.push(field);
                covered.push(field);

                for compare_field in border_blocks {
                    let mut connected = false;

                    if finished_region.contains(compare_field) {
                        continue;
                    }

                    let field_x = field.0 as i32;
                    let field_z = field.1 as i32;
                    let compare_x = compare_field.0 as i32;
                    let compare_z = compare_field.1 as i32;

                    if (field_x - compare_x).abs() <= 2 && (field_z - compare_z).abs() <= 2 {
                        'search: for x in 0..self.board.x_size {
                            for z in 0..self.board.z_size {
                                if Solver::get_field_value(&self.board.fields, x, z) > 0
                                    && (field_x - x).abs() <= 1
                                    && (field_z - z) <= 1
                                    && (compare_x - x).abs() <= 1
                                    && (compare_z - z).abs() <= 1
                                {
                                    connected = true;
                                    break 'search;
                                }
                            }
                        }
                    }

                    if !connected {
                        continue;
                    }
                    if !queue.contains(compare_field) {
                        queue.push_back(*compare_field)
                    }
                }
            }
            all_regions.push(finished_region);
        }

        all_regions
    }

    fn solve_single(&mut self, x: i32, z: i32) {
        let closed = self.count_surrounding_by_type(x, z, FieldState::Closed);
        if closed == 0 {
            return;
        }

        let mut already_flagged = self.count_surrounding_by_type(x, z, FieldState::Flagged);
        let field_value = Solver::get_field_value(&self.board.fields, x, z);

        if field_value == already_flagged + closed {
            self.interact_surrounding_fields(x, z, InteractAction::Flag);
            already_flagged = self.count_surrounding_by_type(x, z, FieldState::Flagged);
        }

        if field_value == already_flagged {
            self.interact_surrounding_fields(x, z, InteractAction::Open);
        }
    }

    fn count_in_field(&self, board: &[Vec<bool>], x: usize, z: usize) -> i32 {
        let mut hits: i32 = 0;
        if z > 0 {
            if x > 0 && board[x - 1][z - 1] {
                hits += 1;
            }
            if board[x][z - 1] {
                hits += 1;
            }
            if x < board.len() - 1 && board[x + 1][z - 1] {
                hits += 1;
            }
        }

        if x > 0 && board[x - 1][z] {
            hits += 1;
        }
        if x < board.len() - 1 && board[x + 1][z] {
            hits += 1;
        }

        if z < board[0].len() - 1 {
            if x > 0 && board[x - 1][z + 1] {
                hits += 1;
            }
            if board[x][z + 1] {
                hits += 1;
            }
            if x < board.len() - 1 && board[x + 1][z + 1] {
                hits += 1;
            }
        }
        hits
    }

    fn count_surrounding_by_type(&self, x: i32, z: i32, search_for: FieldState) -> i32 {
        let mut hits: i32 = 0;
        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if self.is_out_of_bounds(xx, zz) {
                    continue;
                }

                if self.board.fields[xx as usize][zz as usize].field_state == search_for {
                    hits += 1;
                }
            }
        }
        hits
    }

    fn is_boundary(&self, x: i32, z: i32) -> bool {
        if self.board.fields[x as usize][z as usize].field_state != FieldState::Closed {
            return false;
        }

        if z > 0 {
            if x > 0 && Solver::get_field_value(&self.board.fields, x - 1, z - 1) >= 0 {
                return true;
            }
            if Solver::get_field_value(&self.board.fields, x, z - 1) >= 0 {
                return true;
            }
            if x < self.board.x_size - 1
                && Solver::get_field_value(&self.board.fields, x + 1, z - 1) >= 0
            {
                return true;
            }
        }

        if x > 0 && Solver::get_field_value(&self.board.fields, x - 1, z) >= 0 {
            return true;
        }
        if x < self.board.x_size - 1 && Solver::get_field_value(&self.board.fields, x + 1, z) >= 0 {
            return true;
        }

        if z < self.board.z_size - 1 {
            if x > 0 && Solver::get_field_value(&self.board.fields, x - 1, z + 1) >= 0 {
                return true;
            }
            if Solver::get_field_value(&self.board.fields, x, z + 1) >= 0 {
                return true;
            }
            if x < self.board.x_size - 1
                && Solver::get_field_value(&self.board.fields, x + 1, z + 1) >= 0
            {
                return true;
            }
        }

        false
    }

    pub(crate) fn is_solved(&self) -> bool {
        let mut flagged_fields: usize = 0;
        for x in &self.board.fields {
            for field in x {
                if field.field_state == FieldState::Flagged {
                    flagged_fields += 1;
                } else if field.field_state == FieldState::Closed {
                    return false;
                }
            }
        }
        flagged_fields == self.board.mine_count
    }

    fn interact_surrounding_fields(&mut self, x: i32, z: i32, action: InteractAction) {
        for xd in -1..=1 {
            for zd in -1..=1 {
                if xd == 0 && zd == 0 {
                    continue;
                }
                let xx = x + xd;
                let zz = z + zd;
                if self.is_out_of_bounds(xx, zz) {
                    continue;
                }

                let temp_field = &mut self.board.fields[xx as usize][zz as usize];

                if action == InteractAction::Open && temp_field.field_state == FieldState::Closed {
                    self.board.open_field(xx as usize, zz as usize);
                    self.made_changes = true;
                } else if action == InteractAction::Flag
                    && temp_field.field_state == FieldState::Closed
                {
                    temp_field.field_state = FieldState::Flagged;
                    self.made_changes = true;
                }
            }
        }
    }

    fn is_out_of_bounds(&self, x: i32, z: i32) -> bool {
        x < 0 || x >= self.board.x_size || z < 0 || z >= self.board.z_size
    }

    fn get_field_value(fields: &[Vec<Field>], x: i32, z: i32) -> i32 {
        let field = &fields[x as usize][z as usize];
        match field.field_state {
            FieldState::Open => field.value as i32,
            FieldState::Closed => -2,
            FieldState::Flagged => -1,
        }
    }
}
