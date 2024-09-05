#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CellState {
    Alive(AliveContext),
    Dead(DeadContext),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum AliveContext {
    Birth,   // 誕生
    Survive, // 生存
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum DeadContext {
    Overpopulated,  // 過密
    Underpopulated, // 過疎
    CannotBirth,    // 誕生できる状態ではない
}

impl CellState {
    /// ライフゲームにおける次のセルの状態を決定する関数
    ///
    /// 生存: 生きているセルに隣接する生きたセルが2つか3つならば、次の世代でも生存する。
    /// 過疎: 生きているセルに隣接する生きたセルが1つ以下ならば、過疎により死滅する。
    /// 過密: 生きているセルに隣接する生きたセルが4つ以上ならば、過密により死滅する。
    /// 誕生: 死んでいるセルに隣接する生きたセルがちょうど3つあれば、次の世代が誕生する。
    pub(crate) fn next(&self, living_neighbors: &usize) -> CellState {
        match self {
            CellState::Alive(_) => match living_neighbors {
                0 | 1 => CellState::Dead(DeadContext::Underpopulated),
                2 | 3 => CellState::Alive(AliveContext::Survive),
                4.. => CellState::Dead(DeadContext::Overpopulated),
            },
            CellState::Dead(_) => match living_neighbors {
                3 => CellState::Alive(AliveContext::Birth),
                _ => CellState::Dead(DeadContext::CannotBirth),
            },
        }
    }
}

pub fn living_cells(cells: &Vec<CellState>) -> usize {
    cells.iter().fold(0, |acc, cell: &CellState| match cell {
        CellState::Alive(_) => acc + 1,
        CellState::Dead(_) => acc,
    })
}
