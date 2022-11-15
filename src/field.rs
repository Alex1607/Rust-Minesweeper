#[derive(Clone)]
pub struct Field {
    pub value: u8,
    pub field_state: FieldState,
    pub mine: bool,
}

#[derive(Clone, PartialEq)]
pub enum FieldState {
    OPEN,
    CLOSED,
    MARKED,
}

impl Field {
    pub(crate) fn new() -> Self {
        Field {
            value: 0,
            field_state: FieldState::CLOSED,
            mine: false
        }
    }
}
