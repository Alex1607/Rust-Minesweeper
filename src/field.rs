#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub value: u8,
    pub field_state: FieldState,
    pub mine: bool,
    pub x: usize,
    pub z: usize,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FieldState {
    OPEN,
    CLOSED,
    FLAGGED,
}

impl Field {
    pub(crate) fn new() -> Self {
        Field {
            value: 0,
            field_state: FieldState::CLOSED,
            mine: false,
            x: 0,
            z: 0,
        }
    }
}
