use crate::field::Field;

pub struct Board {
    pub(crate) fields: Vec<Vec<Field>>,
    pub(crate) mine_count: usize,
    pub(crate) x_size: i32,
    pub(crate) y_size: i32,
}

impl Board {
    pub(crate) fn new(mine_count: usize, x_size: i32, y_size: i32) -> Self {
        Board {
            fields: vec![vec![Field::new(); y_size as usize]; x_size as usize],
            mine_count,
            x_size,
            y_size
        }
    }
}