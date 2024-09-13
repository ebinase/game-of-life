use crate::advanced::cell::CellState;

pub struct Field {
    pub(crate) cell_state: CellState,
    pub(crate) resource_level: isize,
}

impl Field {
    pub fn new(cell_state: CellState) -> Self {
        Field {
            cell_state,
            resource_level: 0,
        }
    }
}
