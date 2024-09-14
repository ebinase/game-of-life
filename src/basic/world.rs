use crate::basic::cell::{living_cells, AliveContext, CellState, DeadContext};
use crate::shared::matrix::Matrix;
use crate::shared::world::World;
use rand::random;

pub struct BasicWorld {
    pub(crate) gen: usize,
    pub(crate) width: usize,
    height: usize,
    pub(crate) cells: Vec<CellState>,
}

impl World for BasicWorld {
    fn new(width: usize, height: usize, density: f64) -> Self {
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

    fn update(&self) -> Self {
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

impl std::fmt::Display for BasicWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Generation: {}\n", self.gen)?;
        for line in self.cells.as_slice().chunks(self.width) {
            for &cell in line {
                let symbol = match cell {
                    CellState::Alive(_) => '〇',
                    CellState::Dead(_) => '・',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
