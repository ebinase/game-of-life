#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

#[allow(dead_code)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T: std::clone::Clone> Matrix<T> {
    pub(crate) fn from_vec(vec: &Vec<T>, size: usize) -> Self {
        assert_eq!(
            vec.len() % size,
            0,
            "Each matrix dimension must be the same size"
        );

        let matrix: Vec<_> = vec.chunks(size).map(|row| row.to_vec()).collect();

        Matrix {
            width: size,
            height: matrix.len(),
            data: matrix,
        }
    }
    pub(crate) fn neighbors(&self, index: usize) -> Vec<T> {
        let width = self.data[0].len();
        let position = Position {
            row: index / width,
            col: index % width,
        };

        let mut neighbors = vec![];
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
                if i == 0 && j == 0 {
                    continue;
                }
                let row = position.row as i32 + i;
                let col = position.col as i32 + j;
                // 行と列が範囲内かを確認して、`Some`なセルだけをpush
                if let Some(data) = self
                    .data
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
