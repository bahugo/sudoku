use ndarray::{s, Array1, Array2, ArrayView1};

fn main() {}

#[derive(Debug, Clone)]
struct Board {
    array: Array2<u8>,
}

impl Board {
    fn get_row(&self, row: usize) -> ArrayView1<u8> {
        self.array.row(row)
    }
    fn get_col(&self, col: usize) -> ArrayView1<u8> {
        self.array.column(col)
    }
    fn get_block(&self, row: isize, col: isize) -> Array1<u8> {
        let start_row = 0;
        let end_row = 3;
        let start_col = 0;
        let end_col = 3;
        Array1::from_iter(self.array.slice(s![start_row..end_row, start_col..end_col]).iter().cloned())
    }
    fn solve(&self) -> Board {
        Self {
            array: self.array.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Board;
    use ndarray::array;

    #[test]
    fn test_access_methods() {
        let input = Board {
            array: array![
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9]
            ],
        };
        let actual = input.get_row(0);
        assert_eq!(actual, array![5, 3, 0, 0, 7, 0, 0, 0, 0]);
        let actual = input.get_row(2);
        assert_eq!(actual, array![0, 9, 8, 0, 0, 0, 0, 6, 0]);
        let actual = input.get_col(0);
        assert_eq!(actual, array![5, 6, 0, 8, 4, 7, 0, 0, 0]);
        let actual = input.get_col(8);
        assert_eq!(actual, array![0, 0, 0, 3, 1, 6, 0, 5, 9]);
        let actual = input.get_block(0, 0);
        assert_eq!(actual, array![5, 3, 0, 6, 0, 0, 0, 9, 8]);
        let actual = input.get_block(3, 0);
        assert_eq!(actual, array![8, 0, 0, 4, 0, 0, 7, 0, 0]);
    }

    #[test]
    fn test_01() {
        let input = Board {
            array: array![
                [5, 3, 0, 0, 7, 0, 0, 0, 0],
                [6, 0, 0, 1, 9, 5, 0, 0, 0],
                [0, 9, 8, 0, 0, 0, 0, 6, 0],
                [8, 0, 0, 0, 6, 0, 0, 0, 3],
                [4, 0, 0, 8, 0, 3, 0, 0, 1],
                [7, 0, 0, 0, 2, 0, 0, 0, 6],
                [0, 6, 0, 0, 0, 0, 2, 8, 0],
                [0, 0, 0, 4, 1, 9, 0, 0, 5],
                [0, 0, 0, 0, 8, 0, 0, 7, 9]
            ],
        };

        let actual = input.solve();

        let expected = Board {
            array: array![
                [5, 3, 4, 6, 7, 8, 9, 1, 2],
                [6, 7, 2, 1, 9, 5, 3, 4, 8],
                [1, 9, 8, 3, 4, 2, 5, 6, 7],
                [8, 5, 9, 7, 6, 1, 4, 2, 3],
                [4, 2, 6, 8, 5, 3, 7, 9, 1],
                [7, 1, 3, 9, 2, 4, 8, 5, 6],
                [9, 6, 1, 5, 3, 7, 2, 8, 4],
                [2, 8, 7, 4, 1, 9, 6, 3, 5],
                [3, 4, 5, 2, 8, 6, 1, 7, 9]
            ],
        };
        assert_eq!(actual.array, expected.array);
    }
}
