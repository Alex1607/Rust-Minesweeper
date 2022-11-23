use crate::field::{Field, FieldState};
use crate::generator::Generator;

#[derive(Clone, Debug)]
pub struct Board {
    pub(crate) fields: Vec<Vec<Field>>,
    pub(crate) mine_count: usize,
    pub(crate) x_size: i32,
    pub(crate) z_size: i32,
    pub(crate) game_state: GameState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameState {
    PREGAME,
    PLAYING,
    GAMEOVER_SOLVED,
    GAMEOVER_FAILED,
}

impl Board {
    pub(crate) fn new(mine_count: usize, x_size: i32, z_size: i32) -> Self {
        Board {
            fields: vec![vec![Field::new(); z_size as usize]; x_size as usize],
            mine_count,
            x_size,
            z_size,
            game_state: GameState::PREGAME,
        }
    }

    pub(crate) fn open_field(&mut self, x: usize, z: usize) {
        let field = &mut self.fields[x][z];

        if self.game_state == GameState::PREGAME {
            self.game_state = GameState::PLAYING;
        }

        //If flagged or already open return
        if field.field_state != FieldState::CLOSED {
            return;
        }

        if field.mine {
            self.game_state = GameState::GAMEOVER_FAILED;
            return;
        }

        field.field_state = FieldState::OPEN;

        if field.value == 0 {
            for xd in -1..=1_i32 {
                for zd in -1..=1_i32 {
                    let xx = xd + x as i32;
                    let zz = zd + z as i32;
                    if self.is_out_of_bounds(xx, zz) || xd == 0 && zd == 0 {
                        continue;
                    }
                    self.open_field(xx as usize, zz as usize)
                }
            }
        }
    }

    pub(crate) fn generate_board<T: Generator>(
        &mut self,
        generator: T,
        start_x: usize,
        start_z: usize,
    ) {
        generator.generate_field(self, start_x, start_z);
    }

    pub(crate) fn print(&self) {
        for x in &self.fields {
            for field in x {
                print!("{}", Self::get_field_text(field));
            }
            println!()
        }
        println!("{:?}", self.game_state)
    }

    pub(crate) fn is_out_of_bounds(&self, x: i32, z: i32) -> bool {
        x < 0 || x >= self.x_size || z < 0 || z >= self.z_size
    }

    fn get_field_text(field: &Field) -> String {
        match field.field_state {
            FieldState::OPEN => field.value.to_string(),
            FieldState::CLOSED => "_".to_string(),
            FieldState::FLAGGED => "¶".to_string(),
        }
    }
}
