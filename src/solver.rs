use std::collections::LinkedList;

use crate::board::GameState;
use crate::{
    board::Board,
    field::{Field, FieldState},
};

pub(crate) struct Solver<'a> {
    pub board: &'a mut Board,
    tank_board: Option<Board>,
    made_changes: bool,
    border_optimization: bool,
    known_mines: Vec<Vec<bool>>,
    known_empty: Vec<Vec<bool>>,
    tank_solutions: Vec<Vec<bool>>,
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
            tank_solutions: Vec::new(),
        }
    }

    pub(crate) fn solve_next_step(&mut self) {
        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if self.get_field_value(x, z) > 0 {
                    self.solve_single(x, z);
                }
            }
        }

        if !self.made_changes {
            if self.is_solved() {
                self.board.game_state = GameState::GAMEOVER;
                return;
            } else {
                self.tank_solver();
            }
        }
    }

    /**
     * Tank solver
     * By LuckyToilet: https://luckytoilet.wordpress.com/2012/12/23/2125/
     */
    fn tank_solver(&mut self) {
        let mut border_blocks: Vec<&Field> = Vec::new();
        let mut all_empty_blocks: Vec<&Field> = Vec::new();
        let mut fields_to_open: Vec<(usize, usize)> = Vec::new();
        let mut fields_to_flag: Vec<(usize, usize)> = Vec::new();
        self.border_optimization = false;

        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if self.board.fields[x as usize][z as usize].field_state == FieldState::CLOSED {
                    all_empty_blocks.push(&self.board.fields[x as usize][z as usize]);
                }
                if self.is_boundary(x, z)
                    && self.board.fields[x as usize][z as usize].field_state != FieldState::FLAGGED
                {
                    border_blocks.push(&self.board.fields[x as usize][z as usize]);
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
            println!("An error occurred");
            return;
        }

        let segregated: Vec<Vec<&Field>>;
        if self.border_optimization {
            segregated = self.tank_segregate(&border_blocks);
        } else {
            segregated = vec![border_blocks];
        }

        for fields in segregated.iter() {
            let tank_solution: Vec<Vec<bool>> = Vec::new();
            self.tank_board = Some(self.board.clone());

            self.known_mines =
                vec![vec![false; self.board.z_size as usize]; self.board.x_size as usize];
            self.known_empty =
                vec![vec![false; self.board.z_size as usize]; self.board.x_size as usize];
            for x in 0..self.board.x_size {
                for z in 0..self.board.z_size {
                    self.known_mines[x as usize][z as usize] =
                        self.board.fields[x as usize][z as usize].field_state
                            == FieldState::FLAGGED;
                    self.known_empty[x as usize][z as usize] =
                        self.get_field_value(x as i32, z as i32) >= 0;
                }
            }

            self.tank_recurse(fields, 0);

            if tank_solution.is_empty() {
                println!("An error occurred");
                return;
            }

            for (i, _field) in fields.iter().enumerate() {
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

                let field = fields[i];

                if all_mine {
                    fields_to_flag.push((field.x, field.z));
                    self.made_changes = true;
                } else if all_empty {
                    fields_to_open.push((field.x, field.z));
                }
            }
        }

        for x in fields_to_flag {
            self.board.fields[x.0][x.1].field_state = FieldState::FLAGGED;
        }

        for x in fields_to_open {
            self.board.open_field(x.0, x.1);
        }
    }

    fn tank_recurse(&mut self, border_tiles: &Vec<&Field>, depth: usize) {
        let mut flag_count = 0;
        for x in 0..self.board.x_size {
            for z in 0..self.board.z_size {
                if self.known_mines[x as usize][z as usize] {
                    flag_count += 1;
                }

                let current_value = self.get_field_value(x, z);
                if current_value < 0 {
                    continue;
                }

                if self.count_surrounding_by_type(x, z, FieldState::FLAGGED) > current_value {
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

                if bordering - self.count_surrounding_by_type(x, z, FieldState::OPEN)
                    < current_value
                {
                    return;
                }
            }

            if flag_count > self.board.mine_count {
                return;
            }

            if depth == border_tiles.len() {
                if !self.border_optimization && flag_count < self.board.mine_count {
                    return;
                }

                let mut solution: Vec<bool> = Vec::new();
                for x in border_tiles {
                    solution.push(self.known_mines[x.x][x.z]);
                }
                self.tank_solutions.push(solution);
                return;
            }

            let field = border_tiles[depth];
            self.known_mines[field.x][field.z] = true;
            self.tank_recurse(border_tiles, depth + 1);
            self.known_mines[field.x][field.z] = false;

            self.known_empty[field.x][field.z] = true;
            self.tank_recurse(border_tiles, depth + 1);
            self.known_empty[field.x][field.z] = false;
        }
    }

    fn tank_segregate<'a>(&mut self, border_blocks: &Vec<&'a Field>) -> Vec<Vec<&'a Field>> {
        let mut all_regions: Vec<Vec<&'a Field>> = Vec::new();
        let mut covered: Vec<&Field> = Vec::new();

        loop {
            let mut queue: LinkedList<&Field> = LinkedList::new();
            let mut finished_region: Vec<&Field> = Vec::new();

            for x in border_blocks {
                if !covered.contains(x) {
                    queue.push_back(x);
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

                    let field_x = field.x as i32;
                    let field_z = field.z as i32;
                    let compare_x = compare_field.x as i32;
                    let compare_z = compare_field.z as i32;

                    if (field_x - compare_x).abs() <= 2 && (field_z - compare_z).abs() <= 2 {
                        'search: for x in 0..self.board.x_size {
                            for z in 0..self.board.z_size {
                                if self.get_field_value(x, z) > 0
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
                        queue.push_back(compare_field)
                    }
                }
            }
            all_regions.push(finished_region);
        }

        all_regions
    }

    fn solve_single(&mut self, x: i32, z: i32) {
        let closed = self.count_surrounding_by_type(x, z, FieldState::CLOSED);
        if closed == 0 {
            return;
        }

        let mut already_flagged = self.count_surrounding_by_type(x, z, FieldState::FLAGGED);
        let field_value = self.get_field_value(x, z);

        if field_value == already_flagged + closed {
            self.interact_surrounding_fields(x, z, InteractAction::FLAG);
            already_flagged = self.count_surrounding_by_type(x, z, FieldState::FLAGGED);
        }

        if field_value == already_flagged {
            self.interact_surrounding_fields(x, z, InteractAction::OPEN);
        }
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
        if self.board.fields[x as usize][z as usize].field_state != FieldState::CLOSED {
            return false;
        }

        for xd in -1..=1 {
            for zd in -1..=1 {
                let xx = x + xd;
                let zz = z + zd;
                if self.is_out_of_bounds(xx, zz) {
                    continue;
                }

                return self.get_field_value(xx, zz) >= 0;
            }
        }

        false
    }

    pub(crate) fn is_solved(&self) -> bool {
        let mut flagged_fields: usize = 0;
        for x in &self.board.fields {
            for field in x {
                if field.field_state == FieldState::FLAGGED {
                    flagged_fields += 1;
                } else if field.field_state == FieldState::CLOSED {
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

                if action == InteractAction::OPEN && temp_field.field_state == FieldState::CLOSED {
                    self.board.open_field(xx as usize, zz as usize);
                } else if action == InteractAction::FLAG
                    && temp_field.field_state == FieldState::CLOSED
                {
                    temp_field.field_state = FieldState::FLAGGED;
                }
            }
        }
    }

    fn is_out_of_bounds(&self, x: i32, z: i32) -> bool {
        x < 0 || x >= self.board.x_size || z < 0 || z >= self.board.z_size
    }

    fn get_field_value(&self, x: i32, z: i32) -> i32 {
        let field = &self.board.fields[x as usize][z as usize];
        match field.field_state {
            FieldState::OPEN => field.value as i32,
            FieldState::CLOSED => -2,
            FieldState::FLAGGED => -1,
        }
    }
}
