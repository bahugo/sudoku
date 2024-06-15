use std::collections::{HashSet};

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
    fn get_block(&self, row: usize, col: usize) -> Array1<u8> {
        const BLOCK_WIDTH: usize = 3;
        const BLOCK_HEIGHT: usize = 3;
        let start_row = (row / BLOCK_WIDTH) * BLOCK_WIDTH;
        let end_row = start_row + BLOCK_WIDTH;
        let start_col = (col / BLOCK_HEIGHT) * BLOCK_HEIGHT;
        let end_col = start_col + BLOCK_HEIGHT;
        Array1::from_iter(
            self.array
                .slice(s![start_row..end_row, start_col..end_col])
                .iter()
                .cloned(),
        )
    }
    fn get_neighbor_values(&self, row: usize, col: usize) -> HashSet<u8> {
        let mut neighbor_values = HashSet::from_iter(self.get_row(row).into_iter().cloned());
        neighbor_values.extend(self.get_col(col).into_iter().cloned());
        neighbor_values.extend(self.get_block(row, col).iter().cloned());
        neighbor_values.remove(&0);
        neighbor_values
    }
    fn get_candidate_values(&self, row: usize, col: usize) -> HashSet<u8> {
        let candidate_values: HashSet<u8> = HashSet::from([1, 2, 3, 4, 5 ,6 ,7 ,8, 9]);
        let neighbor_values = self.get_neighbor_values(row, col);
        HashSet::difference(&candidate_values, &neighbor_values).cloned().collect()
    }
    fn solve(&self) -> Board {
        Self {
            array: self.array.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

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
        let actual = input.get_block(3, 3);
        assert_eq!(actual, array![0, 6, 0, 8, 0, 3, 0, 2, 0]);
        let actual = input.get_neighbor_values(3, 3);
        assert_eq!(actual, HashSet::from([1, 2, 3, 4, 6, 8]));
        let actual = input.get_neighbor_values(7, 7);
        assert_eq!(actual, HashSet::from([1, 2, 4, 5, 6, 7, 8, 9]));
        let actual = input.get_neighbor_values(7, 8);
        assert_eq!(actual, HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        let actual = input.get_candidate_values(3, 3);
        assert_eq!(actual, HashSet::from([5, 7, 9]));
        let actual = input.get_candidate_values(7, 8);
        assert_eq!(actual, HashSet::from([]));
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
