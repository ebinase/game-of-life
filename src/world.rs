use crate::cell::{living_cells, AliveContext, CellState, DeadContext};
use crate::matrix::Matrix;
use rand::random;

pub struct World {
    pub(crate) gen: usize,
    pub(crate) width: usize,
    height: usize,
    pub(crate) cells: Vec<CellState>,
}

impl World {
    pub(crate) fn new(width: usize, height: usize, density: f64) -> Self {
        let cells = (0..width * height)
            .map(|_| {
                if random::<f64>() <= density {
                    CellState::Alive(AliveContext::Birth)
                } else {
                    CellState::Dead(DeadContext::CannotBirth)
                }
            })
            .collect();
        Self {
            gen: 1,
            width,
            height,
            cells,
        }
    }

    pub(crate) fn update(&self) -> Self {
        let matrix = Matrix::from_vec(&self.cells, self.width);

        let updated = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, cell)| cell.next(&living_cells(&matrix.neighbors(index))))
            .collect();

        Self {
            gen: self.gen + 1,
            width: self.width,
            height: self.height,
            cells: updated,
        }
    }
}
