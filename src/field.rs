#[derive(Clone, Debug)]
pub struct Field {
    pub value: u8,
    pub field_state: FieldState,
    pub mine: bool,
}

#[derive(Clone, PartialEq, Debug)]
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
        }
    }
}
