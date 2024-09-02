fn main() {
    println!("Hello, world!");

    #[derive(PartialEq, Copy, Clone)]
    enum CellState {
        Alive = 1,
        Dead = 0,
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
