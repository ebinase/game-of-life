use crate::advanced::cell::{AliveContext, CellState, DeadContext};
use crate::advanced::field::Field;
use crate::shared::matrix::Matrix;
use crate::shared::world::World;
use console::style;
use rand::random;

pub struct AdvancedWorld {
    pub(crate) gen: usize,
    pub(crate) width: usize,
    height: usize,
    pub(crate) fields: Vec<Field>,
}

impl World for AdvancedWorld {
    fn new(width: usize, height: usize, density: f64) -> Self {
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

    fn update(&self) -> Self {
        let cell_matrix = Matrix::from_vec(
            &self.fields.iter().map(|field| field.cell_state).collect(),
            self.width,
        );

        let updated = self
            .fields
            .iter()
            .enumerate()
            .map(|(index, field)| Field {
                cell_state: field.cell_state.next(&cell_matrix.neighbors(index)),
                resource: match field.cell_state {
                    CellState::Alive(_) => field.resource.consume(),
                    CellState::Dead(_) => field.resource.recover(),
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
}

impl AdvancedWorld {
    fn apply_resource_effect(field: Field) -> Field {
        match (field.resource.value, field.cell_state) {
            // 資源が不足しているならSurviveできない可能性がある
            (..-1, CellState::Alive(_)) => {
                if random::<f64>() < field.resource.percentage() {
                    Field {
                        cell_state: CellState::Dead(DeadContext::Starvation), // 資源不足でDeadへ
                        resource: field.resource,
                    }
                } else {
                    field
                }
            }
            // 資源が豊富ならDeadを回避する可能性がある
            (1.., CellState::Dead(context)) => {
                match (context, random::<f64>() < field.resource.percentage()) {
                    (DeadContext::Underpopulated, true) => Field {
                        cell_state: CellState::Alive(AliveContext::Survive), // Deadを回避
                        resource: field.resource,
                    },
                    _ => field,
                }
            }
            _ => field,
        }
    }
}

impl std::fmt::Display for AdvancedWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Generation: {}\n", self.gen)?;
        for line in self.fields.as_slice().chunks(self.width) {
            for field in line {
                let symbol = match field.cell_state {
                    CellState::Alive(_) => style('〇'),
                    CellState::Dead(_) => match field.resource.value {
                        ..-1 => style('　'),
                        _ => style('・').green().bold(),
                    },
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
