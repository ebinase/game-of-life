use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Welcome to Game Of Life!");

    #[derive(PartialEq, Copy, Clone, Debug)]
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
        cells.iter().fold(0, |acc, cell: &CellState| {
            match cell {
                CellState::Alive => acc + 1,
                CellState::Dead => acc,
            }
        })
    }

    #[derive(Debug)]
    struct Position {
        row: u32,
        col: u32
    }

    fn neighbors(matrix: &Matrix, index: &u32) -> Vec<CellState> {
        let width = matrix[0].len() as u32;
        let position = Position{row: index / width, col: index % width};

        let mut neighbors = vec![];
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if i == 0 && j == 0 {
                    continue
                }
                let row= position.row as i32 + i;
                let col = position.col as i32 + j;
                // 行と列が範囲内かを確認して、`Some`なセルだけをpush
                if let Some(cell_state) = matrix
                    .get(row as usize)
                    .and_then(|line: &Vec<CellState>| line.get(col as usize))
                {
                    neighbors.push(*cell_state); // 参照をデリファレンスして`CellState`の値を格納
                }
            }
        }

        neighbors
    }

    type Matrix = Vec<Vec<CellState>>;

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

        fn update(&self) -> Self {
            let matrix: Matrix = self.cells
                .chunks(self.width as usize)
                .map(|row| {row.to_vec()})
                .collect();

            let updated = self.cells
                .iter()
                .enumerate()
                .map(|(index, cell)| {
                    let index_u32 = index as u32;
                    next_state(&cell, &living_cells(&neighbors(&matrix, &index_u32)))
                })
                .collect();

            Self {
                width: self.width,
                height: self.height,
                cells: updated
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

    let mut world = World::new(20, 10);
    println!("{}", world);
    sleep(Duration::from_secs(1));

    loop {
        world = world.update();
        println!("{}", world);
        sleep(Duration::from_secs(1));
    }
}
