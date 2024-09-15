use crate::advanced::cell::CellState;
use std::cmp::{max, min};

pub struct Field {
    pub(crate) cell_state: CellState,
    pub(crate) resource: Resource,
}

impl Field {
    pub fn new(cell_state: CellState) -> Self {
        Field {
            cell_state,
            resource: Resource::new(),
        }
    }
}

// TODO: 内部パラメータのvalueが露出しているのでいい感じに隠蔽する
pub struct Resource {
    pub value: isize,
}

impl Resource {
    const MAX: isize = 10;
    const MIN: isize = -10;
    const DEFAULT: isize = 0;

    fn new() -> Self {
        Resource {
            value: Self::DEFAULT,
        }
    }

    pub fn recover(&self) -> Self {
        Resource {
            value: min(self.value + 1, Self::MAX),
        }
    }

    pub fn consume(&self) -> Self {
        Resource {
            value: max(self.value - 3, Self::MIN),
        }
    }

    pub fn percentage(&self) -> f64 {
        self.value.abs() as f64 / 10.0
    }
}
