use std::any::type_name;

fn main() {
    println!("Hello, world!");

    #[derive(PartialEq, Copy, Clone)]
    enum CellState {
        Alive,
        Dead,
    }

    /// ライフゲームにおける次のセルの状態を決定する関数
    ///
    /// 生存: 生きているセルに隣接する生きたセルが2つか3つならば、次の世代でも生存する。
    /// 過疎: 生きているセルに隣接する生きたセルが1つ以下ならば、過疎により死滅する。
    /// 過密: 生きているセルに隣接する生きたセルが4つ以上ならば、過密により死滅する。
    /// 誕生: 死んでいるセルに隣接する生きたセルがちょうど3つあれば、次の世代が誕生する。
    fn next_state(current: &CellState, living_neighbors: &u32) -> CellState {
        match current {
            CellState::Alive => match living_neighbors {
                0 | 1 => CellState::Dead,   // 過疎
                2 | 3 => CellState::Alive,  // 生存
                4.. => CellState::Dead,     // 過密
            },
            CellState::Dead => match living_neighbors {
                3 => CellState::Alive,  // 誕生
                _ => CellState::Dead,   // 何も起こらない
            }
        }
    }

    fn living_cells(cells: &Vec<CellState>) -> u32 {
        cells.iter().fold(0, |acc, cell: CellState| {
            match cell {
                CellState::Alive => acc + 1,
                CellState::Dead => acc,
            }
        })
    }

    fn neighbors(world: &World, index: &u32) -> Vec<CellState> {

    }

    fn neighbor_indexes(width: u32, height: u32, index: u32) -> Vec<u32> {
        let position = (index%width, index/width);
        // indexが範囲外かチェック
        if index >= width * height {
            return vec![];
        }

        let mut indexes: Vec<u32> = vec![];
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                let x= position.0 as i32 + i;
                let y = position.1 as i32 + j;
                if !(i == 0 && j == 0) && x <= width as i32 && y <= height as i32 && x >= 0 && y >= 0 {
                    indexes.push(y as u32 * width + x as u32);
                } else {
                    continue;
                }
            }
        }
        indexes
    }

    struct World {
        width: u32,
        height: u32,
        cells: Vec<CellState>,
    }

    impl World {
        fn new(width: u32, height: u32) -> Self {
            let cells = (0..width * height)
                .map(|x| {
                    if x % 5 == 0 {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    }
                })
                .collect();
            Self {
                width,
                height,
                cells,
            }
        }
    }

    impl std::fmt::Display for World {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            for line in self.cells.as_slice().chunks(self.width as usize) {
                for &cell in line {
                    let symbol = if cell == CellState::Dead {
                        '・'
                    } else {
                        '〇'
                    };
                    write!(f, "{}", symbol)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    println!("{}", World::new(20, 10));
}
