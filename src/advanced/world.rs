use rand::random;
use std::cmp::{max, min};
use crate::advanced::cell::{living_cells, AliveContext, CellState, DeadContext};
use crate::advanced::field::Field;
use crate::shared::matrix::Matrix;

pub struct World {
    pub(crate) gen: usize,
    pub(crate) width: usize,
    height: usize,
    pub(crate) fields: Vec<Field>,
}

impl World {
    pub(crate) fn new(width: usize, height: usize, density: f64) -> Self {
        let fields = (0..width * height)
            .map(|_| {
                if random::<f64>() <= density {
                    CellState::Alive(AliveContext::Birth)
                } else {
                    CellState::Dead(DeadContext::CannotBirth)
                }
            })
            .map(|cell| Field::new(cell))
            .collect();
        Self {
            gen: 1,
            width,
            height,
            fields,
        }
    }

    pub(crate) fn update(&self) -> Self {
        let cell_matrix = Matrix::from_vec(
            &self.fields.iter().map(|field| field.cell_state).collect(),
            self.width,
        );

        let updated = self
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| Field {
                cell_state: field
                    .cell_state
                    .next(&living_cells(&cell_matrix.neighbors(index))),
                resource_level: match field.cell_state {
                    CellState::Alive(_) => max(field.resource_level - 3, -10),
                    CellState::Dead(_) => min(field.resource_level + 1, 10),
                },
            })
            .map(Self::apply_resource_effect)
            .collect();

        Self {
            gen: self.gen + 1,
            width: self.width,
            height: self.height,
            fields: updated,
        }
    }

    fn apply_resource_effect(field: Field) -> Field {
        match (field.resource_level, field.cell_state) {
            // 資源が不足しているならSurviveできない可能性がある
            (..-1, CellState::Alive(_)) => {
                if random::<f64>() < field.resource_level.abs() as f64 / 10.0 {
                    Field {
                        cell_state: CellState::Dead(DeadContext::Overpopulated), // コンテキストは仮置き
                        resource_level: field.resource_level,
                    }
                } else {
                    field
                }
            }
            // 資源が豊富ならDeadを回避する可能性がある
            (1.., CellState::Dead(context)) => match (
                context,
                random::<f64>() < field.resource_level.abs() as f64 / 10.0,
            ) {
                (DeadContext::Underpopulated | DeadContext::Overpopulated, true) => Field {
                    cell_state: CellState::Alive(AliveContext::Survive),
                    resource_level: field.resource_level,
                },
                _ => field,
            },
            _ => field,
        }
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Generation: {}\n", self.gen)?;
        for line in self.fields.as_slice().chunks(self.width) {
            for field in line {
                let symbol = match field.cell_state {
                    CellState::Alive(context) => match context {
                        AliveContext::Birth => '〇',
                        AliveContext::Survive => '〇',
                    },
                    CellState::Dead(_) => match field.resource_level.abs() {
                        _ => '・'
                    },
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}