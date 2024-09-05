use std::thread::sleep;
use std::time::Duration;
use rand::random;

fn main() {
    println!("Welcome to Game Of Life!");

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum CellState {
        Alive(AliveContext),
        Dead(DeadContext),
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum AliveContext {
        Birth,    // 誕生
        Survive,  // 生存
    }

    #[derive(PartialEq, Copy, Clone, Debug)]
    enum DeadContext {
        Overpopulated,  // 過密
        Underpopulated, // 過疎
        CannotBirth     // 誕生できる状態ではない
    }

    impl CellState {
        /// ライフゲームにおける次のセルの状態を決定する関数
        ///
        /// 生存: 生きているセルに隣接する生きたセルが2つか3つならば、次の世代でも生存する。
        /// 過疎: 生きているセルに隣接する生きたセルが1つ以下ならば、過疎により死滅する。
        /// 過密: 生きているセルに隣接する生きたセルが4つ以上ならば、過密により死滅する。
        /// 誕生: 死んでいるセルに隣接する生きたセルがちょうど3つあれば、次の世代が誕生する。
        fn next(&self, living_neighbors: &usize) -> CellState {
            match self {
                CellState::Alive(_) => match living_neighbors {
                    0 | 1 => CellState::Dead(DeadContext::Underpopulated),
                    2 | 3 => CellState::Alive(AliveContext::Survive),
                    4.. => CellState::Dead(DeadContext::Overpopulated),
                },
                CellState::Dead(_) => match living_neighbors {
                    3 => CellState::Alive(AliveContext::Birth),
                    _ => CellState::Dead(DeadContext::CannotBirth),
                }
            }
        }
    }

    fn living_cells(cells: &Vec<CellState>) -> usize {
        cells.iter().fold(0, |acc, cell: &CellState| {
            match cell {
                CellState::Alive(_) => acc + 1,
                CellState::Dead(_) => acc,
            }
        })
    }

    #[derive(Debug)]
    struct Position {
        row: usize,
        col: usize
    }

    struct Matrix<T> {
        width: usize,
        height: usize,
        data: Vec<Vec<T>>,
    }

    impl <T: std::clone::Clone> Matrix<T> {
        fn from_vec(vec: &Vec<T>, size: usize) -> Self {
            assert_eq!(vec.len() % size, 0, "Vec size does not match matrix dimensions");

            let matrix: Vec<_> = vec
                .chunks(size)
                .map(|row| {row.to_vec()})
                .collect();

            Matrix {
                width: size,
                height: matrix.len(),
                data: matrix
            }
        }
        fn neighbors(&self, index: usize) -> Vec<T> {
            let width = self.data[0].len();
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
                    if let Some(data) = self.data
                        .get(row as usize)
                        .and_then(|line: &Vec<T>| line.get(col as usize))
                    {
                        neighbors.push(data.clone());
                    }
                }
            }

            neighbors
        }
    }

    struct World {
        gen: usize,
        width: usize,
        height: usize,
        cells: Vec<CellState>,
    }

    impl World {
        fn new(width: usize, height: usize, density: f64) -> Self {
            let cells = (0..width * height)
                .map(|x| {
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

            let updated = self.cells
                .iter()
                .enumerate()
                .map(|(index, cell)| {
                    cell.next(&living_cells(&matrix.neighbors(index)))
                })
                .collect();

            Self {
                gen: self.gen + 1,
                width: self.width,
                height: self.height,
                cells: updated
            }
        }
    }

    impl std::fmt::Display for World {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Generation: {}\n", self.gen)?;
            for line in self.cells.as_slice().chunks(self.width) {
                for &cell in line {
                    let symbol = match cell {
                        CellState::Alive(context) => {match context {
                            AliveContext::Birth => '〇',
                            AliveContext::Survive => '〇',
                        }}
                        CellState::Dead(context) => {match context {
                            DeadContext::Overpopulated =>'・',
                            DeadContext::Underpopulated => '・',
                            DeadContext::CannotBirth => '・',
                        }}
                    };
                    write!(f, "{}", symbol)?;
                }
                write!(f, "\n")?;
            }
            Ok(())
        }
    }

    let mut world = World::new(40, 25, 0.3);
    println!("{}", world);
    sleep(Duration::from_secs(1));

    loop {
        world = world.update();
        println!("{}", world);
        sleep(Duration::from_secs(1));
    }
}
